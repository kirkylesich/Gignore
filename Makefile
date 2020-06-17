all:install

install: build
	cargo build --release
	cp target/release/gignore /usr/bin
	rm -rf ../Gignore

build:
	rustup install stable
	rustup default stable
