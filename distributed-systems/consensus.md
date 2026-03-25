# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Consensus algorithms in distributed systems](#consensus-algorithms-in-distributed-systems)
- [Blockchain consensus](#blockchain-consensus)
  - [Smart contract](#smart-contract)
  - [PoW vs. PoS](#pow-vs-pos)
  - [DPoS vs. PoS](#dpos-vs-pos)
<!-- TOC -->

<br>

# Consensus algorithms in distributed systems
**Consensus** ensures that **all nodes** in a distributed system **agree** on a single value or decision despite potential failures, delays, or differences in their initial states.<br>
**Consensus algorithms** in distributed systems play a crucial role in maintaining consistency, reliability, and coordination across decentralized networks.<br>

<br>

**Consensus algorithms**:
- **Paxos** is a **single-value** consensus protocol;
- **Raft** is a **multi-valued** consensus protocol;
  - it **elects a leader** among a group of nodes, and the **leader** is responsible for coordinating the consensus process;
- **PBFT**

<br>

# Blockchain consensus
Blockchain uses a specific kind of distributed consensus to manage transactions and maintain a secure.<br>

<br>

**Blockchain consensus mechanisms**:
- **PoW**
- **PoS**
- **DPoS**

<br>

## Smart contract
A **smart contract** is a self-executing program that automates the actions required in a blockchain transaction.

<br>

##  PoW vs. PoS
![PoS-vs-PoW](/img/PoS-vs-PoW.png)

<br>

**PoW** is a **consensus mechanism** that requires participants (aka miners) to solve complex mathematical puzzles to validate transactions. The first miner to solve the puzzle earn rewards
is rewarded with newly minted cryptocurrency.<br>

<br>

The **main idea** behind PoW is in the **asymmetry of time costs**: a *client* **must solve** computational puzzle before it can send its request to a server, but the **result** of such computational puzzle can be **easily checked** by a **server**. So, client spends more resources that server and thus it **cannot** issue many **valid** mesage per unit of time, but it still can generate many **invalid** requests.<br>

<br>

**PoS** is far **more energy-efficient** than PoW. It removes the need for powerful mining equipment.<br>
In **PoS** a **stake** refers to certain amount of coins that users lock up to become network **validators**. Only **validators** can to add new blocks to the blockchain and verify transactions.<br>
**Selection mechanism**: network selects **validators** based on their **stake** size, i.e. higher stake amounts significantly increase the probability of being selected, in other words, a validator with more staked coins has the better chance to be selected.<br>

<br>

So,
- **PoS**:
  - **low** energy usage;
  - owners of the cryptocurrency can **stake their coins**, then **protocol selects a validator** who adds a new block of transactions and erns rewards;
- **PoW**:
  - **high** energy usage;
  - the **first** miner who **solves the puzzle** adds a new block of transactions and erns rewards;

<br>

## DPoS vs. PoS
In **DPoS** (**Delegated Proof of Stake**) only **delegates** can to add new blocks to the blockchain and verify transactions.<br>
These **delegates** are elected by the community through a **voting process**, ensuring decentralization and transparency.<br>
**DPoS** is **faster** than **PoS** because has smaller fixed number of nodes handle consensus.<br>
