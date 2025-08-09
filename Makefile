EXE = Tcheran

ifeq ($(OS),Windows_NT)
	NAME := $(EXE).exe
else
	NAME := $(EXE)
endif

openbench:
	cargo rustc --bin engine --release --no-default-features --features release -- -C target-cpu=native --emit link=$(NAME)
