
build:
	cargo build --bin=convert --release

install:
	sudo cp target/release/convert /usr/local/bin/convert
