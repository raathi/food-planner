use crate::models::*;
use chrono::{Datelike, NaiveDate, Weekday};
use std::collections::HashMap;

pub struct MealPlanner;

impl MealPlanner {
    /// Generate a weekly plan using the built-in template logic.
    /// This is used as fallback when AI is unavailable or offline mode is requested.
    pub fn generate_offline_plan(week_start: NaiveDate) -> WeeklyPlan {
        let mut days = Vec::new();

        for day_offset in 0..7u64 {
            let date = week_start + chrono::Duration::days(day_offset as i64);
            let weekday = date.weekday();
            let day_name = weekday_name(weekday);
            let meals = Self::meals_for_day(weekday, day_offset);

            days.push(DayPlan {
                date,
                day_name,
                meals,
            });
        }

        let shopping_list = Self::build_shopping_list(&days);

        WeeklyPlan {
            week_start,
            days,
            shopping_list,
        }
    }

    fn meals_for_day(weekday: Weekday, day_offset: u64) -> Vec<Meal> {
        let mut meals = Vec::new();

        // --- Breakfast ---
        let breakfast = match weekday {
            Weekday::Sat => saturday_breakfast(),
            Weekday::Sun => sunday_breakfast(),
            _ => weekday_breakfast(day_offset),
        };
        meals.push(breakfast);

        // --- Kids' Snack (fruit-based) ---
        meals.push(kids_fruit_snack(day_offset));

        // --- Kids' Lunch ---
        meals.push(kids_lunch(day_offset));

        // --- Dinner ---
        let dinner = match weekday {
            Weekday::Sun => sunday_nonveg_dinner(),
            Weekday::Wed => chicken_dinner(),
            _ => {
                // Mushroom dish on Tuesday
                if weekday == Weekday::Tue {
                    mushroom_dinner()
                } else {
                    regular_dinner(day_offset)
                }
            }
        };
        meals.push(dinner);

        meals
    }

    fn build_shopping_list(days: &[DayPlan]) -> Vec<ShoppingItem> {
        let mut aggregated: HashMap<String, (f64, String, IngredientCategory)> = HashMap::new();

        for day in days {
            for meal in &day.meals {
                for ing in &meal.ingredients {
                    let qty: f64 = ing.quantity.parse().unwrap_or(1.0);
                    let entry = aggregated
                        .entry(ing.name.clone())
                        .or_insert((0.0, ing.unit.clone(), ing.category.clone()));
                    entry.0 += qty;
                }
            }
        }

        let mut items: Vec<ShoppingItem> = aggregated
            .into_iter()
            .map(|(name, (qty, unit, category))| ShoppingItem {
                ingredient: name,
                total_quantity: format!("{}", qty),
                unit,
                category,
            })
            .collect();

        // Sort by category then name
        items.sort_by(|a, b| {
            format!("{}{}", a.category, a.ingredient)
                .cmp(&format!("{}{}", b.category, b.ingredient))
        });

        items
    }
}

// ── Breakfast helpers ─────────────────────────────────────────────────────────

fn saturday_breakfast() -> Meal {
    // Saturday special: Pongal or Poori (alternating)
    Meal {
        meal_type: MealType::Breakfast,
        name: "Ven Pongal with Sambar & Chutney".to_string(),
        description: "Traditional Saturday breakfast - creamy rice and moong dal pongal served with sambar and coconut chutney".to_string(),
        ingredients: vec![
            ing("Raw rice", "500", "g", IngredientCategory::Grains),
            ing("Moong dal (yellow)", "150", "g", IngredientCategory::Grains),
            ing("Ghee", "50", "ml", IngredientCategory::Oils),
            ing("Black pepper", "10", "g", IngredientCategory::Spices),
            ing("Cumin seeds", "10", "g", IngredientCategory::Spices),
            ing("Cashews", "30", "g", IngredientCategory::Other),
            ing("Curry leaves", "2", "sprig", IngredientCategory::Vegetables),
            ing("Toor dal (for sambar)", "100", "g", IngredientCategory::Grains),
            ing("Tomatoes", "3", "pcs", IngredientCategory::Vegetables),
            ing("Onion", "2", "pcs", IngredientCategory::Vegetables),
            ing("Tamarind", "20", "g", IngredientCategory::Spices),
            ing("Sambar powder", "20", "g", IngredientCategory::Spices),
            ing("Coconut (grated)", "100", "g", IngredientCategory::Other),
            ing("Green chilli", "3", "pcs", IngredientCategory::Vegetables),
        ],
        suitable_for: vec!["adults".to_string(), "kids".to_string()],
    }
}

fn sunday_breakfast() -> Meal {
    Meal {
        meal_type: MealType::Breakfast,
        name: "Egg Dosa with Tomato Chutney".to_string(),
        description: "Sunday special crispy dosas topped with spiced egg scramble and fresh tomato chutney".to_string(),
        ingredients: vec![
            ing("Dosa batter (idli rice + urad dal)", "1000", "g", IngredientCategory::Grains),
            ing("Eggs", "6", "pcs", IngredientCategory::Proteins),
            ing("Onion", "2", "pcs", IngredientCategory::Vegetables),
            ing("Green chilli", "3", "pcs", IngredientCategory::Vegetables),
            ing("Tomatoes", "4", "pcs", IngredientCategory::Vegetables),
            ing("Coriander leaves", "1", "bunch", IngredientCategory::Vegetables),
            ing("Oil", "50", "ml", IngredientCategory::Oils),
            ing("Salt", "10", "g", IngredientCategory::Spices),
            ing("Turmeric powder", "5", "g", IngredientCategory::Spices),
        ],
        suitable_for: vec!["adults".to_string(), "kids".to_string()],
    }
}

fn weekday_breakfast(day_offset: u64) -> Meal {
    let breakfasts = [
        (
            "Idli with Sambar & Chutneys",
            "Soft steamed rice and lentil cakes served with hot sambar and coconut chutney",
            vec![
                ing("Idli batter (rice + urad dal)", "800", "g", IngredientCategory::Grains),
                ing("Toor dal (for sambar)", "100", "g", IngredientCategory::Grains),
                ing("Tomatoes", "2", "pcs", IngredientCategory::Vegetables),
                ing("Onion", "1", "pcs", IngredientCategory::Vegetables),
                ing("Coconut (grated)", "80", "g", IngredientCategory::Other),
                ing("Sambar powder", "15", "g", IngredientCategory::Spices),
                ing("Mustard seeds", "5", "g", IngredientCategory::Spices),
                ing("Curry leaves", "2", "sprig", IngredientCategory::Vegetables),
                ing("Oil", "30", "ml", IngredientCategory::Oils),
            ],
        ),
        (
            "Upma with Coconut Chutney",
            "Savory semolina breakfast with vegetables, mustard tempering and fresh coconut chutney",
            vec![
                ing("Rava (semolina)", "300", "g", IngredientCategory::Grains),
                ing("Onion", "2", "pcs", IngredientCategory::Vegetables),
                ing("Green chilli", "3", "pcs", IngredientCategory::Vegetables),
                ing("Carrot", "1", "pcs", IngredientCategory::Vegetables),
                ing("Beans", "50", "g", IngredientCategory::Vegetables),
                ing("Mustard seeds", "5", "g", IngredientCategory::Spices),
                ing("Urad dal", "15", "g", IngredientCategory::Grains),
                ing("Curry leaves", "2", "sprig", IngredientCategory::Vegetables),
                ing("Coconut (grated)", "80", "g", IngredientCategory::Other),
                ing("Oil", "40", "ml", IngredientCategory::Oils),
                ing("Ghee", "15", "ml", IngredientCategory::Oils),
            ],
        ),
        (
            "Dosa with Sambar & Chutney",
            "Crispy fermented crepes with lentil sambar and coconut chutney",
            vec![
                ing("Dosa batter (idli rice + urad dal)", "1000", "g", IngredientCategory::Grains),
                ing("Toor dal (for sambar)", "100", "g", IngredientCategory::Grains),
                ing("Tomatoes", "2", "pcs", IngredientCategory::Vegetables),
                ing("Onion", "2", "pcs", IngredientCategory::Vegetables),
                ing("Coconut (grated)", "80", "g", IngredientCategory::Other),
                ing("Sambar powder", "15", "g", IngredientCategory::Spices),
                ing("Oil", "50", "ml", IngredientCategory::Oils),
            ],
        ),
        (
            "Idiyappam with Coconut Milk",
            "Steamed rice noodles served with sweet coconut milk - light and nutritious",
            vec![
                ing("Rice flour (idiyappam)", "400", "g", IngredientCategory::Grains),
                ing("Coconut milk", "400", "ml", IngredientCategory::Dairy),
                ing("Coconut (grated)", "100", "g", IngredientCategory::Other),
                ing("Jaggery", "50", "g", IngredientCategory::Other),
                ing("Salt", "5", "g", IngredientCategory::Spices),
            ],
        ),
        (
            "Poha (Aval) with Curd",
            "Flattened rice tempered with mustard, curry leaves, and served with fresh curd",
            vec![
                ing("Poha / Aval (flattened rice)", "300", "g", IngredientCategory::Grains),
                ing("Onion", "1", "pcs", IngredientCategory::Vegetables),
                ing("Green chilli", "2", "pcs", IngredientCategory::Vegetables),
                ing("Peanuts", "50", "g", IngredientCategory::Proteins),
                ing("Mustard seeds", "5", "g", IngredientCategory::Spices),
                ing("Curry leaves", "2", "sprig", IngredientCategory::Vegetables),
                ing("Curd / Yogurt", "200", "ml", IngredientCategory::Dairy),
                ing("Oil", "20", "ml", IngredientCategory::Oils),
            ],
        ),
    ];

    let idx = day_offset as usize % breakfasts.len();
    let (name, desc, ingredients) = breakfasts[idx].clone();

    Meal {
        meal_type: MealType::Breakfast,
        name: name.to_string(),
        description: desc.to_string(),
        ingredients,
        suitable_for: vec!["adults".to_string(), "kids".to_string()],
    }
}

// ── Kids' snack (fruits) ──────────────────────────────────────────────────────

fn kids_fruit_snack(day_offset: u64) -> Meal {
    let snacks = [
        ("Banana & Dates", "2 bananas + 4 dates per child", vec![
            ing("Banana", "4", "pcs", IngredientCategory::Fruits),
            ing("Dates", "8", "pcs", IngredientCategory::Fruits),
        ]),
        ("Seasonal Fruit Salad", "Mixed seasonal fruits with a drizzle of honey", vec![
            ing("Apple", "2", "pcs", IngredientCategory::Fruits),
            ing("Papaya", "200", "g", IngredientCategory::Fruits),
            ing("Pomegranate", "1", "pcs", IngredientCategory::Fruits),
            ing("Honey", "15", "ml", IngredientCategory::Other),
        ]),
        ("Mango (seasonal) / Sapota", "Fresh mango or chiku – a Tamil Nadu favorite", vec![
            ing("Mango", "3", "pcs", IngredientCategory::Fruits),
        ]),
        ("Guava with Chilli Salt", "Fresh guava slices with a pinch of chilli powder and salt", vec![
            ing("Guava", "4", "pcs", IngredientCategory::Fruits),
        ]),
        ("Orange & Watermelon", "Hydrating fruit combo ideal for Chennai weather", vec![
            ing("Orange", "4", "pcs", IngredientCategory::Fruits),
            ing("Watermelon", "500", "g", IngredientCategory::Fruits),
        ]),
        ("Banana Smoothie", "Blended banana with milk and a pinch of cardamom", vec![
            ing("Banana", "3", "pcs", IngredientCategory::Fruits),
            ing("Milk", "300", "ml", IngredientCategory::Dairy),
            ing("Cardamom powder", "2", "g", IngredientCategory::Spices),
        ]),
        ("Papaya & Pineapple", "Enzyme-rich tropical fruit combo", vec![
            ing("Papaya", "300", "g", IngredientCategory::Fruits),
            ing("Pineapple", "200", "g", IngredientCategory::Fruits),
        ]),
    ];

    let idx = day_offset as usize % snacks.len();
    let (name, desc, ingredients) = snacks[idx].clone();

    Meal {
        meal_type: MealType::KidsSnack,
        name: name.to_string(),
        description: desc.to_string(),
        ingredients,
        suitable_for: vec!["kids".to_string(), "infant".to_string()],
    }
}

// ── Kids' lunch ───────────────────────────────────────────────────────────────

fn kids_lunch(day_offset: u64) -> Meal {
    let lunches = [
        (
            "Rice + Sambar + Egg Curry",
            "Steamed rice with mild sambar and a simple egg curry, easy on kids' stomachs",
            vec![
                ing("Raw rice", "300", "g", IngredientCategory::Grains),
                ing("Toor dal (for sambar)", "80", "g", IngredientCategory::Grains),
                ing("Eggs", "4", "pcs", IngredientCategory::Proteins),
                ing("Onion", "1", "pcs", IngredientCategory::Vegetables),
                ing("Tomatoes", "2", "pcs", IngredientCategory::Vegetables),
                ing("Coriander powder", "10", "g", IngredientCategory::Spices),
                ing("Oil", "20", "ml", IngredientCategory::Oils),
            ],
        ),
        (
            "Curd Rice with Pickle",
            "Creamy curd rice with a small portion of mango pickle - a kid favorite",
            vec![
                ing("Raw rice", "300", "g", IngredientCategory::Grains),
                ing("Curd / Yogurt", "300", "ml", IngredientCategory::Dairy),
                ing("Milk", "100", "ml", IngredientCategory::Dairy),
                ing("Mango pickle", "20", "g", IngredientCategory::Spices),
                ing("Mustard seeds", "5", "g", IngredientCategory::Spices),
                ing("Curry leaves", "1", "sprig", IngredientCategory::Vegetables),
            ],
        ),
        (
            "Mini Idlis with Ghee & Sambar",
            "Bite-size idlis drizzled with ghee in sambar - perfect for young kids",
            vec![
                ing("Idli batter (rice + urad dal)", "600", "g", IngredientCategory::Grains),
                ing("Ghee", "30", "ml", IngredientCategory::Oils),
                ing("Toor dal (for sambar)", "80", "g", IngredientCategory::Grains),
                ing("Tomatoes", "2", "pcs", IngredientCategory::Vegetables),
                ing("Onion", "1", "pcs", IngredientCategory::Vegetables),
            ],
        ),
        (
            "Vegetable Khichdi",
            "Soft rice-lentil porridge with mixed vegetables, easy to digest for all ages",
            vec![
                ing("Raw rice", "200", "g", IngredientCategory::Grains),
                ing("Moong dal (yellow)", "100", "g", IngredientCategory::Grains),
                ing("Carrot", "1", "pcs", IngredientCategory::Vegetables),
                ing("Potato", "1", "pcs", IngredientCategory::Vegetables),
                ing("Beans", "50", "g", IngredientCategory::Vegetables),
                ing("Ghee", "20", "ml", IngredientCategory::Oils),
                ing("Turmeric powder", "3", "g", IngredientCategory::Spices),
            ],
        ),
        (
            "Chapathi with Dal & Vegetables",
            "Soft wheat rotis with mild lentil curry and vegetables",
            vec![
                ing("Whole wheat flour (atta)", "250", "g", IngredientCategory::Grains),
                ing("Moong dal (yellow)", "100", "g", IngredientCategory::Grains),
                ing("Spinach", "100", "g", IngredientCategory::Vegetables),
                ing("Carrot", "1", "pcs", IngredientCategory::Vegetables),
                ing("Ghee", "20", "ml", IngredientCategory::Oils),
                ing("Cumin seeds", "5", "g", IngredientCategory::Spices),
            ],
        ),
        (
            "Rice + Fish Curry (mild, deboned)",
            "Steamed rice with mild fish curry using deboned fish pieces - rich in omega-3",
            vec![
                ing("Raw rice", "300", "g", IngredientCategory::Grains),
                ing("Fish (Tilapia / Rohu, cleaned)", "400", "g", IngredientCategory::Proteins),
                ing("Coconut milk", "200", "ml", IngredientCategory::Dairy),
                ing("Onion", "1", "pcs", IngredientCategory::Vegetables),
                ing("Tomatoes", "2", "pcs", IngredientCategory::Vegetables),
                ing("Turmeric powder", "3", "g", IngredientCategory::Spices),
                ing("Coriander powder", "10", "g", IngredientCategory::Spices),
                ing("Oil", "25", "ml", IngredientCategory::Oils),
            ],
        ),
        (
            "Tomato Rice with Raita",
            "Tangy tomato rice with a cooling cucumber raita - nutritious and easy",
            vec![
                ing("Raw rice", "300", "g", IngredientCategory::Grains),
                ing("Tomatoes", "4", "pcs", IngredientCategory::Vegetables),
                ing("Onion", "1", "pcs", IngredientCategory::Vegetables),
                ing("Curd / Yogurt", "200", "ml", IngredientCategory::Dairy),
                ing("Cucumber", "1", "pcs", IngredientCategory::Vegetables),
                ing("Oil", "20", "ml", IngredientCategory::Oils),
                ing("Mustard seeds", "5", "g", IngredientCategory::Spices),
            ],
        ),
    ];

    let idx = day_offset as usize % lunches.len();
    let (name, desc, ingredients) = lunches[idx].clone();

    Meal {
        meal_type: MealType::KidsLunch,
        name: name.to_string(),
        description: desc.to_string(),
        ingredients,
        suitable_for: vec!["kids".to_string(), "infant".to_string()],
    }
}

// ── Dinner helpers ────────────────────────────────────────────────────────────

fn sunday_nonveg_dinner() -> Meal {
    Meal {
        meal_type: MealType::Dinner,
        name: "Mutton Biryani with Raita & Boiled Egg".to_string(),
        description: "Sunday special! Aromatic Chettinad-style mutton biryani served with cooling raita and boiled egg"
            .to_string(),
        ingredients: vec![
            ing("Basmati rice", "600", "g", IngredientCategory::Grains),
            ing("Mutton (bone-in)", "800", "g", IngredientCategory::Proteins),
            ing("Onion", "4", "pcs", IngredientCategory::Vegetables),
            ing("Tomatoes", "3", "pcs", IngredientCategory::Vegetables),
            ing("Ginger-garlic paste", "50", "g", IngredientCategory::Spices),
            ing("Curd / Yogurt", "200", "ml", IngredientCategory::Dairy),
            ing("Biryani masala", "30", "g", IngredientCategory::Spices),
            ing("Bay leaves", "3", "pcs", IngredientCategory::Spices),
            ing("Cinnamon sticks", "2", "pcs", IngredientCategory::Spices),
            ing("Cardamom pods", "4", "pcs", IngredientCategory::Spices),
            ing("Cloves", "4", "pcs", IngredientCategory::Spices),
            ing("Mint leaves", "1", "bunch", IngredientCategory::Vegetables),
            ing("Coriander leaves", "1", "bunch", IngredientCategory::Vegetables),
            ing("Eggs", "6", "pcs", IngredientCategory::Proteins),
            ing("Coconut milk", "200", "ml", IngredientCategory::Dairy),
            ing("Ghee", "50", "ml", IngredientCategory::Oils),
            ing("Saffron", "0.5", "g", IngredientCategory::Spices),
            ing("Cucumber", "2", "pcs", IngredientCategory::Vegetables),
        ],
        suitable_for: vec!["adults".to_string(), "kids".to_string()],
    }
}

fn chicken_dinner() -> Meal {
    Meal {
        meal_type: MealType::Dinner,
        name: "Chicken Chettinad Curry with Rice & Rasam".to_string(),
        description: "Aromatic mid-week chicken Chettinad curry with steamed rice and pepper rasam"
            .to_string(),
        ingredients: vec![
            ing("Chicken (curry cut)", "800", "g", IngredientCategory::Proteins),
            ing("Raw rice", "500", "g", IngredientCategory::Grains),
            ing("Onion", "3", "pcs", IngredientCategory::Vegetables),
            ing("Tomatoes", "3", "pcs", IngredientCategory::Vegetables),
            ing("Ginger-garlic paste", "40", "g", IngredientCategory::Spices),
            ing("Chettinad masala", "30", "g", IngredientCategory::Spices),
            ing("Kalpasi (stone flower)", "3", "g", IngredientCategory::Spices),
            ing("Star anise", "2", "pcs", IngredientCategory::Spices),
            ing("Coconut (grated)", "100", "g", IngredientCategory::Other),
            ing("Curry leaves", "3", "sprig", IngredientCategory::Vegetables),
            ing("Oil", "50", "ml", IngredientCategory::Oils),
            ing("Toor dal (for rasam)", "50", "g", IngredientCategory::Grains),
            ing("Tamarind", "20", "g", IngredientCategory::Spices),
            ing("Pepper", "10", "g", IngredientCategory::Spices),
            ing("Cumin seeds", "10", "g", IngredientCategory::Spices),
        ],
        suitable_for: vec!["adults".to_string(), "kids".to_string()],
    }
}

fn mushroom_dinner() -> Meal {
    Meal {
        meal_type: MealType::Dinner,
        name: "Mushroom Masala with Chapathi & Rice".to_string(),
        description:
            "Earthy mushroom masala in a rich tomato-onion gravy, served with chapathi and rice"
                .to_string(),
        ingredients: vec![
            ing("Button mushrooms", "500", "g", IngredientCategory::Vegetables),
            ing("Onion", "3", "pcs", IngredientCategory::Vegetables),
            ing("Tomatoes", "3", "pcs", IngredientCategory::Vegetables),
            ing("Ginger-garlic paste", "30", "g", IngredientCategory::Spices),
            ing("Coriander powder", "15", "g", IngredientCategory::Spices),
            ing("Cumin powder", "10", "g", IngredientCategory::Spices),
            ing("Garam masala", "10", "g", IngredientCategory::Spices),
            ing("Chilli powder", "8", "g", IngredientCategory::Spices),
            ing("Turmeric powder", "3", "g", IngredientCategory::Spices),
            ing("Coriander leaves", "1", "bunch", IngredientCategory::Vegetables),
            ing("Oil", "40", "ml", IngredientCategory::Oils),
            ing("Whole wheat flour (atta)", "300", "g", IngredientCategory::Grains),
            ing("Raw rice", "300", "g", IngredientCategory::Grains),
        ],
        suitable_for: vec!["adults".to_string(), "kids".to_string()],
    }
}

fn regular_dinner(day_offset: u64) -> Meal {
    let dinners = [
        (
            "Fish Curry with Rice & Papad",
            "Traditional Chennai fish curry (meen kulambu) with steamed rice and crispy papad",
            vec![
                ing("Fish (Seer fish / Vanjaram)", "600", "g", IngredientCategory::Proteins),
                ing("Raw rice", "500", "g", IngredientCategory::Grains),
                ing("Onion", "2", "pcs", IngredientCategory::Vegetables),
                ing("Tomatoes", "3", "pcs", IngredientCategory::Vegetables),
                ing("Tamarind", "30", "g", IngredientCategory::Spices),
                ing("Chilli powder", "15", "g", IngredientCategory::Spices),
                ing("Coriander powder", "15", "g", IngredientCategory::Spices),
                ing("Turmeric powder", "5", "g", IngredientCategory::Spices),
                ing("Fenugreek seeds", "5", "g", IngredientCategory::Spices),
                ing("Coconut (grated)", "100", "g", IngredientCategory::Other),
                ing("Mustard seeds", "5", "g", IngredientCategory::Spices),
                ing("Curry leaves", "3", "sprig", IngredientCategory::Vegetables),
                ing("Sesame oil (nallennai)", "40", "ml", IngredientCategory::Oils),
                ing("Papad", "6", "pcs", IngredientCategory::Other),
            ],
        ),
        (
            "Sambar Rice + Kootu + Poriyal",
            "Comforting one-pot sambar rice with vegetable kootu and stir-fry poriyal",
            vec![
                ing("Raw rice", "500", "g", IngredientCategory::Grains),
                ing("Toor dal (for sambar)", "150", "g", IngredientCategory::Grains),
                ing("Drumstick (Murungakkai)", "2", "pcs", IngredientCategory::Vegetables),
                ing("Eggplant / Brinjal", "200", "g", IngredientCategory::Vegetables),
                ing("Carrot", "2", "pcs", IngredientCategory::Vegetables),
                ing("Cabbage", "200", "g", IngredientCategory::Vegetables),
                ing("Chana dal", "50", "g", IngredientCategory::Grains),
                ing("Coconut (grated)", "100", "g", IngredientCategory::Other),
                ing("Tamarind", "25", "g", IngredientCategory::Spices),
                ing("Sambar powder", "25", "g", IngredientCategory::Spices),
                ing("Mustard seeds", "10", "g", IngredientCategory::Spices),
                ing("Curry leaves", "3", "sprig", IngredientCategory::Vegetables),
                ing("Oil", "40", "ml", IngredientCategory::Oils),
            ],
        ),
        (
            "Egg Masala Curry with Rice & Rasam",
            "Boiled eggs in spicy masala gravy with steamed rice and pepper-cumin rasam",
            vec![
                ing("Eggs", "8", "pcs", IngredientCategory::Proteins),
                ing("Raw rice", "500", "g", IngredientCategory::Grains),
                ing("Onion", "3", "pcs", IngredientCategory::Vegetables),
                ing("Tomatoes", "3", "pcs", IngredientCategory::Vegetables),
                ing("Ginger-garlic paste", "30", "g", IngredientCategory::Spices),
                ing("Chilli powder", "12", "g", IngredientCategory::Spices),
                ing("Coriander powder", "12", "g", IngredientCategory::Spices),
                ing("Toor dal (for rasam)", "50", "g", IngredientCategory::Grains),
                ing("Tamarind", "20", "g", IngredientCategory::Spices),
                ing("Pepper", "8", "g", IngredientCategory::Spices),
                ing("Oil", "40", "ml", IngredientCategory::Oils),
                ing("Curry leaves", "2", "sprig", IngredientCategory::Vegetables),
            ],
        ),
        (
            "Chettinad Prawns / Fish Fry with Rice & Keerai Masiyal",
            "Spicy Chettinad-style prawns or fish fry with steamed rice and spinach masiyal",
            vec![
                ing("Prawns / Fish fillet", "600", "g", IngredientCategory::Proteins),
                ing("Raw rice", "500", "g", IngredientCategory::Grains),
                ing("Spinach (keerai)", "300", "g", IngredientCategory::Vegetables),
                ing("Onion", "2", "pcs", IngredientCategory::Vegetables),
                ing("Chilli powder", "15", "g", IngredientCategory::Spices),
                ing("Turmeric powder", "5", "g", IngredientCategory::Spices),
                ing("Fennel seeds", "10", "g", IngredientCategory::Spices),
                ing("Garlic", "6", "pcs", IngredientCategory::Spices),
                ing("Sesame oil (nallennai)", "50", "ml", IngredientCategory::Oils),
                ing("Coconut (grated)", "50", "g", IngredientCategory::Other),
                ing("Toor dal", "50", "g", IngredientCategory::Grains),
            ],
        ),
        (
            "Chicken Kuzhambu with Rice & Beans Poriyal",
            "Country-style chicken kuzhambu with coconut milk base, rice and green beans stir-fry",
            vec![
                ing("Chicken (bone-in)", "700", "g", IngredientCategory::Proteins),
                ing("Raw rice", "500", "g", IngredientCategory::Grains),
                ing("Coconut milk", "300", "ml", IngredientCategory::Dairy),
                ing("Green beans", "200", "g", IngredientCategory::Vegetables),
                ing("Onion", "2", "pcs", IngredientCategory::Vegetables),
                ing("Tomatoes", "2", "pcs", IngredientCategory::Vegetables),
                ing("Chilli powder", "15", "g", IngredientCategory::Spices),
                ing("Coriander powder", "15", "g", IngredientCategory::Spices),
                ing("Coconut (grated)", "80", "g", IngredientCategory::Other),
                ing("Oil", "40", "ml", IngredientCategory::Oils),
                ing("Curry leaves", "3", "sprig", IngredientCategory::Vegetables),
                ing("Mustard seeds", "5", "g", IngredientCategory::Spices),
            ],
        ),
    ];

    let idx = day_offset as usize % dinners.len();
    let (name, desc, ingredients) = dinners[idx].clone();

    Meal {
        meal_type: MealType::Dinner,
        name: name.to_string(),
        description: desc.to_string(),
        ingredients,
        suitable_for: vec!["adults".to_string(), "kids".to_string()],
    }
}

// ── Utility ───────────────────────────────────────────────────────────────────

fn ing(name: &str, qty: &str, unit: &str, category: IngredientCategory) -> Ingredient {
    Ingredient {
        name: name.to_string(),
        quantity: qty.to_string(),
        unit: unit.to_string(),
        category,
    }
}

fn weekday_name(weekday: Weekday) -> String {
    match weekday {
        Weekday::Mon => "Monday".to_string(),
        Weekday::Tue => "Tuesday".to_string(),
        Weekday::Wed => "Wednesday".to_string(),
        Weekday::Thu => "Thursday".to_string(),
        Weekday::Fri => "Friday".to_string(),
        Weekday::Sat => "Saturday".to_string(),
        Weekday::Sun => "Sunday".to_string(),
    }
}
