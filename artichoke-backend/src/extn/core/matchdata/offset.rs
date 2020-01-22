//! [`MatchData#offset`](https://ruby-doc.org/core-2.6.3/MatchData.html#method-i-offset)

use std::convert::TryFrom;
use std::str;

use crate::extn::core::matchdata::MatchData;
use crate::extn::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Args<'a> {
    Index(Int),
    Name(&'a str),
}

impl<'a> Args<'a> {
    pub fn extract(interp: &mut Artichoke, elem: Value) -> Result<Self, Exception> {
        let name = elem.pretty_name(interp);
        if let Ok(index) = elem.try_into::<Int>(interp) {
            Ok(Self::Index(index))
        } else if let Ok(name) = elem.funcall::<&str>(interp, "to_str", &[], None) {
            Ok(Self::Name(name))
        } else if let Ok(index) = elem.funcall::<Int>(interp, "to_int", &[], None) {
            Ok(Self::Index(index))
        } else {
            Err(Exception::from(TypeError::new(
                interp,
                format!("no implicit conversion of {} into Integer", name),
            )))
        }
    }
}

pub fn method(interp: &mut Artichoke, args: Args, value: &Value) -> Result<Value, Exception> {
    let data = unsafe { MatchData::try_from_ruby(interp, value) }.map_err(|_| {
        Fatal::new(
            interp,
            "Unable to extract Rust MatchData from Ruby MatchData receiver",
        )
    })?;
    let borrow = data.borrow();
    let haystack = &borrow.string[borrow.region.start..borrow.region.end];
    let index = match args {
        Args::Index(index) => {
            let captures_len = borrow.regexp.inner().captures_len(interp, Some(haystack))?;
            if index < 0 {
                // Positive Int must be usize
                let idx = usize::try_from(-index).map_err(|_| {
                    Fatal::new(interp, "Expected positive position to convert to usize")
                })?;
                if let Some(idx) = captures_len.checked_sub(idx) {
                    idx
                } else {
                    return Ok(interp.convert([None::<Value>, None::<Value>].as_ref()));
                }
            } else {
                let idx = usize::try_from(index).map_err(|_| {
                    Fatal::new(interp, "Expected positive position to convert to usize")
                })?;
                if idx > captures_len {
                    return Ok(interp.convert([None::<Value>, None::<Value>].as_ref()));
                }
                idx
            }
        }
        Args::Name(name) => {
            let indexes = borrow
                .regexp
                .inner()
                .capture_indexes_for_name(interp, name.as_bytes())?;
            let indexes = if let Some(indexes) = indexes {
                indexes
            } else {
                return Ok(interp.convert([None::<Value>, None::<Value>].as_ref()));
            };
            if let Some(Ok(index)) = indexes.last().copied().map(usize::try_from) {
                index
            } else {
                return Ok(interp.convert([None::<Value>, None::<Value>].as_ref()));
            }
        }
    };
    if let Some((begin, end)) = borrow.regexp.inner().pos(interp, haystack, index)? {
        let begin = if let Ok(haystack) = str::from_utf8(&haystack[0..begin]) {
            haystack.chars().count()
        } else {
            haystack.len()
        };
        let begin = begin + borrow.region.start;
        let begin = Int::try_from(begin)
            .map_err(|_| Fatal::new(interp, "MatchData begin pos does not fit in Integer"))?;

        let end = if let Ok(haystack) = str::from_utf8(&haystack[0..end]) {
            haystack.chars().count()
        } else {
            haystack.len()
        };
        let end = end + borrow.region.start;
        let end = Int::try_from(end)
            .map_err(|_| Fatal::new(interp, "MatchData end pos does not fit in Integer"))?;

        Ok(interp.convert([begin, end].as_ref()))
    } else {
        Ok(interp.convert([None::<Value>, None::<Value>].as_ref()))
    }
}
