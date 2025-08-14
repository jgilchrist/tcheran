# Network

This file contains information about the engine's networks and the data used to train them.

## Networks

Parameters in the following table are noted only where they differ from Bullet's defaults, which [can be found here](https://github.com/jgilchrist/bullet/blob/e1d5ced0916dbbc0c1e603e67542cbe99d2e05b7/src/main.rs).

| Network | Dataset    | Architecture       | Parameters         | SPRT            | Notes                |
| ------- | ---------- | ------------------ | ------------------ | --------------- | -------------------- |
| 0       | Dataset #0 | (768->16)x2->1     | WDL 0.1, LR 0.01   | 38.11 +- 13.69  | Hello world!         |

## Training datasets

| Dataset | # Fens      | Datagen runs    |
| ------- | ----------- | --------------- |
|       0 |   3,847,979 | 20251013-114424 |

## Datagen runs

All data used for training is self-play data generated using the datagen code from this repository.

| Run             | # Fens     | Info                              |
| --------------- | ---------- | --------------------------------- |
| 20251013-114424 |  3,847,979 | Depth 8, no persistent TT, no TBs |
