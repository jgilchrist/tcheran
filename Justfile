_default: list

list:
	@just --list

build:
	@cargo build --release

run:
	@cargo run --bin engine

time-perft n:
	cargo build --release && time ./target/release/engine perft {{n}}

instruments-time-perft n:
	cd engine && cargo instruments -t "time" --release -- perft {{n}}

compare-perft n BIN1 BIN2:
	hyperfine '{{BIN1}} perft {{n}}' '{{BIN2}} perft {{n}}'

test-perft:
	cargo test -- perft --include-ignored

copy-bin name:
	cargo build --release
	cp target/release/engine bins/{{name}}
