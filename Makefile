all:install

install: build
	cp target/release/gignore /usr/bin

build:
	rustup install stable
	rustup default stable
	cargo build --release


