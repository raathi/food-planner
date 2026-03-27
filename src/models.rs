use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyProfile {
    pub adults: Vec<Adult>,
    pub kids: Vec<Kid>,
    pub infant: Option<Infant>,
    pub cuisine: String,
    pub region: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adult {
    pub age_range: String,
    pub count: u8,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kid {
    pub age_range: String,
    pub count: u8,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Infant {
    pub age_years: u8,
    pub count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MealType {
    Breakfast,
    KidsSnack,
    KidsLunch,
    Lunch,
    Dinner,
}

impl std::fmt::Display for MealType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MealType::Breakfast => write!(f, "Breakfast"),
            MealType::KidsSnack => write!(f, "Kids' Snack"),
            MealType::KidsLunch => write!(f, "Kids' Lunch"),
            MealType::Lunch => write!(f, "Lunch"),
            MealType::Dinner => write!(f, "Dinner"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meal {
    pub meal_type: MealType,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<Ingredient>,
    pub suitable_for: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: String,
    pub unit: String,
    pub category: IngredientCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IngredientCategory {
    Vegetables,
    Fruits,
    Grains,
    Proteins,
    Dairy,
    Spices,
    Oils,
    Other,
}

impl std::fmt::Display for IngredientCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IngredientCategory::Vegetables => write!(f, "Vegetables"),
            IngredientCategory::Fruits => write!(f, "Fruits"),
            IngredientCategory::Grains => write!(f, "Grains & Staples"),
            IngredientCategory::Proteins => write!(f, "Proteins (Meat/Eggs/Fish)"),
            IngredientCategory::Dairy => write!(f, "Dairy"),
            IngredientCategory::Spices => write!(f, "Spices & Condiments"),
            IngredientCategory::Oils => write!(f, "Oils & Fats"),
            IngredientCategory::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayPlan {
    pub date: NaiveDate,
    pub day_name: String,
    pub meals: Vec<Meal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyPlan {
    pub week_start: NaiveDate,
    pub days: Vec<DayPlan>,
    pub shopping_list: Vec<ShoppingItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoppingItem {
    pub ingredient: String,
    pub total_quantity: String,
    pub unit: String,
    pub category: IngredientCategory,
}

impl Default for FamilyProfile {
    fn default() -> Self {
        FamilyProfile {
            adults: vec![Adult {
                age_range: "30-37".to_string(),
                count: 4,
            }],
            kids: vec![Kid {
                age_range: "7-10".to_string(),
                count: 2,
            }],
            infant: Some(Infant {
                age_years: 2,
                count: 1,
            }),
            cuisine: "South Indian".to_string(),
            region: "Chennai".to_string(),
        }
    }
}
