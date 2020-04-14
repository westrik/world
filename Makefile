all:
	cd server && cargo build --release
	cd client-web && yarn && yarn build

check:
	cd server; cargo check; cargo clippy
	cd client-web && yarn lint
	cd server && cargo test
	cd client-web && yarn test
	cd infra && terraform validate

fmt:
	cd server && cargo fmt
	cd client-web && yarn fix
	cd infra && make fmt

prepush:
	make fmt && make check

check_and_test:
	cd server && cargo watch -x check -x test -s 'touch ../.trigger'

run_server:
	cd server && cargo watch --no-gitignore -w ../.trigger -x 'run --bin run_server'

run_client:
	cd client-web && yarn start
