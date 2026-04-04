# 🏢 RWA Market — Tokenization of Real-World Assets on Solana

## 📌 Problem
Real-world assets (real estate, bonds, commodities) are:
- Inaccessible to most people due to high entry costs
- Illiquid and hard to transfer
- Non-transparent in ownership and operations

## 💡 Solution
RWA Market is a platform that tokenizes real-world assets on Solana, enabling fractional ownership. Large companies list their assets, and anyone can buy a fraction starting from as little as $10.

## 🚀 How It Works

### User Flow
1. **Company** connects wallet → creates an asset (e.g. "Almaty Business Center") → sets total value and number of fractions
2. **Investor** connects wallet → browses available assets → buys fractions with SOL
3. **Company** distributes yield (rent/dividends) proportionally to fraction holders

## 🔧 Tech Stack
- **Blockchain:** Solana (Devnet)
- **Smart Contract:** Rust + Anchor 1.0.0
- **Frontend:** HTML / CSS / JavaScript
- **Wallet:** Phantom

## 📦 Smart Contract

**Program ID:** `9GWa9HyP887TpVKAqk9qwd6TaCyVyULgARkEY4wCFDkm`

**Network:** Solana Devnet

**Explorer:** [View on Solana Explorer](https://explorer.solana.com/address/9GWa9HyP887TpVKAqk9qwd6TaCyVyULgARkEY4wCFDkm?cluster=devnet)

### Instructions
| Instruction | Description |
|-------------|-------------|
| `create_asset` | Company tokenizes a real-world asset |
| `buy_fractions` | Investor purchases fractional ownership |
| `distribute_yield` | Owner distributes income to investors |

### On-chain State
| Account | Fields |
|---------|--------|
| `Asset` | authority, name, description, total_value, total_fractions, available_fractions, price_per_fraction |
| `InvestorPosition` | investor, asset, fractions_owned, yield_claimed |

## 🏗️ Architecture
Frontend (HTML/JS)
↓
Phantom Wallet (transaction signing)
↓
Solana Devnet
↓
RWA Program (Anchor 1.0.0)
↓
On-chain State (Asset + InvestorPosition accounts)

## 🛠️ Local Setup
```bash
# Install dependencies
npm install -g @coral-xyz/anchor-cli

# Build smart contract
anchor build

# Deploy to devnet
anchor program deploy target/deploy/rwa_v2.so \
  --program-id 9GWa9HyP887TpVKAqk9qwd6TaCyVyULgARkEY4wCFDkm \
  --program-keypair target/deploy/rwa_v2-keypair.json
```

## 📈 Scaling Potential
- Add SPL token minting for tradeable fraction tokens
- Integrate Metaplex for asset metadata and documents
- Add oracle (Pyth/Switchboard) for real-time asset valuation
- Build P2P marketplace for fraction trading
- KYC/AML compliance layer for institutional investors

## 🏆 Hackathon
Built for **Colosseum Hackathon** — RWA Tokenization track.