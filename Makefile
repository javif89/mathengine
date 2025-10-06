run:
	cargo run -p mathengine-cli

fmt:
	cargo fmt

lint:
	cargo clippy --workspace

lint-fix:
	cargo clippy --workspace --fix