all:
	cd server && cargo build --release
	cd web-client && yarn build

check:
	cd server && cargo check && cargo clippy
	cd web-client && yarn lint
	cd server && cargo test
	cd web-client && yarn test

format:
	cd server && cargo fmt
	cd web-client && yarn cute

preflight:
	make && make check && make format
