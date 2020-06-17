all:install

install: build
	cargo build --release
	cp target/release/gignore /usr/bin

build:
	rustup install stable
	rustup default stable
