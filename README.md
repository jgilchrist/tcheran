# <div align="center">Tcheran</div>

A UCI compatible engine developed by [@jgilchrist](https://github.com/jgilchrist), written in Rust.

## Ratings

Thank you to everybody who has tested the engine.

| Version | [CCRL Blitz][ccrl-blitz] | [MCERL][mcerl] | [CEDR][cedr] |
| ------- | -------------------------|----------------|--------------|
| v2.1    | 2272                     | 2540           | -            |
| v2.0    | -                        | 2425           | -            |
| v1.1    | -                        | 2228           | 2261         |
| v1.0    | 1879                     | -              | -            |

[ccrl-blitz]: https://computerchess.org.uk/ccrl/404/
[mcerl]: https://www.chessengeria.eu/mcerl
[cedr]: https://chessengines.blogspot.com/p/rating-jcer.html

It can also be found on Lichess as [`jpg-bot`](https://lichess.org/@/jpg-bot). Its current ratings are:

[![lichess-rapid](https://lichess-shield.vercel.app/api?username=jpg-bot&format=bullet)](https://lichess.org/@/jpg-bot/perf/bullet)
[![lichess-rapid](https://lichess-shield.vercel.app/api?username=jpg-bot&format=blitz)](https://lichess.org/@/jpg-bot/perf/blitz)
[![lichess-rapid](https://lichess-shield.vercel.app/api?username=jpg-bot&format=rapid)](https://lichess.org/@/jpg-bot/perf/rapid)

## Features

* Board
    * Bitboard board representation
    * Redundant mailbox representation for square lookups
    * Zobrist hashing

* Move generation
    * Fully legal move generation
    * Fancy Magic bitboards

* Search
    * Iterative deepening
    * Negamax
    * Quiescence search
    * Principal variation search (PVS)
    * Check extensions
    * Transposition table
    * Null move pruning
    * Reverse futility pruning

* Move ordering
    * Previous best move
    * Most Valuable Victim - Least Valuable Aggressor (MVV-LVA)
    * Killer move heuristic
    * History heuristic
    * Incremental sorting

* Evaluation
    * Material difference
    * Midgame and endgame piece square tables
    * Tapered midgame vs. endgame evaluation
    * Incremental updates
