mod ai;
mod config;
mod display;
mod models;
mod planner;

use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate, Weekday};
use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "food-planner",
    about = "🍛 South Indian Weekly Food Planner for Dev Families\n   Powered by GitHub Copilot AI",
    version,
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a weekly food plan (uses AI if configured, else built-in template)
    Generate {
        /// Start date for the week (YYYY-MM-DD, defaults to next Monday)
        #[arg(short, long)]
        date: Option<String>,

        /// Use the built-in template instead of AI (offline mode)
        #[arg(long, default_value_t = false)]
        offline: bool,

        /// Output format: pretty (default) or json
        #[arg(short, long, default_value = "pretty")]
        output: String,
    },
    /// Show the shopping list for the upcoming week
    Shopping {
        /// Start date for the week (YYYY-MM-DD, defaults to next Monday)
        #[arg(short, long)]
        date: Option<String>,

        /// Output format: pretty (default) or json
        #[arg(short, long, default_value = "pretty")]
        output: String,
    },
    /// Configure GitHub Copilot API access
    Config {
        /// Your GitHub personal access token (with Copilot access)
        #[arg(long)]
        token: Option<String>,

        /// GitHub Copilot API URL (default: https://api.githubcopilot.com)
        #[arg(long)]
        api_url: Option<String>,

        /// AI model to use (default: gpt-4o)
        #[arg(long)]
        model: Option<String>,

        /// Show current configuration (without revealing the token)
        #[arg(long, default_value_t = false)]
        show: bool,
    },
    /// Show meal rules and family profile
    Profile,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            date,
            offline,
            output,
        } => {
            let week_start = parse_week_start(date)?;
            let config = config::Config::load()?;

            println!(
                "\n{} {}",
                "🍛 Food Planner".bright_green().bold(),
                "— Generating weekly meal plan...".bright_black()
            );

            let plan = if offline || config.get_token().is_none() {
                if !offline && config.get_token().is_none() {
                    println!(
                        "{}",
                        "  ℹ  No GitHub token configured. Using built-in South Indian templates."
                            .yellow()
                    );
                    println!(
                        "{}",
                        "     Run `food-planner config --token <TOKEN>` to enable AI generation."
                            .bright_black()
                    );
                    println!();
                }
                planner::MealPlanner::generate_offline_plan(week_start)
            } else {
                println!(
                    "{}",
                    "  🤖 Using GitHub Copilot AI to generate your personalised plan..."
                        .bright_cyan()
                );
                match generate_ai_plan(&config, week_start).await {
                    Ok(plan) => plan,
                    Err(e) => {
                        eprintln!(
                            "{} {}",
                            "  ⚠  AI generation failed:".yellow(),
                            e.to_string().bright_black()
                        );
                        eprintln!(
                            "{}",
                            "  ↳  Falling back to built-in template plan.".yellow()
                        );
                        println!();
                        planner::MealPlanner::generate_offline_plan(week_start)
                    }
                }
            };

            match output.as_str() {
                "json" => display::print_json(&plan)?,
                _ => display::print_weekly_plan(&plan),
            }
        }

        Commands::Shopping { date, output } => {
            let week_start = parse_week_start(date)?;
            let config = config::Config::load()?;

            println!(
                "\n{} {}",
                "🛒 Food Planner".bright_green().bold(),
                "— Building shopping list...".bright_black()
            );

            let plan = planner::MealPlanner::generate_offline_plan(week_start);

            match output.as_str() {
                "json" => {
                    let json = serde_json::to_string_pretty(&plan.shopping_list)?;
                    println!("{}", json);
                }
                _ => {
                    let _ = config;
                    display::print_shopping_list(&plan);
                }
            }
        }

        Commands::Config {
            token,
            api_url,
            model,
            show,
        } => {
            let mut cfg = config::Config::load()?;

            if show {
                print_config(&cfg);
                return Ok(());
            }

            let mut changed = false;

            if let Some(t) = token {
                cfg.ai.github_token = Some(t);
                changed = true;
                println!("{}", "  ✓  GitHub token saved.".bright_green());
            }
            if let Some(url) = api_url {
                cfg.ai.copilot_api_url = url;
                changed = true;
                println!("{}", "  ✓  API URL updated.".bright_green());
            }
            if let Some(m) = model {
                cfg.ai.model = m;
                changed = true;
                println!("{}", "  ✓  Model updated.".bright_green());
            }

            if changed {
                cfg.save()?;
            } else {
                print_config_help();
            }
        }

        Commands::Profile => {
            print_family_profile();
        }
    }

    Ok(())
}

async fn generate_ai_plan(
    config: &config::Config,
    week_start: NaiveDate,
) -> Result<models::WeeklyPlan> {
    let client = ai::CopilotClient::new(config.clone());

    let prompt = format!(
        r#"Generate a complete weekly South Indian (Chennai-style) food plan for the week starting {week_start}.

The plan should follow these rules exactly:
1. Saturday breakfast: Pongal OR Poori (alternate each week)
2. Sunday dinner: Mutton/Chicken Biryani (non-vegetarian special)
3. Wednesday OR Thursday dinner: Chicken curry/dish
4. Tuesday: Include a mushroom dish (dinner)
5. All breakfasts: South Indian style (idli, dosa, upma, idiyappam, pongal, etc.)
6. Kids' snacks: Fresh fruits only (2 portions per child per day)
7. Kids' lunch: Child-friendly South Indian meal, mildly spiced
8. Dinner: South Indian family dinner

Return the response as valid JSON matching this structure:
{{
  "week_start": "{week_start}",
  "days": [
    {{
      "date": "YYYY-MM-DD",
      "day_name": "Monday",
      "meals": [
        {{
          "meal_type": "Breakfast",
          "name": "meal name",
          "description": "brief description",
          "ingredients": [
            {{"name": "ingredient", "quantity": "100", "unit": "g", "category": "Grains"}}
          ],
          "suitable_for": ["adults", "kids"]
        }}
      ]
    }}
  ],
  "shopping_list": []
}}"#,
        week_start = week_start
    );

    let response = client.generate_weekly_plan(&prompt).await?;

    // Try to parse the AI response as JSON
    // Extract JSON block from response if wrapped in markdown
    let json_str = extract_json(&response);
    let plan: models::WeeklyPlan = serde_json::from_str(&json_str)?;
    Ok(plan)
}

fn extract_json(response: &str) -> String {
    // Look for JSON code block
    if let Some(start) = response.find("```json") {
        let after = &response[start + 7..];
        if let Some(end) = after.find("```") {
            return after[..end].trim().to_string();
        }
    }
    // Look for raw JSON object
    if let Some(start) = response.find('{') {
        if let Some(end) = response.rfind('}') {
            if end > start {
                return response[start..=end].to_string();
            }
        }
    }
    response.to_string()
}

fn parse_week_start(date_str: Option<String>) -> Result<NaiveDate> {
    match date_str {
        Some(s) => NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .map_err(|_| anyhow::anyhow!("Invalid date format. Use YYYY-MM-DD (e.g. 2024-03-25)")),
        None => {
            // Default to next Monday
            let today = Local::now().date_naive();
            let days_until_monday = match today.weekday() {
                Weekday::Mon => 0,
                Weekday::Tue => 6,
                Weekday::Wed => 5,
                Weekday::Thu => 4,
                Weekday::Fri => 3,
                Weekday::Sat => 2,
                Weekday::Sun => 1,
            };
            Ok(today + chrono::Duration::days(days_until_monday))
        }
    }
}

fn print_config(cfg: &config::Config) {
    println!("\n{}", "⚙️  Current Configuration".bright_white().bold());
    println!("{}", "─".repeat(50));
    println!("  AI Settings:");
    println!(
        "    API URL  : {}",
        cfg.ai.copilot_api_url.bright_cyan()
    );
    println!("    Model    : {}", cfg.ai.model.bright_cyan());
    let token_status = if cfg.ai.github_token.is_some() {
        "✓ Configured".bright_green().to_string()
    } else if std::env::var("GITHUB_TOKEN").is_ok() {
        "✓ From GITHUB_TOKEN env var".bright_green().to_string()
    } else {
        "✗ Not set (using offline templates)".yellow().to_string()
    };
    println!("    Token    : {}", token_status);
    println!();
    println!("  Family Settings:");
    println!("    Adults   : {} (ages 30-37)", cfg.family.adults_count);
    println!("    Kids     : {} (ages 7-10)", cfg.family.kids_count);
    println!("    Infant   : {} (age 2yr)", cfg.family.infant_count);
    println!("    Cuisine  : {} — {}", cfg.family.cuisine, cfg.family.region);
    println!(
        "    Proteins : {}",
        cfg.family.protein_preferences.join(", ")
    );
    println!();
}

fn print_config_help() {
    println!("\n{}", "⚙️  food-planner config".bright_white().bold());
    println!("{}", "─".repeat(50));
    println!("  Set your GitHub Copilot token:");
    println!(
        "    {}",
        "food-planner config --token <YOUR_GITHUB_TOKEN>".bright_cyan()
    );
    println!();
    println!("  Or set environment variable:");
    println!("    {}", "export GITHUB_TOKEN=<YOUR_GITHUB_TOKEN>".bright_cyan());
    println!();
    println!("  Show current config:");
    println!("    {}", "food-planner config --show".bright_cyan());
    println!();
    println!("  Change model:");
    println!(
        "    {}",
        "food-planner config --model gpt-4o".bright_cyan()
    );
    println!();
}

fn print_family_profile() {
    println!("\n{}", "👨‍👩‍👧‍👦  Family Profile & Meal Rules".bright_white().bold());
    println!("{}", "═".repeat(60));
    println!();
    println!("  {}", "Family Members:".bright_white().bold());
    println!("    👨‍💻 4 Adults      — ages 30-37 years");
    println!("    🧒 2 Kids        — ages 7-10 years");
    println!("    👶 1 Infant      — age 2 years");
    println!();
    println!("  {}", "Cuisine Style:".bright_white().bold());
    println!("    🌍 South Indian — Chennai / Tamil Nadu");
    println!("    🥩 Protein sources: egg, fish, chicken, mutton");
    println!();
    println!("  {}", "Weekly Meal Rules:".bright_white().bold());
    println!("    🌅 Breakfast     — South Indian style every day");
    println!("    🍚 Saturday      — Pongal OR Poori (special breakfast)");
    println!("    🥩 Sunday        — Non-veg special dinner (biryani/feast)");
    println!("    🍗 Mid-week      — Chicken dinner (Wed/Thu)");
    println!("    🍄 Mushroom      — Mushroom dish once a week (Tue)");
    println!("    🍎 Kids' Snacks  — Fresh fruits only");
    println!("    🎒 Kids' Lunch   — Child-friendly, mildly spiced");
    println!("    🌙 Dinner        — Full South Indian family dinner");
    println!("    👶 Infant meals  — Soft adaptations from family meals");
    println!();
    println!("  {}", "Nutrition Focus:".bright_white().bold());
    println!("    ⚖️  Balanced diet — proteins, carbs, fiber, vitamins");
    println!("    🥛 Dairy for kids — milk, curd, coconut milk");
    println!("    🐟 Fish omega-3  — twice a week");
    println!("    🌿 Seasonal veg  — local Chennai market produce");
    println!();
}
