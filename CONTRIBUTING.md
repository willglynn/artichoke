# Contributing to Artichoke

👋 Hi and welcome to [Artichoke](https://github.com/artichoke). Thanks for
taking the time to contribute! 💪💎🙌

Artichoke aspires to be a Ruby 2.6.3-compatible implementation of the Ruby
programming language.
[There is lots to do](https://github.com/artichoke/artichoke/issues).

If Artichoke does not run Ruby source code in the same way that MRI does, it is
a bug and we would appreciate if you
[filed an issue so we can fix it](https://github.com/artichoke/artichoke/issues/new).

If you would like to contribute code 👩‍💻👨‍💻, find an issue that looks interesting
and leave a comment that you're beginning to investigate. If there is no issue,
please file one before beginning to work on a PR.
[Good first issues are labeled `E-easy`](https://github.com/artichoke/artichoke/labels/E-easy).

## Discussion

If you'd like to engage in a discussion outside of GitHub, you can
[join Artichoke's public Discord server](https://discord.gg/QCe2tp2).

## Implementation Philosophy

- Prefer pure Ruby implementations when initially implementing features.
- A feature is not done until it passes [ruby/spec](RUBYSPEC.md).
- Move implementations to Rust for performance, e.g.
  [using Serde to implement the JSON package](https://github.com/artichoke/artichoke/issues/77).
- If there is a Rust crate that does what we need, prefer to use it. Forking is
  OK, too, e.g.
  [artichoke/rust-onig](https://github.com/artichoke/rust-onig/tree/artichoke-vendor).

## Setup

Artichoke includes Rust, Ruby, C, and Text sources. Developing on Artichoke
requires configuring several dependencies.

### Rust Toolchain

Artichoke depends on Rust and several compiler plugins for linting and
formatting. The specific version of Rust Artichoke requires is specified in the
[toolchain file](rust-toolchain).

Toolchain requirements are documented in [`BUILD.md`](BUILD.md#rust-toolchain).

### C Toolchain

Some artichoke dependencies, like the mruby [`sys`](artichoke-backend/src/sys)
and [`onig`](https://crates.io/crates/onig), build C static libraries and
require a C compiler.

Toolchain requirements are documented in [`BUILD.md`](BUILD.md#c-toolchain).

### Ruby

Artichoke requires a recent Ruby 2.x and [bundler](https://bundler.io/) 2.x. The
[`.ruby-version`](.ruby-version) file in this repository specifies Ruby 2.6.3.

Toolchain requirements are documented in [`BUILD.md`](BUILD.md#ruby-toolchain).

Artichoke uses [`rake`](Rakefile) as a task runner. You can see the available
tasks by running:

```console
$ bundle exec rake --tasks
rake doc               # Generate Rust API documentation
rake doc:open          # Generate Rust API documentation and open it in a web browser
rake lint:all          # Lint and format
rake lint:deps         # Install linting dependencies
rake lint:eslint       # Run eslint
rake lint:format       # Format sources
rake lint:links        # Check markdown links
rake lint:restriction  # Lint with restriction pass (unenforced lints)
rake lint:rubocop      # Run rubocop
rake spec              # Run enforced ruby/spec suite
```

To lint Ruby sources, Artichoke uses
[RuboCop](https://github.com/rubocop-hq/rubocop). RuboCop runs as part of the
`lint:all` task. To run RuboCop by itself, invoke the `lint:rubocop` task.

### Node.js

Node.js is an optional dependency that is used for formatting text sources with
[prettier](https://prettier.io/).

Node.js is only required for formatting if modifying the following filetypes:

- `c`
- `h`
- `html`
- `json`
- `md`
- `toml`
- `yaml`
- `yml`

You will need to install
[Node.js](https://nodejs.org/en/download/package-manager/).

On macOS, you can install Node.js with
[Homebrew](https://docs.brew.sh/Installation):

```sh
brew install node
```

### Node.js Packages

Once you have Node.js installed, you can install the packages specified in
[`package.json`](package.json).

Node.js packages are automatically installed by linting tasks defined in the
[`Rakefile`](Rakefile).

## Code Quality

### Linting

Once you [configure a development environment](#setup), run the following to
lint sources:

```sh
rake lint:all
```

Merges will be blocked by CI if there are lint errors.

### Testing

A PR must have tests for it to be merged. The
[Rust book chapter on testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
is a good place to start. If you'd like to see some examples in Artichoke, take
a look at the `Value` tests in
[`artichoke-backend/src/value/mod.rs`](artichoke-backend/src/value.rs).

To run tests:

```sh
cargo test --workspace
```

If you are only working on one package, it can speed up iteration time to only
build and run tests for that package:

```sh
cargo test -p artichoke-backend
```

`cargo test` accepts a filter argument that will limit test execution to tests
that substring match. For example, to run all of the Ruby/Rust interop tests:

```sh
cargo test -p artichoke-backend convert
```

Tests are run for every PR. All builds must pass before merging a PR.

## Updating Dependencies

### Rust Toolchain

Upgrades to the Rust toolchain should happen in a dedicated PR that addresses
any changes to ructc warnings and clippy lints. See
[GH-482](https://github.com/artichoke/artichoke/pull/482) for an example.

### Rust Crates

Version specifiers in `Cargo.toml` are NPM caret-style by default. A version
specifier of `4.1.2` means `4.1.2 <= version < 5.0.0`.

To see what crates are outdated, you can use
[cargo-outdated](https://github.com/kbknapp/cargo-outdated).

If you need to pull in an updated version of a crate for a bugfix or a new
feature, update the version number in `Cargo.toml`. See
[GH-548](https://github.com/artichoke/artichoke/pull/548) for an example.

To update Rust crate dependencies run the following command and check in the
updated `Cargo.lock` file:

```sh
cargo update
```

### Node.js Packages

To see what packages are outdated, you can run `npm outdated`.

To update Node.js package dependencies run the following command and check in
the updated `package-lock.json` file:

```sh
npm update
```

If after running `npm update` there are still outdated packages reported by
`npm outdated`, there has likely been a major release of a dependency. If you
would like to update the dependency and deal with any breakage, please do;
otherwise, please
[file an issue](https://github.com/artichoke/artichoke/issues/new).
