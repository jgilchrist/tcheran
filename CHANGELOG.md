# Changelog

## [Unreleased]

### Misc

* Allow making multiple moves with `d move [moves]`

## [2.2]

* Restore zobrist hash, incremental evaluation fields and castle rights from history when undoing move (~17 Elo)
* Fix throwing away old en passant target during null moves (~19 Elo)
* Use a dedicated `MoveList` struct instead of `Vec<Move>`
* Store castle rights as an array indexed by player
* Correct stored mate values in TT
* Make a panic move if there wasn't enough time to find a PV move during search

### Misc

* Split eval tapering into its own module
* Bundle midgame and endgame evals into a `PhasedEval` struct
* Fix taking up more memory than needed when the transposition table is resized repeatedly
* Remove the default 50ms move overhead and add a UCI option to configure it
* Always log crashes to a .crash.log file
* Check for time termination in the root
* Don't try reporting PV beyond actual depth searched

## [2.1]

* Use transposition table entries from the same depth (~101 Elo)
* Always extend when in check (~29 Elo)

### Misc

* Avoid double-counting 'root' quiescence nodes

## [2.0]

* Add null move pruning
* Add killer move ordering
* Sort moves via individual move scoring
* Sort moves incrementally
* Add reverse futility pruning
* Add history heuristic move ordering

### Misc

* Remove the ability to specify alternate strategies

## [1.1]

* Disable logging by default
* Use Rust 1.75
* Switch movegen to use orthogonal/diagonal pin approach from [this article](https://www.codeproject.com/Articles/5313417/Worlds-Fastest-Bitboard-Chess-Movegenerator)
* Use `.get_unchecked()` for all static array accesses (-1.18% perft(8) time)
* Store `Square` as a `u8` instead of a `Bitboard` internally
* Use an array for `PlayerPieces` (-6.37% search(9) time)
* Generate attackers for single squares instead of all attacks in movegen (+8.0 Elo)
* Optimise castle move generation
* Don't generate non-capture underpromotions in quiescence search
* Remove the `Ctx` struct from movegen (-3.13% perft(7) time)
* Reorganise everything into a single crate
* Re-enable incremental compilation for an unexplained performance boost in `sort_unstable_by`
* Consider bishops more valuable than knights for MVV-LVA

### Misc

* Add SAN parsing and formatting
* Add the 'Win At Chess' test suite
* Add Justfile commands for STC and LTC tests
* Use `u64` for node counts to prevent overflows with large perft results
* Remove support for `go searchmoves` and `go mate`
* Collapse castle detection for kingside/queenside into a single code path
* Various refactoring and simplification around `Bitboard` and `Square` abstractions
* Remove `EngineGame`
* Add a `wait` extension to allow piping `go` commands to the engine for benchmarking
* Make the halfmove clock and fullmove number optional in FEN parsing
* Add a way to easily jump to useful debugging positions (e.g. `d position kiwipete`)
* Add the ability to pass UCI commands to run as command line arguments

## [1.0]

Initial release with the following major features:

* Board
    * Bitboard board representation
    * Redundant mailbox representation for square lookups
    * Zobrist hashing

* Move generation
    * Fully legal move generation (~200 million NPS)
    * Fancy Magic bitboards

* Search
    * Negamax
    * Iterative deepening
    * Quiescence search
    * Principal Variation Search (PVS)
    * Check extensions
    * Transposition table

* Move ordering
    * Previous best move
    * Most Valuable Victim - Least Valuable Aggressor (MVV-LVA)

* Evaluation
    * Material difference
    * Midgame and endgame piece square tables
    * Tapered midgame vs. endgame evaluation
    * Incremental updates

[unreleased]: https://github.com/jgilchrist/chess-engine/compare/v2.2...HEAD
[2.2]: https://github.com/jgilchrist/chess-engine/compare/v2.1..v2.2
[2.1]: https://github.com/jgilchrist/chess-engine/compare/v2.0..v2.1
[2.0]: https://github.com/jgilchrist/chess-engine/compare/v1.1..v2.0
[1.1]: https://github.com/jgilchrist/chess-engine/compare/v1.0..v1.1
[1.0]: https://github.com/jgilchrist/chess-engine/releases/tag/v1.0
