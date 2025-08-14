# Network

This file contains information about the engine's networks and the data used to train them.

## Networks

Parameters in the following table are noted only where they differ from Bullet's defaults, which [can be found here](https://github.com/jgilchrist/bullet/blob/e1d5ced0916dbbc0c1e603e67542cbe99d2e05b7/src/main.rs).

| Network | Training data used | Architecture       | Parameters         | SPRT           | Notes         |
| ------- | ------------------ | ------------------ | ------------------ | -------------- | ------------- |
| 0       | 20251013-114424    | (768->16)x2->1     | WDL 0.1, LR 0.01   | 38.11 +- 13.69 | Hello world!  |

## Data

All data used for training is self-play data generated using the datagen code from this repository.

| Run             | # Fens   | Info                              |
| --------------- | -------- | --------------------------------- |
| 20251013-114424 |  3847979 | Depth 8, no persistent TT, no TBs |
| 20251013-192617 |  5778545 | Depth 8, no persistent TT, no TBs |
| 20251014-152147 |  4751407 | Depth 8, no persistent TT, no TBs |
| 20251014-233358 | 18499477 | Depth 8, no persistent TT, no TBs |
| 20251015-120401 |  8955294 | Depth 8, no persistent TT, no TBs |
| 20251015-163757 | 18139522 | Depth 8, no persistent TT, no TBs |
