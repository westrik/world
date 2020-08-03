all:
	cargo build --release
	cd web-client && yarn && yarn build

check:
	cargo check
	cargo clippy
	cargo clippy --tests
	cd web-client && yarn lint && yarn typecheck
	cd infra && terraform validate

test:
	cargo test
	cd web-client && yarn test

fmt:
	cargo fmt
	cd web-client && yarn fix
	cd infra && make fmt

prepush:
	make fmt && make check

run_server_restart_trigger:
	cargo watch -x check -x test -s 'touch .trigger'

run_server:
	cargo watch --no-gitignore -w .trigger -x 'run --bin world_api_server'

run_worker:
	cargo watch --no-gitignore -w .trigger -x 'run --bin world_worker'

run_client:
	cd web-client && yarn start

chk: check

lint: check

prepush_checks: check test

fix: fmt
