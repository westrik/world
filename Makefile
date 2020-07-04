all:
	cd server && cargo build --release
	cd web-client && yarn && yarn build

check:
	cd server; cargo check; cargo clippy
	cd web-client && yarn lint
	cd server && cargo test
	cd web-client && yarn test
	cd infra && terraform validate

fix: fmt

fmt:
	cd server && cargo fmt
	cd web-client && yarn fix
	cd infra && make fmt

prepush:
	make fmt && make check

check_and_test:
	cd server && cargo watch -x check -x test -s 'touch ../.trigger'

run_server:
	cd server && cargo watch --no-gitignore -w ../.trigger -x 'run --bin run_server'

run_client:
	cd web-client && yarn start
