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
	@cargo test --release

############################### Profiling #####################################

instruments +CMD:
	cargo instruments -t "time" --release -- "{{CMD}}"

instruments-debug +CMD:
	cargo instruments -t "time" -- "{{CMD}}"

################################# Misc #######################################

sprt-progression ll ld dd wd ww:
	@just sprt 0.0 5.0 {{ll}} {{ld}} {{dd}} {{wd}} {{ww}}

sprt-regression ll ld dd wd ww:
	@just sprt -5.0 0.0 {{ll}} {{ld}} {{dd}} {{wd}} {{ww}}

sprt elo0 elo1 ll ld dd wd ww:
	@./etc/sprt.py --elo0 {{elo0}} --elo1 {{elo1}} --results {{ll}} {{ld}} {{dd}} {{wd}} {{ww}}

copy-bin name:
	cargo build --release
	cp target/release/engine bins/{{name}}

test-stc new baseline concurrency="2":
	just playoff-sprt {{new}} {{baseline}} {{concurrency}} 8+0.08

test-ltc new baseline concurrency="2":
	just playoff-sprt {{new}} {{baseline}} {{concurrency}} 40+0.4

test-stc-with-adjudication new baseline concurrency="2":
	just playoff-sprt-with-adjudication {{new}} {{baseline}} {{concurrency}} 8+0.08

test-ltc-with-adjudication new baseline concurrency="2":
	just playoff-sprt-with-adjudication {{new}} {{baseline}} {{concurrency}} 40+0.4

[private]
playoff-sprt new baseline concurrency tc:
	cutechess-cli \
		-engine name="$(basename {{new}})" cmd="{{new}}" \
		-engine name="$(basename {{baseline}})" cmd="{{baseline}}" \
		-openings file=./etc/openings/UHO_Lichess_4852_v1.epd format=epd order=random \
		-ratinginterval {{concurrency}} \
		-concurrency {{concurrency}} \
		-rounds 100000 -games 2 -repeat \
		-pgnout "./bins/$(basename {{new}})-vs-$(basename {{baseline}})-{{tc}}.pgn" \
		-sprt elo0=0 elo1=5 alpha=0.05 beta=0.05 \
		-each \
			proto=uci \
			tc={{tc}} \
			restart=on

[private]
playoff-sprt-with-adjudication new baseline concurrency tc:
	cutechess-cli \
		-engine name="$(basename {{new}})" cmd="{{new}}" \
		-engine name="$(basename {{baseline}})" cmd="{{baseline}}" \
		-openings file=./etc/openings/UHO_Lichess_4852_v1.epd format=epd order=random \
		-ratinginterval {{concurrency}} \
		-concurrency {{concurrency}} \
		-rounds 100000 -games 2 -repeat \
		-draw movenumber=40 movecount=8 score=10 \
		-resign movecount=3 score=400 twosided=true \
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
		-openings file=./etc/openings/UHO_Lichess_4852_v1.epd format=epd order=random \
		-ratinginterval {{concurrency}} \
		-concurrency {{concurrency}} \
		-rounds {{rounds}} -games 2 -repeat \
		-pgnout "./bins/$(basename {{new}})-vs-$(basename {{baseline}})-{{tc}}.pgn" \
		-each \
			proto=uci \
			tc={{tc}} \
			restart=on
