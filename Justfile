_default: list

list:
	@just --list

time-perft-5:
	cargo build --release && time ./target/release/engine perft 5

instruments-time-perft-5:
	cd engine && cargo instruments -t "time" --release -- perft 5

compare-perft n BIN1 BIN2:
	hyperfine '{{BIN1}} perft {{n}}' '{{BIN2}} perft {{n}}'
