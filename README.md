## 🧵 User Story

### User Story

**As a** wardrobe-conscious user,  
**I want to** increment and track how many times I use certain types of clothing,  
**So that** I can reflect on my usage habits and make more conscious buying decisions.

# 🧵 Wardrobe Tracker (Solana + Anchor)

This is a basic on-chain wardrobe tracking program built using [Anchor](https://book.anchor-lang.com/).  
It allows users to track how many times they’ve used different categories of wardrobe items: clothing, shoes, and accessories.

> Use this application for practicing purposes

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
