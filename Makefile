
install: build
	cp target/release/gignore /usr/bin

build:
	cargo build --release
	rustup install stable
	rustup default stable


