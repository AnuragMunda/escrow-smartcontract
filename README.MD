# Escrow Smart Contract (Solana + Anchor)

This project implements a secure token-based **Escrow Smart Contract** using the **Anchor framework** on the **Solana blockchain**.  
It allows two parties to exchange tokens safely without trusting each other.

---

## 📌 What is an Escrow?

An **escrow** holds assets in a neutral account until specific conditions are met.  
The smart contract acts as the trusted intermediary, ensuring:

- Neither party can cheat.
- No assets move unless both sides meet the agreed terms.
- The exchange is **atomic** — all-or-nothing.

---

## 🎯 Use Case Overview

| Role | Sends | Receives |
|------|-------|----------|
| **Initializer (Party A)** | Token A | Token B |
| **Taker (Party B)** | Token B | Token A |

Token A is locked inside a **vault token account** owned by a **Program Derived Address (PDA)**.

Only the escrow program can move the funds from this vault — not the initializer, taker, or any external account.

---

## ⚙️ Workflow

### 1. **Initialize**
- The initializer deposits **Token A** into the **vault account**.
- Escrow terms (amounts + accounts) are stored in the `Escrow` state.
- Vault authority is changed to the **PDA**, making it secure.

### 2. **Accept**
- The taker sends **Token B** to the initializer.
- Program transfers **Token A** from the vault to the taker.
- Escrow is marked **inactive**.

### 3. **Cancel**
- If the taker doesn't accept, the initializer may cancel.
- The program returns **Token A** from the vault to the initializer.

---

## 🧱 Program Architecture

### On-Chain Accounts

| Account | Description |
|--------|-------------|
| `initializer` | The user starting the escrow |
| `taker` | The user accepting the escrow |
| `escrow` | Stores escrow configuration & state |
| `vault_account` | Holds Token A. Owned by PDA |
| `initializer_token_account` | Initial token source (Token A) |
| `initializer_receive_token_account` | Where Token B will be received |
| `taker_token_account` | Taker’s Token B source |
| `taker_receive_token_account` | Where Token A will be received |
| `token_program` | SPL token program for transfers |

---

## 📦 State Layout

```rust
pub struct Escrow {
    pub initializer: Pubkey,
    pub initializer_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub vault_account: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub bump: u8,
    pub is_active: bool,
}
```
---

## 🔐 Security Model
| Guarantee | Explanation |
|--------|-------------|
| `Vault` | controlled by PDA	Prevents unauthorized withdrawals
| `Atomic swaps` | Both token transfers happen in one transaction
| `Cancelable by initializer only` | Ensures funds recovery if trade not accepted
| `Escrow must be active` | Prevents double execution

---

## 🚀 Building & Deploying
### Requirements

- Rust Toolchain
- Solana CLI
- Anchor CLI
- Node.js + npm/yarn

### Build
```sh
anchor build
```

### Deploy to Devnet
```sh
solana config set --url devnet
anchor deploy
```

---

## 🧪 Testing

Run the test suite:
```sh
anchor test
```
If you need a ready-made test suite, ask and I’ll generate it.

---

## 🌱 Possible Extensions

| Feature |	Description |
|--------|-------------|
| Expiry / Time-lock |	Auto-cancel after deadline
| Marketplace Fees | Fee to platform or treasury
| NFT Escrow |	Swap NFTs for tokens or other NFTs
| Partial Fills | Allow multi-party participation

---

## 📄 License

This project is licensed under the MIT License.