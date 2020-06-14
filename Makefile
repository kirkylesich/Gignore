all:install

install: build
	cd installer && make
	cp target/release/gignore /usr/bin

build:
	rustup install stable
	rustup default stable
	cargo build --release


