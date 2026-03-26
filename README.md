# SplitPay

## Project Title
**SplitPay** - Decentralized Expense Sharing on Stellar

---

## Project Description

SplitPay is a decentralized expense sharing application built on the Stellar blockchain. It allows users to create groups, track shared expenses, and settle balances fairly using Soroban smart contracts. Whether you're splitting bills with roommates, organizing a group trip, or managing shared expenses with friends, SplitPay provides a transparent and trustless way to handle group finances.

The application eliminates the need for manual calculations and trust between group members by leveraging blockchain technology to automatically calculate fair splits and handle settlements on-chain.

---

## Project Vision

Our vision is to create a **fair, transparent, and accessible expense sharing platform** for everyone. We believe that:

- **Financial transparency** should be the default, not the exception
- **Group expenses** shouldn't require trust in a single person to manage funds
- **Settlement** should be automatic and fair, with no room for disputes
- **Blockchain technology** can simplify everyday financial interactions

SplitPay aims to become the go-to solution for group expense management, making it as easy to split a bill on the blockchain as it is to use traditional payment apps—while providing the added benefits of transparency, security, and decentralization.

---

## Key Features

### 1. Wallet Integration
- Seamless connection with Freighter wallet (Stellar's most popular browser extension wallet)
- Secure transaction signing directly in your browser
- Automatic network detection (testnet/mainnet)

### 2. Group Management
- Create expense groups with custom names
- Add multiple members to each group using Stellar addresses
- View all groups in a personalized dashboard
- Track group creation date and member count

### 3. Expense Tracking
- Add expenses with descriptions and amounts
- Automatic fair splitting among all group members
- Real-time balance calculations showing who owes what
- Complete expense history for each group

### 4. Smart Contract Settlement
- On-chain settlement using Soroban smart contracts
- Automatic fund distribution to all members
- Secure escrow mechanism ensures fair payments
- Transaction receipts and confirmation tracking

### 5. Modern User Interface
- Responsive design that works on desktop and mobile
- Clean, gradient-based dashboard with intuitive navigation
- Real-time updates when transactions confirm on-chain
- Dark/light mode support

### 6. Transparency & Security
- All transactions recorded on the Stellar blockchain
- Immutable expense and settlement history
- No central authority controlling funds
- Open-source smart contracts for auditability

---

## Deployed Smart Contract Details

### Contract Information

| Parameter | Value |
|-----------|-------|
| **Contract ID** | `CDVJA6VO3AK7EOZXZ7QKXZUAEEJMNYPR35NZC4B3BGK55AUTQHWSZSMR` |
| **Network** | Stellar Testnet |
| **Token Contract** | `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC` |
| **Explorer URL** | [View on StellarExpert](https://stellar.expert/explorer/testnet/contract/CDVJA6VO3AK7EOZXZ7QKXZUAEEJMNYPR35NZC4B3BGK55AUTQHWSZSMR) |

### Contract Verification

You can verify the deployed contract using the following methods:

1. **StellarExpert**: Visit the contract page using the link above to see transaction history and contract details

2. **Stellar Laboratory**: Use the [Stellar Laboratory](https://laboratory.stellar.org) to query contract state and invoke functions

3. **Soroban CLI**:
   ```bash
   soroban contract read --id CDVJA6VO3AK7EOZXZ7QKXZUAEEJMNYPR35NZC4B3BGK55AUTQHWSZSMR \
     --rpc-url https://soroban-testnet.stellar.org \
     --source-account your_key
   ```

### Block Explorer Screenshot

Below is a screenshot of the deployed contract on StellarExpert:

![StellarExpert](/screenshots/stellarexpert.png)

---

## UI Screenshots

### Dashboard

The main dashboard provides an overview of all your expense groups with quick access to create new groups and manage existing ones.

![Dashboard](/screenshots/dashboard.png)

### Create Group

Easily create new expense groups by providing a name and adding members with their Stellar addresses.

![Create Group](/screenshots/create-group.png)

### Group Details & Expenses

View detailed expense history, member balances, and settlement status for each group.

![Group Details and Expenses](/screenshots/group-details-and-expenses.png)

### Wallet Connection

Connect your Freighter wallet securely to interact with the Stellar blockchain.

![Wallet Connection](/screenshots/wallet-connection.png)

### Settlement

Settle group balances on-chain with a single click, distributing funds fairly to all members.

![Settlement](/screenshots/settlement.png)

---
## Project Setup Guide

### Prerequisites

Before you begin, ensure you have:

- **Node.js** (v18 or higher) - [Download here](https://nodejs.org/)
- **Git** - [Download here](https://git-scm.com/)
- **Freighter Wallet** extension installed in your browser - [Get Freighter](https://www.freighter.app/)
- **Stellar Testnet account** with some XLM (use [Friendbot](https://laboratory.stellar.org/#account-creator) to fund your account)

### Installation Steps

#### 1. Clone the Repository

```bash
git clone https://github.com/MayankDew08/splitpay.git
cd splitpay
```

#### 2. Install Frontend Dependencies

```bash
cd frontend
npm install
```

#### 3. Configure Environment Variables

Create a `.env.local` file in the `frontend` directory:

```bash
touch .env.local
```

Add the following configuration:

```env
# Smart Contract Configuration
NEXT_PUBLIC_CONTRACT_ID=CDVJA6VO3AK7EOZXZ7QKXZUAEEJMNYPR35NZC4B3BGK55AUTQHWSZSMR
NEXT_PUBLIC_TOKEN_ID=CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC

# Network Configuration
NEXT_PUBLIC_NETWORK=testnet
NEXT_PUBLIC_HORIZON_URL=https://horizon-testnet.stellar.org
NEXT_PUBLIC_SOROBAN_URL=https://soroban-testnet.stellar.org
```

#### 4. Run the Development Server

```bash
npm run dev
```

The application will be available at [http://localhost:3000](http://localhost:3000)

#### 5. Connect Your Wallet

1. Open the app in your browser
2. Click "Connect Wallet"
3. Authorize the connection in Freighter
4. Ensure your wallet is set to Testnet mode

### Smart Contract Development (Optional)

If you want to modify or deploy the smart contract:

#### Prerequisites
- **Rust** - [Install Rust](https://www.rust-lang.org/tools/install)
- **Soroban CLI** - [Installation guide](https://soroban.stellar.org/docs/getting-started/setup)

#### Build and Test

```bash
cd contracts/splitpay

# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test

# Deploy (requires testnet funds)
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/splitpay.wasm \
  --source your_key \
  --rpc-url https://soroban-testnet.stellar.org
```

---

## Technology Stack

| Component | Technology |
|-----------|------------|
| Frontend | Next.js 14, TypeScript, Tailwind CSS |
| Blockchain | Stellar SDK, Soroban RPC |
| Wallet | Freighter API |
| Smart Contracts | Rust (Soroban SDK) |
| UI Components | Radix UI, Tailwind CSS |
| Icons | Lucide React |

---

## Future Scope

### Near-term (Next 3 months)
- **Multi-currency Support**: Add support for multiple tokens (USDC, native assets)
- **Expense Categories**: Categorize expenses (food, travel, utilities) with visual breakdowns
- **Recurring Expenses**: Set up recurring bills for regular group payments
- **Push Notifications**: Real-time alerts for new expenses and settlements

### Medium-term (3-6 months)
- **Mobile App**: Native iOS and Android applications using React Native
- **Off-chain Messaging**: In-app chat for group members to discuss expenses
- **Receipt Upload**: Attach receipt images to expenses for record keeping
- **Advanced Split Options**: Uneven splits (percentages, custom amounts)

### Long-term (6+ months)
- **Mainnet Migration**: Deploy contracts to Stellar mainnet for production use
- **Payment Integration**: Direct fiat on/off-ramps for easier onboarding
- **Multi-signature Groups**: Require multiple approvals for large expenses
- **Analytics Dashboard**: Spending insights and budget tracking across groups
- **DAO Integration**: Community governance for protocol upgrades

### Potential Integrations
- **Stellar Anchor**: Fiat currency deposits and withdrawals
- **Interledger**: Cross-ledger payments for non-Stellar users
- **IPFS**: Decentralized storage for receipt images
- **Price Oracles**: Real-time currency conversion rates

---

## License

This project is licensed under the MIT License.

---

## Support

For questions or support, please open an issue on GitHub or contact the development team.

---

**Built with ❤️ on Stellar**