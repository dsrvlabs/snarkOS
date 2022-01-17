

# Welcome to DSRV Pool.

our mining pool always operated at **NO COST, NO COMMISSION!!** in Aleo Testnet2.

Profit will be distributed fairly with Aleo Team's guide or recommendation.
Currently, we have plan the reward distribution as follows.

```
prover's expected rewards = <prover's shares count> / <total shares count of our pool> * rewards for our pool
```

## Pool operator address
```
aleo1dgxjkw2989xd97k5eeucv6uvtgst5g5hh62p65par466qhv5xczsslrr58
```
Contact : `heejin|DSRV #4902` on Aleo Discord

# Quick Opeartion Guide
Run snarkOS as below, You can join our mining pool.

```
snarkos --prover <YOUR ALEO ADDRESS starting with aleo1> --pool 46.4.29.248:4132 --connect 46.4.29.248:4132 --trial --verbosity 2
``` 
- Please read below detailed operation guide, If you are not familiar with running snarkOS.
- Also, You can find more details on [snarkOS Github](https://github.com/AleoHQ/snarkOS)
- THE POOL OPERATOR AND ALL POOL PARTICIPANTS that contribute to the pool for any length of time will be required to complete KYC.

# DashBoard
You can find the shares of our pool in below dashboard.
 - Pool status
 - Prover status

http://165.227.146.192:3000/d/80kP3L1nz/dsrv-aleo-testnet-pool?orgId=2

![image](https://user-images.githubusercontent.com/83936146/149810805-44878991-9d3f-4f5b-85d7-099fc64502e0.png)

---

# Operation Guide
## <a name='TableofContents'></a>Table of Contents

* [1. Overview](#1-overview)
* [2. Prover Running Guide](#2-prover-running-guide)
    * [2.1 Requirements](#21-requirements)
    * [2.2 Installation](#22-installation)
* [3. Run an Aleo Prover Node](#3-run-an-aleo-prover-node)
* [4. Update SnarkOS](#4-update-snarkos)

## 1. Overview

 We are DSRV from South Korea, leading validator and infrastructure provider for more than 20+ top blockchain networks!

 We are a dedicated team of experts, united by a mission to provide secure and state-of-the-art infrastruture to the community.

 We offer a lot of RPC Node service at NO COST to community developers since we were started from community based validators.

 As we always has been, we are opening our mining pool for all community members, and our mining pool also operated at **"NO COST, NO COMMISSION"** as same as our RPC Node service allthatnode.com

- Node Types in Aleo mining pool
  - Operator : An operating node is a full node, capable of coordinating provers in a pool. We operate Aleo node as a Operator.
  - Prover : A proving node is a full node, capable of producing proofs for a pool.
You can join our mining pool as a prover, prover will be contributed to the probability of finding a block.
and operator will broadcast mined block over a network, and split the reward equally to provers.

## 2. Prover Running Guide

We always use pure source code from Aleo Team.
so, You can find more details on [Aleo Team Github](https://github.com/AleoHQ/snarkOS)

### 2.1 Requirements

The following are **minimum** requirements to run an Aleo node:
 - **CPU**: 16-cores (32-cores preferred)
 - **RAM**: 16GB of memory (32GB preferred)
 - **Storage**: 128GB of disk space
 - **Network**: 50 Mbps of upload **and** download bandwidth

Please note to run an Aleo mining node that is **competitive**, the machine will require more than these requirements.

### 2.2 Installation

> Before beginning, please ensure your machine has `Rust v1.56+` installed. Instructions to [install Rust can be found here.](https://www.rust-lang.org/tools/install)
>

Start by cloning the snarkOS Github repository:
```
git clone https://github.com/AleoHQ/snarkOS.git --depth 1
```

Next, move into the snarkOS directory:
```
cd snarkOS
```

**[For Ubuntu users]** A helper script to install dependencies is available. From the snarkOS directory, run:
```
./testnet2_ubuntu.sh
```


## 3. Run an Aleo Prover Node

Start by following the instructions in the [Build Guide](#2-build-guide).

Next, to generate an Aleo miner address, run:
```
snarkos experimental new_account 
```
or from the snarkOS directory, run:
```
cargo run --release -- experimental new_account
```
This will output a new Aleo account in the terminal.

**Please remember to save the account private key and view key.** The following is an example output:
```
 Attention - Remember to store this account private key and view key.

  Private Key  APrivateKey1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  <-- Save Me
     View Key  AViewKey1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  <-- Save Me
      Address  aleo1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx  <-- Use Me For The Next Step
```

Then, Running snarkOS as prover
```
snarkos --prover <YOUR ALEO ADDRESS starting with aleo1> --pool 46.4.29.248:4132 --connect 46.4.29.248:4132 --trial --verbosity 2
```
```

         ╦╬╬╬╬╬╦
        ╬╬╬╬╬╬╬╬╬                    ▄▄▄▄        ▄▄▄
       ╬╬╬╬╬╬╬╬╬╬╬                  ▐▓▓▓▓▌       ▓▓▓
      ╬╬╬╬╬╬╬╬╬╬╬╬╬                ▐▓▓▓▓▓▓▌      ▓▓▓     ▄▄▄▄▄▄       ▄▄▄▄▄▄
     ╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬              ▐▓▓▓  ▓▓▓▌     ▓▓▓   ▄▓▓▀▀▀▀▓▓▄   ▐▓▓▓▓▓▓▓▓▌
    ╬╬╬╬╬╬╬╜ ╙╬╬╬╬╬╬╬            ▐▓▓▓▌  ▐▓▓▓▌    ▓▓▓  ▐▓▓▓▄▄▄▄▓▓▓▌ ▐▓▓▓    ▓▓▓▌
   ╬╬╬╬╬╬╣     ╠╬╬╬╬╬╬           █▓▓▓▓▓▓▓▓▓▓█    ▓▓▓  ▐▓▓▀▀▀▀▀▀▀▀▘ ▐▓▓▓    ▓▓▓▌
  ╬╬╬╬╬╬╣       ╠╬╬╬╬╬╬         █▓▓▓▌    ▐▓▓▓█   ▓▓▓   ▀▓▓▄▄▄▄▓▓▀   ▐▓▓▓▓▓▓▓▓▌
 ╬╬╬╬╬╬╣         ╠╬╬╬╬╬╬       ▝▀▀▀▀      ▀▀▀▀▘  ▀▀▀     ▀▀▀▀▀▀       ▀▀▀▀▀▀
╚╬╬╬╬╬╩           ╩╬╬╬╬╩

Welcome to Aleo! We thank you for running a network node and supporting privacy.

Your Aleo address is aleo1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx

Starting a proving node on testnet2.


 ==================================================================================================

                         Welcome to Aleo Testnet2 - Incentivization Period

 ==================================================================================================

```

### Status Report

After the proving node has booted up, a periodic report is provided with the status of node:
```
2022-01-11T07:32:22.769020Z  INFO Prover successfully mined a share for unconfirmed block 152061 with proof difficulty of 12619414114267194111
2022-01-11T07:32:22.769070Z TRACE Sending 'PoolResponse' to 46.4.29.248:4132
2022-01-11T07:32:22.832205Z TRACE Sending 'PoolRegister' to 46.4.29.248:4132
2022-01-11T07:32:23.554153Z DEBUG Peer 89.113.168.108:4132 is at block 152068 (type = Miner, status = Mining, is_fork = false, cumulative_weight = 47070914429, common_ancestor = 152068)
```

## License

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)
