setup:
    cargo install git-cliff
    rustup component add llvm-tools-preview --toolchain nightly
    cargo install cargo-llvm-cov

install:
    cargo install --path .

build:
    cargo build --all

test:
    cargo test --all

release:
    cargo build --verbose --release --all

coverage:
    cargo llvm-cov --all-features --workspace --html

pages:
    cargo run -- pagedump
    cargo build -p quake_wasm
    cd quake_webapp && npm run local

    # copy file after has dist
    cp -a pagedump/* quake_webapp/dist/
    cd quake_wasm && wasm-pack build --scope quakeworks --target web
    mkdir -p quake_webapp/dist/js/dashboard/
    cp quake_wasm/pkg/quake_wasm_bg.wasm quake_webapp/dist/js/dashboard/

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
