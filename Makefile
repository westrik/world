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

watch_check:
	cd server && cargo watch -x check -s 'touch ../.trigger'

watch_run_server:
	cd server && cargo watch --no-gitignore -w ../.trigger -x 'run --bin run_server'

watch_run_tests:
	cd server && cargo watch --no-gitignore -w ../.trigger -x test

run_client:
	cd client-web && yarn start
