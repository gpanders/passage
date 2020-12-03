prefix := /usr/local
bindir := $(prefix)/bin
sharedir := $(prefix)/share

.PHONY: all
all: target/release/passage target/doc/passage.1

target/release/passage: Cargo.toml $(wildcard src/*.rs src/*/*.rs)
	cargo build --release

target/doc/passage.1: doc/passage.1.txt
	install -d $(@D)
	asciidoctor --backend manpage --doctype manpage -o $@ $<

.PHONY: test
test:
	cargo test

.PHONY: install
install: target/release/passage target/doc/passage.1
	install -d $(bindir) $(sharedir)/man/man1
	install -m 0755 target/release/passage $(bindir)/
	install -m 0644 target/doc/passage.1 $(sharedir)/man/man1/
