## User Story

**As a** wardrobe-conscious user,  
**I want to** increment and track how many times I use certain types of clothing,  
**So that** I can reflect on my usage habits and make more conscious buying decisions.

## Program Architecture

This diagram shows the relationship between the user's wallet, the WardrobeCounter PDA, and the Solana program logic.

![Wardrobe Tracker Architecture](./docs/CounterUseCaseSolana.png)

## 🧵 Wardrobe Tracker (Solana + Anchor)

This is a basic on-chain wardrobe tracking program built using [Anchor](https://book.anchor-lang.com/).  
It allows users to track how many times they’ve used different categories of wardrobe items: clothing, shoes, and accessories.

> I create this application for practicing purposes

---

## 📦 Program Features

- Creates a `WardrobeCounter` account (PDA) per user
- Tracks number of times user wears:
  - 👕 Clothing
  - 👠 Shoes
  - 💍 Accessories
- All data is stored **on-chain** in a program-owned PDA

---

## 🛠 Tech Stack

- [Solana](https://solana.com/)
- [Anchor framework](https://book.anchor-lang.com/)
- TypeScript (for tests)
- Mocha + Chai (for assertions)

---

## 🚀 Getting Started

### ✅ 1. Clone the Repository

```bash
git clone https://github.com/vilmapato/wardrobe_tracker.git
cd wardrobe_tracker
```

### 2. Build the anchor program

```bash
anchor build
```

### 2. Test the program

```bash
anchor test
```
