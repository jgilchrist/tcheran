# <div align="center">Tcheran</div>

A UCI compatible engine developed by [@jgilchrist](https://github.com/jgilchrist), written in Rust.

## Ratings

Thank you to everybody who has tested the engine.

| Version | [CCRL 40/15][ccrl-ltc] | [CCRL Blitz][ccrl-blitz] | [MCERL][mcerl] |
| ------- | ---------------------- | -------------------------|----------------|
| v4.0    | 2517                   | 2564                     | 2726           |
| v3.0    | 2423                   | 2485                     | 2659           |
| v2.5    | 2370                   | -                        | 2621           |
| v2.4    | -                      | 2306                     | 2583           |
| v2.3    | -                      | 2330                     | 2557           |
| v2.2    | -                      | 2268                     | 2550           |
| v2.1    | 2276                   | 2231                     | 2534           |
| v2.0    | -                      | -                        | 2430           |
| v1.1    | -                      | -                        | 2231           |
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
