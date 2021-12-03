setup:
    cargo install git-cliff

build:
    cargo build --all

test:
    cargo test --all

release:
    cargo build --verbose --release --all

@bench:
	cargo bench

@lint:
	rustup component add clippy
	rustup component add rustfmt
	cargo clippy -- -D warnings
	cargo clippy --tests
	cargo fmt -- --check

@fix:
    cargo fmt --all

# cargo install cargo-bloat
@analysis:
    cargo bloat --release -n 50

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;

changelog:
    git cliff --output CHANGELOG.md
