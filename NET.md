# Network

This file contains information about the engine's networks and the data used to train them.

## Networks

Parameters in the following table are noted only where they differ from Bullet's defaults, which [can be found here](https://github.com/jgilchrist/bullet/blob/e1d5ced0916dbbc0c1e603e67542cbe99d2e05b7/src/main.rs).

| Network | Dataset    | Architecture       | Parameters         | SPRT            | Notes                |
| ------- | ---------- | ------------------ | ------------------ | --------------- | -------------------- |
| 0       | Dataset #0 | (768->16)x2->1     | WDL 0.1, LR 0.01   | 38.11 +- 13.69  | Hello world!         |
| 1       | Dataset #1 | (768->256)x2->1    | WDL 0.1, LR 0.01   | 223.69 +- 33.98 | First proper dataset |

## Training datasets

| Dataset | # Fens      | Datagen runs                                                                                                          |
| ------- | ----------- | --------------------------------------------------------------------------------------------------------------------- |
|       0 |   3,847,979 | 20251013-114424                                                                                                       |
|       1 | 101,588,007 | 20251013-114424, 20251013-192617, 20251014-152147, 20251014-233358, 20251015-120401, 20251015-163757, 20251016-110035 |

## Datagen runs

All data used for training is self-play data generated using the datagen code from this repository.

| Run             | # Fens     | Info                              |
| --------------- | ---------- | --------------------------------- |
| 20251013-114424 |  3,847,979 | Depth 8, no persistent TT, no TBs |
| 20251013-192617 |  5,778,545 | Depth 8, no persistent TT, no TBs |
| 20251014-152147 |  4,751,407 | Depth 8, no persistent TT, no TBs |
| 20251014-233358 | 18,499,477 | Depth 8, no persistent TT, no TBs |
| 20251015-120401 |  8,955,294 | Depth 8, no persistent TT, no TBs |
| 20251015-163757 | 18,139,522 | Depth 8, no persistent TT, no TBs |
| 20251016-110035 | 41,615,783 | Depth 8, no persistent TT, no TBs |
