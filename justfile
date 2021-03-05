build mode='':
	{{ if mode == "release" { "cargo build --release" } else { "cargo build" } }}

run mode='':
	{{ if mode == "release" { "cargo run --release" } else { "cargo run" } }}

dev:
	cargo watch -x run

default:
	just build

# vim:ft=make
