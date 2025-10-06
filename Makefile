run:
	cargo run -p mathengine-cli

fmt:
	cargo fmt

lint:
	cargo clippy --workspace

lint-fix:
	cargo clippy --workspace --fix

test:
	cargo test --workspace

publish:
	cargo publish -p mathengine-units
	cargo publish -p mathengine-lexer
	cargo publish -p mathengine-parser
	cargo publish -p mathengine-evaluator
	cargo publish -p mathengine