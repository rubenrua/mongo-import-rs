all:
	@echo 'MONGODB IMPORT RS'
	@echo '================='
	@echo ''
	@echo 'make release'
	@echo 'make test'
	@echo 'make fmt'
	@echo 'make ci'
build:
	cargo build --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/mongo-import-rs dist/mongo-import-rs
release: build
	strip dist/mongo-import-rs
test:
	cargo test -- --nocapture
fmt:
	cargo fmt
ci:
	cargo fmt -- --check
	touch ./src/*.rs && cargo clippy
	cargo build
	cargo test
run:
	cargo run -- -v
.PHONY: all release test fmt ci
