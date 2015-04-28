run:
	cargo run

build:
	cargo build

reload-run:
	@nodemon -e rs,toml -x "cargo run"

reload-build:
	@nodemon -e rs,toml -x "cargo build"

