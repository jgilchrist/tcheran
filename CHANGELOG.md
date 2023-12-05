# Changelog

## [Unreleased]

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
