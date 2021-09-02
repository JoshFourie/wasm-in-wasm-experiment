.PHONY: artifact-registry modules build runtime alpha beta help

all: runtime-serve runtime-watch artifact-registry-serve

runtime:
	cd runtime && make $(rule)

runtime-watch:
	make runtime rule=watch

runtime-serve:
	make runtime rule=serve

alpha:
	cd alpha && make $(rule)

beta:
	cd beta && make $(rule)

modules:
	mkdir -p artifact-registry/src/resources
	cat modules.txt | xargs -I@ make @ rule=build
	cat modules.txt | xargs -I@ cp @/pkg/@_bg.wasm artifact-registry/src/resources/@.wasm

artifact-registry:
	cd artifact-registry && make $(rule)

artifact-registry-serve:
	make artifact-registry rule=serve

help:
	$(info run make runtime to pass a rule to the runtime package.)
