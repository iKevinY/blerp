PREFIX ?= /usr/local

all: build

build:
	@which rustc > /dev/null || { echo "blerp requires Rust to compile (see http://rust-lang.org/)."; exit 1; }
	@cargo build --release

clean:
	@rm -rf target

install: build
	@install -dm755 $(PREFIX)/bin/ $(PREFIX)/share/man/man1/
	@install -sm755 target/release/blerp $(PREFIX)/bin/
	@install -m644 man/blerp.1 $(PREFIX)/share/man/man1/

uninstall:
	@rm -f $(PREFIX)/bin/blerp
	@rm -f $(PREFIX)/share/man/man1/blerp.1

.PHONY: all build clean install uninstall
