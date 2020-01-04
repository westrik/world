all:
	cd server && cargo build --release
	cd web-client && yarn && yarn build

check:
	cd server && cargo check && cargo clippy
	cd web-client && yarn lint
	cd server && cargo test
	cd web-client && yarn test

format:
	cd server && cargo fmt
	cd web-client && yarn fix

prepush:
	make format && make check
