_default: list

list:
	@just --list

################################### Basics ####################################

build:
	@cargo build --release

run:
	@cargo run --release

################################## Tests ######################################

test:
	@cargo test

test-perft:
	@cargo test --release --package chess perft -- --include-ignored

test-perft-tt:
	@cargo test --release --package engine perft_tt -- --include-ignored

############################### Profiling #####################################

instruments-time-perft n:
	cd engine && cargo instruments -t "time" --release -- perft {{n}}

instruments-time-search:
	cd engine && cargo instruments -t "time" --release --time-limit 60000

################################# Misc #######################################

copy-bin name:
	cargo build --release
	cp target/release/engine bins/{{name}}
