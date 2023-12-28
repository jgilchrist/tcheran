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
	@cargo test --release tests::perft_tests -- --include-ignored

test-perft-tt:
	@cargo test --release tests::zobrist_perft_tests -- --include-ignored

############################### Profiling #####################################

instruments:
	cd engine && cargo instruments -t "time" --release --time-limit 60000

instruments-debug:
	cd engine && cargo instruments -t "time" --time-limit 60000

################################# Misc #######################################

copy-bin name:
	cargo build --release
	cp target/release/engine bins/{{name}}

playoff-stc new baseline concurrency="2":
	just playoff {{new}} {{baseline}} {{concurrency}} 8+0.08

playoff-ltc new baseline concurrency="2":
	just playoff {{new}} {{baseline}} {{concurrency}} 60+0.6

[private]
playoff new baseline concurrency tc:
	cutechess-cli \
		-engine name="$(basename {{new}})" cmd="{{new}}" \
		-engine name="$(basename {{baseline}})" cmd="{{baseline}}" \
		-openings file=./etc/openings/UHO_Lichess_4852_v1.epd format=epd \
		-ratinginterval {{concurrency}} \
		-concurrency {{concurrency}} \
		-rounds 100000 -games 2 -repeat \
		-pgnout "./bins/$(basename {{new}})-vs-$(basename {{baseline}})-ltc.pgn" \
		-sprt elo0=0 elo1=5 alpha=0.05 beta=0.05 \
		-each \
			proto=uci \
			tc={{tc}} \
			restart=on
