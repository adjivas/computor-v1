CARGO = cargo

CARGO_OPTS =

all:
	$(MAKE) build
	$(MAKE) doc

build:
	$(CARGO) $(CARGO_OPTS) build

clean:
	$(CARGO) $(CARGO_OPTS) clean

check:
	$(MAKE) build
	$(MAKE) test

test:
	$(CARGO) $(CARGO_OPTS) test

bench:
	$(CARGO) $(CARGO_OPTS) bench

doc:
	$(CARGO) $(CARGO_OPTS) doc

asmjs:
	$(CARGO) build --target asmjs-unknown-emscripten --release

wasm:
	$(CARGO) build --target wasm32-unknown-emscripten --release

.PHONY: all build clean check test bench doc
