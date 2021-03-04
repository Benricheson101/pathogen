build mode='':
	{{ if mode == "release" { "cargo build --release" } else { "cargo build" } }}

dev:
	cargo watch -x run

default:
	just build

# vim:ft=make
