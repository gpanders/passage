prefix = /usr/local
bindir = $(prefix)/bin
sharedir = $(prefix)/share

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
	install -d $(DESTDIR)$(bindir) $(DESTDIR)$(sharedir)/man/man1
	install -m 0755 target/release/passage $(DESTDIR)$(bindir)/
	install -m 0644 target/doc/passage.1 $(DESTDIR)$(sharedir)/man/man1/

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(bindir)/passage $(DESTDIR)$(sharedir)/man/man1/passage.1
