.PHONY: clean help

build:
	wasm-pack build --target web

clean:
	rm -rf $(targets)

watch:
	cargo watch -s 'make build'

help:
	$(info run make build to build a wasm binary for the web target.)
	$(info run make clean to remove all app and build artifacts.)
