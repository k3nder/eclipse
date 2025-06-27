VERSION=0.0.1-alpha

server:
	RUST_LOG=debug cargo run --package eclipse-server
client:
	RUST_LOG=debug cargo run --package eclipse-client

blib:
	cargo build --package eclipse-lib
bserver:
	cargo build --package eclipse-server
bclient:
	cargo build --package eclipse-client

rbuild:
	cargo build --release
build:
	cargo build 

