_default: list

list:
	@just --list

time-perft-5:
	cargo build --release && time ./target/release/engine perft 5

instruments-time-perft-5:
	cd engine && cargo instruments -t "time" --release -- perft 5

compare-perft-5 BIN1 BIN2:
	hyperfine '{{BIN1}} perft 5' '{{BIN2}} perft 5'
