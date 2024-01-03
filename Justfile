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

test-stc new baseline concurrency="2":
	just playoff-sprt {{new}} {{baseline}} {{concurrency}} 8+0.08

test-ltc new baseline concurrency="2":
	just playoff-sprt {{new}} {{baseline}} {{concurrency}} 40+0.4

[private]
playoff-sprt new baseline concurrency tc:
	cutechess-cli \
		-engine name="$(basename {{new}})" cmd="{{new}}" \
		-engine name="$(basename {{baseline}})" cmd="{{baseline}}" \
		-openings file=./etc/openings/UHO_Lichess_4852_v1.epd format=epd \
		-ratinginterval {{concurrency}} \
		-concurrency {{concurrency}} \
		-rounds 100000 -games 2 -repeat \
		-pgnout "./bins/$(basename {{new}})-vs-$(basename {{baseline}})-{{tc}}.pgn" \
		-sprt elo0=0 elo1=5 alpha=0.05 beta=0.05 \
		-each \
			proto=uci \
			tc={{tc}} \
			restart=on

elo-stc new baseline concurrency="2":
	just playoff-elo {{new}} {{baseline}} 2048 {{concurrency}} 8+0.08

elo-ltc new baseline concurrency="2":
	just playoff-elo {{new}} {{baseline}} 512 {{concurrency}} 40+0.4

[private]
playoff-elo new baseline rounds concurrency tc:
	cutechess-cli \
		-engine name="$(basename {{new}})" cmd="{{new}}" \
		-engine name="$(basename {{baseline}})" cmd="{{baseline}}" \
		-openings file=./etc/openings/UHO_Lichess_4852_v1.epd format=epd \
		-ratinginterval {{concurrency}} \
		-concurrency {{concurrency}} \
		-rounds {{rounds}} -games 2 -repeat \
		-pgnout "./bins/$(basename {{new}})-vs-$(basename {{baseline}})-{{tc}}.pgn" \
		-each \
			proto=uci \
			tc={{tc}} \
			restart=on
