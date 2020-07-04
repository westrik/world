all:
	cargo build --release
	cd web-client && yarn && yarn build

check:
	cargo check
	cargo clippy
	cargo test
	cd web-client && yarn lint
	cd web-client && yarn test
	cd infra && terraform validate

fix: fmt

fmt:
	cargo fmt
	cd web-client && yarn fix
	cd infra && make fmt

prepush:
	make fmt && make check

check_and_test:
	cargo watch -x check -x test -s 'touch ../.trigger'

run_server:
	cargo watch --no-gitignore -w ../.trigger -x 'run --bin run_server'

run_client:
	cd web-client && yarn start
