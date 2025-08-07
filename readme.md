# Rust Wallet Tracker

A fast and secure command-line tool to track Ethereum wallet balances, transactions, and statistics, written in Rust. Supports querying on-chain data and provides detailed insights on wallet activity.

## Features

- Fetch ETH balance for a wallet
- Convert ETH balance to USD using real-time exchange rates
- Retrieve recent transactions with detailed and compact views
- Calculate average gas fees and ETH spent per transaction
- Generate wallet statistics (total transactions, averages, first transaction date)
- Export transaction data
- Easy-to-use CLI interface

## Getting Started

### Prerequisites

- Rust (latest stable version recommended) — [Install Rust](https://www.rust-lang.org/tools/install)
- `.env` file with your [Etherscan API key](https://etherscan.io/apis)

### Installation

Clone the repository:

```bash
git clone https://github.com/gonbvza/rust-wallet-tracker.git
cd rust-wallet-tracker
```

Add your Etherscan API key to .env:

```bash
ETHERSCAN_KEY=your_api_key_here
```

Build the project:

```
cargo build --release
```

Run the CLI tool:

```
cargo run --release
```

## Usage

When you run the program, you’ll be prompted to enter a wallet address and select actions such as:

- View ETH balance
- View USD balance
- List transactions
- Show average gas fees
- Show wallet statistics
- Export transactions
- Exit the program
