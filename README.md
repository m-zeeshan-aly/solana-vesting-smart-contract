# Simple Solana Vesting Contract

This project contains a simple Solana vesting contract built using the Anchor framework. The contract allows users to deposit SOL, wait for a lock period (10 seconds), and then retrieve their SOL. It demonstrates the basics of smart contract development on the Solana blockchain and can be extended for more complex vesting logic. 

This repository will help you understand Solana, Anchor, and how to interact with smart contracts on the Solana blockchain.

## Table of Contents
1. [Overview](#overview)
2. [Technology Stack](#technology-stack)
3. [Installation](#installation)
4. [Usage](#usage)
5. [Development](#development)
6. [Commands](#commands)
7. [Testing](#testing)
8. [License](#license)

## Overview

The simple vesting contract on Solana allows users to:
1. **Deposit SOL**: Users can deposit SOL into the contract.
2. **Lock Period**: After the deposit, the user must wait for 10 seconds before withdrawing the funds.
3. **Withdraw SOL**: Once the lock period has expired, the user can withdraw the deposited funds.

This basic contract serves as a foundation for building more complex vesting contracts, including features like scheduled withdrawals, more flexible lock periods, and additional conditions based on user needs.

### Features:
- Deposit SOL into the contract.
- Enforce a lock period of 10 seconds.
- Withdraw funds after the lock period has passed.
- Easy to extend for more complex vesting functionality.

## Technology Stack

This project uses the following technologies:

- **Solana Blockchain**: The contract runs on the Solana blockchain, which is known for its speed and low transaction fees.
- **Anchor Framework**: A framework for developing Solana smart contracts. It simplifies the development process by providing a set of tools, like a test framework and easier account management.
- **Rust**: The smart contract is written in Rust, which is the language used for building Solana programs.
- **Node.js & Yarn**: These tools are used for interacting with the Solana blockchain, deploying the contract, and running tests.
- **Solana CLI**: A command-line tool for managing Solana accounts and interacting with the blockchain.
- **Anchor CLI**: A command-line tool for deploying and managing Anchor-based Solana programs.

## Installation

### Prerequisites

Before you start, ensure you have the following installed:

1. **Node.js**: Ensure you have [Node.js](https://nodejs.org/en/) installed. You can verify the installation by running `node -v` in the terminal.
2. **Yarn**: Install [Yarn](https://yarnpkg.com/) by running:
   ```bash
   npm install --global yarn
4. **Rust**: Install [Rust](https://www.rust-lang.org/). Follow the instructions on the website to install Rust on your machine. You can install it by running:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
6. **Install Solana CLI**:
  ```bash
  sh -c "$(curl -sSfL https://release.solana.com/v1.11.0/install)"
```bash
git clone https://github.com/yourusername/vesting-contract.git
cd vesting-contract
   
   

