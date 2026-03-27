/// Integration tests for the food-planner CLI.
///
/// These tests verify the core business logic:
/// - Meal plan generation for all 7 days
/// - Day-specific rules (Saturday pongal, Sunday non-veg, Wednesday chicken, Tuesday mushroom)
/// - Shopping list aggregation
/// - JSON serialization round-trips

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use food_planner::models::{MealType, IngredientCategory};
    use food_planner::planner::MealPlanner;

    /// Monday 2024-03-25 → generates a full 7-day plan
    fn week_start() -> NaiveDate {
        NaiveDate::from_ymd_opt(2024, 3, 25).unwrap()
    }

    #[test]
    fn plan_has_seven_days() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        assert_eq!(plan.days.len(), 7, "Weekly plan must have exactly 7 days");
    }

    #[test]
    fn each_day_has_four_meals() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        for day in &plan.days {
            assert_eq!(
                day.meals.len(),
                4,
                "{} should have 4 meals (breakfast, kids snack, kids lunch, dinner)",
                day.day_name
            );
        }
    }

    #[test]
    fn meal_types_are_correct_per_day() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        for day in &plan.days {
            let types: Vec<&MealType> = day.meals.iter().map(|m| &m.meal_type).collect();
            assert!(types.contains(&&MealType::Breakfast), "{} missing Breakfast", day.day_name);
            assert!(types.contains(&&MealType::KidsSnack), "{} missing KidsSnack", day.day_name);
            assert!(types.contains(&&MealType::KidsLunch), "{} missing KidsLunch", day.day_name);
            assert!(types.contains(&&MealType::Dinner), "{} missing Dinner", day.day_name);
        }
    }

    #[test]
    fn saturday_has_pongal_or_poori_breakfast() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        let saturday = plan.days.iter().find(|d| d.day_name == "Saturday").unwrap();
        let breakfast = saturday.meals.iter().find(|m| m.meal_type == MealType::Breakfast).unwrap();
        let name_lower = breakfast.name.to_lowercase();
        assert!(
            name_lower.contains("pongal") || name_lower.contains("poori"),
            "Saturday breakfast should be Pongal or Poori, got: {}",
            breakfast.name
        );
    }

    #[test]
    fn sunday_dinner_is_nonveg_special() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        let sunday = plan.days.iter().find(|d| d.day_name == "Sunday").unwrap();
        let dinner = sunday.meals.iter().find(|m| m.meal_type == MealType::Dinner).unwrap();
        let name_lower = dinner.name.to_lowercase();
        // Must contain mutton, biryani, or chicken (non-veg)
        assert!(
            name_lower.contains("mutton") || name_lower.contains("biryani") || name_lower.contains("chicken"),
            "Sunday dinner should be non-veg special, got: {}",
            dinner.name
        );
    }

    #[test]
    fn wednesday_dinner_is_chicken() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        let wednesday = plan.days.iter().find(|d| d.day_name == "Wednesday").unwrap();
        let dinner = wednesday.meals.iter().find(|m| m.meal_type == MealType::Dinner).unwrap();
        let name_lower = dinner.name.to_lowercase();
        assert!(
            name_lower.contains("chicken"),
            "Wednesday dinner should be chicken, got: {}",
            dinner.name
        );
    }

    #[test]
    fn tuesday_dinner_has_mushroom() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        let tuesday = plan.days.iter().find(|d| d.day_name == "Tuesday").unwrap();
        let dinner = tuesday.meals.iter().find(|m| m.meal_type == MealType::Dinner).unwrap();
        let name_lower = dinner.name.to_lowercase();
        assert!(
            name_lower.contains("mushroom"),
            "Tuesday dinner should include mushroom, got: {}",
            dinner.name
        );
    }

    #[test]
    fn kids_snack_contains_fruits() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        for day in &plan.days {
            let snack = day.meals.iter().find(|m| m.meal_type == MealType::KidsSnack).unwrap();
            let has_fruit = snack.ingredients.iter().any(|i| i.category == IngredientCategory::Fruits);
            assert!(
                has_fruit,
                "{} kids snack should contain fruits, got: {}",
                day.day_name,
                snack.name
            );
        }
    }

    #[test]
    fn shopping_list_is_non_empty() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        assert!(!plan.shopping_list.is_empty(), "Shopping list should not be empty");
    }

    #[test]
    fn shopping_list_contains_proteins() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        let has_protein = plan.shopping_list.iter().any(|i| i.category == IngredientCategory::Proteins);
        assert!(has_protein, "Shopping list must include protein items");
    }

    #[test]
    fn shopping_list_contains_grains() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        let has_grains = plan.shopping_list.iter().any(|i| i.category == IngredientCategory::Grains);
        assert!(has_grains, "Shopping list must include grain/staple items");
    }

    #[test]
    fn plan_serializes_to_json() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        let json = serde_json::to_string(&plan).expect("Plan should serialize to JSON");
        assert!(json.contains("week_start"), "JSON should contain week_start field");
        assert!(json.contains("days"), "JSON should contain days field");
        assert!(json.contains("shopping_list"), "JSON should contain shopping_list field");
    }

    #[test]
    fn week_start_date_is_correct() {
        let start = week_start();
        let plan = MealPlanner::generate_offline_plan(start);
        assert_eq!(plan.week_start, start);
        assert_eq!(plan.days[0].date, start);
    }

    #[test]
    fn all_days_have_nonempty_meal_names() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        for day in &plan.days {
            for meal in &day.meals {
                assert!(!meal.name.is_empty(), "{} has a meal with empty name", day.day_name);
                assert!(!meal.description.is_empty(), "{} has a meal with empty description", day.day_name);
            }
        }
    }

    #[test]
    fn all_meals_have_ingredients() {
        let plan = MealPlanner::generate_offline_plan(week_start());
        for day in &plan.days {
            for meal in &day.meals {
                assert!(
                    !meal.ingredients.is_empty(),
                    "{} — {} should have at least one ingredient",
                    day.day_name,
                    meal.name
                );
            }
        }
    }
}
