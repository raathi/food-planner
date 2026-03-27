use crate::models::*;
use colored::Colorize;
use std::collections::HashMap;

pub fn print_weekly_plan(plan: &WeeklyPlan) {
    println!();
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════════╗"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "║       🍛  WEEKLY SOUTH INDIAN FOOD PLAN  🍚                 ║"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        format!(
            "║  Family: 4 adults • 2 kids • 1 infant | Chennai Style       ║"
        )
        .bright_green()
        .bold()
    );
    println!(
        "{}",
        format!(
            "║  Week of: {:<51}║",
            plan.week_start.format("%d %B %Y")
        )
        .bright_green()
        .bold()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════════╝"
            .bright_green()
            .bold()
    );
    println!();

    for day in &plan.days {
        print_day_plan(day);
    }
}

pub fn print_day_plan(day: &DayPlan) {
    let day_header = format!(
        "  {} {}  {}  ",
        day.day_name.to_uppercase(),
        day.date.format("%d %b"),
        day_emoji(&day.day_name)
    );
    println!("{}", format!("┌─{}─┐", "─".repeat(day_header.len())).cyan());
    println!("{}", format!("│ {} │", day_header).cyan().bold());
    println!("{}", format!("└─{}─┘", "─".repeat(day_header.len())).cyan());
    println!();

    for meal in &day.meals {
        print_meal(meal);
    }
    println!();
}

fn print_meal(meal: &Meal) {
    let (emoji, color_fn): (&str, Box<dyn Fn(&str) -> colored::ColoredString>) =
        match meal.meal_type {
            MealType::Breakfast => ("🌅", Box::new(|s: &str| s.yellow().bold())),
            MealType::KidsSnack => ("🍎", Box::new(|s: &str| s.green().bold())),
            MealType::KidsLunch => ("🎒", Box::new(|s: &str| s.bright_cyan().bold())),
            MealType::Lunch => ("🍽️", Box::new(|s: &str| s.bright_white().bold())),
            MealType::Dinner => ("🌙", Box::new(|s: &str| s.magenta().bold())),
        };

    println!(
        "  {} {}  {}",
        emoji,
        color_fn(&meal.meal_type.to_string()),
        meal.name.white().bold()
    );
    println!("     {}", meal.description.bright_black());
    println!();
}

pub fn print_shopping_list(plan: &WeeklyPlan) {
    println!();
    println!("{}", "🛒  WEEKLY SHOPPING LIST".bright_yellow().bold());
    println!("{}", "═".repeat(60).bright_yellow());
    println!(
        "{}",
        format!("  Week of: {}", plan.week_start.format("%d %B %Y")).bright_black()
    );
    println!();

    // Group by category
    let mut by_category: HashMap<String, Vec<&ShoppingItem>> = HashMap::new();
    for item in &plan.shopping_list {
        by_category
            .entry(format!("{}", item.category))
            .or_default()
            .push(item);
    }

    let category_order = [
        "Grains & Staples",
        "Proteins (Meat/Eggs/Fish)",
        "Vegetables",
        "Fruits",
        "Dairy",
        "Spices & Condiments",
        "Oils & Fats",
        "Other",
    ];

    for category_name in &category_order {
        if let Some(items) = by_category.get(*category_name) {
            println!(
                "  {}",
                format!("📦 {}", category_name).bright_white().bold()
            );
            println!("  {}", "─".repeat(50));
            for item in items {
                println!(
                    "    {} {:>8} {:<10}  {}",
                    "▸".bright_green(),
                    item.total_quantity.bright_yellow(),
                    item.unit.bright_black(),
                    item.ingredient.white()
                );
            }
            println!();
        }
    }

    // Print any categories not in our predefined order
    for (category_name, items) in &by_category {
        if !category_order.contains(&category_name.as_str()) {
            println!(
                "  {}",
                format!("📦 {}", category_name).bright_white().bold()
            );
            println!("  {}", "─".repeat(50));
            for item in items {
                println!(
                    "    {} {:>8} {:<10}  {}",
                    "▸".bright_green(),
                    item.total_quantity.bright_yellow(),
                    item.unit.bright_black(),
                    item.ingredient.white()
                );
            }
            println!();
        }
    }

    println!("{}", "═".repeat(60).bright_yellow());
    println!(
        "  {} {}",
        "Total items:".bright_black(),
        plan.shopping_list.len().to_string().bright_yellow().bold()
    );
    println!();
}

pub fn print_json(plan: &WeeklyPlan) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(plan)?;
    println!("{}", json);
    Ok(())
}

fn day_emoji(day_name: &str) -> &'static str {
    match day_name {
        "Monday" => "💻",
        "Tuesday" => "☕",
        "Wednesday" => "🍗",
        "Thursday" => "🌿",
        "Friday" => "🎉",
        "Saturday" => "🍚",
        "Sunday" => "🥩",
        _ => "📅",
    }
}
