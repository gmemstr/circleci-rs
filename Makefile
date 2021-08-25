build: src/lib.rs src/api_v2.rs
	cargo build --lib

header: src/lib.rs src/api_v2.rs
	cbindgen --config cbindgen.toml --output target/debug/circleci.h
