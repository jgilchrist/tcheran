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

playoff name1 name2 concurrency openingsfile:
	cutechess-cli \
		-engine name="{{name1}}" cmd="./bins/{{name1}}" \
		-engine name="{{name2}}" cmd="./bins/{{name2}}" \
		-openings file={{openingsfile}} \
		-ratinginterval 5 \
		-concurrency {{concurrency}} \
		-rounds 1000 -games 2 -repeat \
		-pgnout ./bins/{{name1}}-vs-{{name2}}.pgn \
		-each \
			proto=uci \
			tc=8+0.2 \
			restart=on
