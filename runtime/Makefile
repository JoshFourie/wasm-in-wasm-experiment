.PHONY: clean help

watch:
	cargo watch -s 'make build'

build:
	wasm-pack build --target bundler

app-resources:
	cat resources/remove.txt | xargs rm -rf
	cp resources/app-package.json app/package.json
	cp resources/index.js app/index.js

app-init:
	npm init wasm-app app

app: app-init app-resources

serve:
	cd app &&\
		npm install &&\
		npm run start

clean:
	rm -rf $(targets)

help:
	$(info run make build to build a wasm binary for the bundler target.)
	$(info run make app to make a wasm-app template.)
	$(info run make clean to remove all app and build artifacts.)
	$(info run make serve to host the app on localhost via npm)
