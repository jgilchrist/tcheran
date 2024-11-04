# <div align="center">Tcheran</div>

A UCI compatible engine developed by [@jgilchrist](https://github.com/jgilchrist), written in Rust.

## Ratings

Thank you to everybody who has tested the engine.

| Version | [CCRL 40/15][ccrl-ltc] | [CCRL Blitz][ccrl-blitz] | [MCERL][mcerl] |
| ------- | ---------------------- | -------------------------|----------------|
| v4.0    | 2517                   | 2564                     | -              |
| v3.0    | 2423                   | 2485                     | -              |
| v2.5    | 2370                   | -                        | -              |
| v2.4    | -                      | 2306                     | -              |
| v2.3    | -                      | 2330                     | -              |
| v2.2    | -                      | 2268                     | -              |
| v2.1    | 2276                   | 2231                     | 2542           |
| v2.0    | -                      | -                        | 2434           |
| v1.1    | -                      | -                        | 2237           |
| v1.0    | -                      | 1871                     | -              |

[ccrl-ltc]: https://computerchess.org.uk/ccrl/4040/
[ccrl-blitz]: https://computerchess.org.uk/ccrl/404/
[mcerl]: https://www.chessengeria.eu/mcerl

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
