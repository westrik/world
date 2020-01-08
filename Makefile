all:
	cd server && cargo build --release
	cd web-client && yarn && yarn build

check:
	cd server && cargo check && cargo clippy
	cd web-client && yarn lint
	cd server && cargo test
	cd web-client && yarn test
	cd infra && terraform validate

fmt:
	cd server && cargo fmt
	cd web-client && yarn fix
	cd infra && make fmt

prepush:
	make format && make check
