all:
	test production

test:
	cargo test -q

production:
	cargo build --release
	strip target/release/emoj
	mv target/release/remojocky /usr/local/bin/
	chmod ugo+x /usr/local/bin/

dev:
	cargo build
	mv target/debug/emoj /usr/local/bin/
	chmod ugo+x /usr/local/bin/
