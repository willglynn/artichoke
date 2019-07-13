#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings, intra_doc_link_resolution_failure)]

use std::cell::RefCell;

use cactusref::CactusRef;

mod leak;

const ITERATIONS: usize = 50;
const LEAK_TOLERANCE: i64 = 1024 * 1024 * 25;

struct RString {
    inner: String,
    link: Option<CactusRef<RefCell<Self>>>,
}

#[test]
fn cactusref_chain_no_leak() {
    let s = "a".repeat(1024 * 1024);

    // 500MB of `String`s will be allocated by the leak detector
    leak::Detector::new("CactusRef chain no leak", ITERATIONS, LEAK_TOLERANCE).check_leaks(|_| {
        // each iteration creates 10MB of `String`s
        let first = CactusRef::new(RefCell::new(RString {
            inner: s.clone(),
            link: None,
        }));
        let mut last = CactusRef::clone(&first);
        for _ in 1..10 {
            let obj = CactusRef::new(RefCell::new(RString {
                inner: s.clone(),
                link: Some(CactusRef::clone(&last)),
            }));
            last = obj;
        }
        assert!(first.borrow().link.is_none());
        assert_eq!(first.borrow().inner, s);
        assert!(last.borrow().link.is_some());
        assert_eq!(last.borrow().inner, s);
        drop(first);
        drop(last);
    });
}
