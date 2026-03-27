# 🍛 food-planner

A **Rust-based command-line food planner** for a South Indian developer family in Chennai — powered by **GitHub Copilot AI**.

Generates a complete **weekly meal plan** with breakfast, kids' snacks, kids' lunch, and family dinner, plus a consolidated **shopping list** organised by ingredient category.

---

## 👨‍👩‍👧‍👦 Family Profile

| Members | Count | Ages |
|---------|-------|------|
| Adults  | 4     | 30–37 years |
| Kids    | 2     | 7–10 years |
| Infant  | 1     | 2 years |

**Cuisine:** South Indian — Chennai / Tamil Nadu  
**Proteins:** Egg · Fish · Chicken · Mutton

---

## 📅 Weekly Meal Rules

| Day | Special Rule |
|-----|-------------|
| **Saturday** | Pongal or Poori breakfast |
| **Sunday** | Non-veg special dinner (Mutton/Chicken Biryani) |
| **Wednesday** | Chicken curry dinner (mid-week) |
| **Tuesday** | Mushroom dish for dinner |
| **Every day** | South Indian breakfast, kids' fruit snack, kids' lunch |

---

## 🚀 Installation

### Prerequisites
- [Rust](https://rustup.rs/) 1.70+
- A GitHub account with [GitHub Copilot](https://github.com/features/copilot) subscription (for AI generation)

### Build from source

```bash
git clone https://github.com/raathi/food-planner.git
cd food-planner
cargo build --release
# Binary will be at ./target/release/food-planner
```

---

## 🛠 Usage

### Generate a weekly meal plan

```bash
# AI-powered plan (requires GitHub token configured)
food-planner generate

# Built-in South Indian template (offline mode)
food-planner generate --offline

# For a specific week starting date
food-planner generate --date 2024-03-25

# JSON output
food-planner generate --offline --output json
```

### Get the weekly shopping list

```bash
food-planner shopping
food-planner shopping --date 2024-03-25
food-planner shopping --output json
```

### View family profile and meal rules

```bash
food-planner profile
```

### Configure GitHub Copilot AI

```bash
# Set your GitHub token (requires Copilot subscription)
food-planner config --token ghp_yourTokenHere

# Or use an environment variable
export GITHUB_TOKEN=ghp_yourTokenHere

# View current configuration
food-planner config --show

# Change AI model
food-planner config --model gpt-4o
```

---

## 🤖 GitHub Copilot Integration

The tool uses the **GitHub Copilot API** (OpenAI-compatible) to generate personalised weekly meal plans.

**How to get a GitHub token:**
1. Go to [github.com/settings/tokens](https://github.com/settings/tokens)
2. Generate a new token (classic) with `read:user` scope
3. Ensure your GitHub account has an active Copilot subscription
4. Run: `food-planner config --token <your-token>`

**Fallback:** If no token is configured, the tool automatically uses the built-in South Indian meal template library — no internet required.

---

## 🍽 Sample Output

```
╔══════════════════════════════════════════════════════════════╗
║       🍛  WEEKLY SOUTH INDIAN FOOD PLAN  🍚                 ║
║  Family: 4 adults • 2 kids • 1 infant | Chennai Style       ║
║  Week of: 25 March 2024                                      ║
╚══════════════════════════════════════════════════════════════╝

  MONDAY 25 Mar  💻
  🌅 Breakfast  Idli with Sambar & Chutneys
  🍎 Kids' Snack  Banana & Dates
  🎒 Kids' Lunch  Rice + Sambar + Egg Curry
  🌙 Dinner  Fish Curry with Rice & Papad

  TUESDAY 26 Mar  ☕
  🌅 Breakfast  Upma with Coconut Chutney
  🍎 Kids' Snack  Seasonal Fruit Salad
  🎒 Kids' Lunch  Curd Rice with Pickle
  🌙 Dinner  Mushroom Masala with Chapathi & Rice   🍄

  WEDNESDAY 27 Mar  🍗
  🌅 Breakfast  Dosa with Sambar & Chutney
  🍗 Dinner  Chicken Chettinad Curry with Rice & Rasam

  SATURDAY 30 Mar  🍚
  🌅 Breakfast  Ven Pongal with Sambar & Chutney

  SUNDAY 31 Mar  🥩
  🌙 Dinner  Mutton Biryani with Raita & Boiled Egg
```

---

## 🛒 Shopping List Sample

```
📦 Proteins (Meat/Eggs/Fish)
  ▸ 800 g  Mutton (bone-in)
  ▸ 800 g  Chicken (curry cut)
  ▸  16 pcs Eggs
  ▸ 1200 g  Fish (Seer fish / Vanjaram)

📦 Vegetables
  ▸ 500 g  Button mushrooms
  ▸  38 pcs Tomatoes
  ▸  32 pcs Onion
```

---

## 🧪 Testing

```bash
cargo test
```

---

## 📁 Project Structure

```
src/
├── main.rs       — CLI entry point and command handling
├── lib.rs        — Library exports for testing
├── models.rs     — Data types: Meal, WeeklyPlan, Ingredient, etc.
├── planner.rs    — Built-in South Indian meal template engine
├── ai.rs         — GitHub Copilot API client
├── config.rs     — Configuration management (TOML, env vars)
└── display.rs    — Coloured terminal output and JSON export

tests/
└── meal_plan_tests.rs  — Integration tests for meal rules
```

---

## 📜 License

MIT
