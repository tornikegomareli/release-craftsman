all: build move

build:
	cargo build --release

move:
	sudo mv ./target/release/releasecraftsman /usr/local/bin
