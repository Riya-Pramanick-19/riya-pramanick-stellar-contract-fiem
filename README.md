
# 🌱 CarbonChain — Permissionless Carbon Credit DApp

A fully **permissionless carbon credit marketplace** built on Stellar using Soroban smart contracts.  
Mint, trade, and retire carbon credits — with **no intermediaries**.

---

## 🔗 Contract Details

Contract ID (Stellar):CAGDK57JMLKMB4HKV2M2FTNM5RQSTVAODCVBDOA3KGTXOFBYMXXP7IXK

---

## 🚀 Features

- 🌱 **Mint Credits** — Anyone can create carbon credits  
- 💰 **Marketplace** — List & buy credits freely  
- 🔥 **Retire Credits** — Permanently burn after use  
- 👛 **Wallet Integration** — Works with Freighter  
- 🔐 **Secure Actions** — Uses `require_auth()`  

> No admin. No approvals. Fully decentralized.

---

## 🧠 Core Idea

**Permissionless by default**

Anyone can:
- Create credits  
- Trade credits  
- Participate freely  

---

## 🏗️ Tech Stack

- Stellar (Soroban, Rust)
- React / Next.js
- Stellar SDK
- Freighter Wallet

---

## 📸 Screenshots

<img width="1919" height="1079" alt="Screenshot 2026-04-09 150354" src="https://github.com/user-attachments/assets/26260fdf-15e6-4661-8c7d-c48a7a8ef085" />

### 🏠 Homepage
![Homepage](./screenshots/home.png)

### 🌱 Mint Credit
![Mint](./screenshots/mint.png)

### 💰 Marketplace
![Marketplace](./screenshots/marketplace.png)

### 🔥 Retire Credits
![Retire](./screenshots/retire.png)

---

## 📦 Smart Contract Functions

- `create_credit` → Mint new credit  
- `list_credit` → Put credit for sale  
- `buy_credit` → Transfer ownership  
- `retire_credit` → Burn credit  
- `get_credit` → View details  
- `get_total_credits` → Total supply  

---

## ⚙️ Quick Start

```bash
# Build contract
cd contract
stellar contract build

# Deploy contract
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source YOUR_ACCOUNT

# Run frontend
cd ../client
npm install
npm run dev
