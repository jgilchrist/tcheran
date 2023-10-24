_default: list

list:
	@just --list

build:
	@cargo build --release

run:
	@cargo run --bin engine

test:
	@cargo test

test-perft:
	@cargo test --release perft -- --include-ignored

time-perft n:
	cargo build --release && time ./target/release/engine perft {{n}}

instruments-time-perft n:
	cd engine && cargo instruments -t "time" --release -- perft {{n}}

instruments-time-search:
	cd engine && cargo instruments -t "time" --release --time-limit 60000

compare-perft n BIN1 BIN2:
	hyperfine '{{BIN1}} perft {{n}}' '{{BIN2}} perft {{n}}'

copy-bin name:
	cargo build --release
	cp target/release/engine bins/{{name}}
