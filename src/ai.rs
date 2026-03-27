use crate::config::Config;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

pub struct CopilotClient {
    client: reqwest::Client,
    config: Config,
}

impl CopilotClient {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::new();
        CopilotClient { client, config }
    }

    pub async fn generate_weekly_plan(&self, prompt: &str) -> Result<String> {
        let token = self
            .config
            .get_token()
            .ok_or_else(|| anyhow!("GitHub token not configured. Run `food-planner config --token <YOUR_GITHUB_TOKEN>` or set the GITHUB_TOKEN environment variable."))?;

        let system_prompt = self.build_system_prompt();

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ];

        let request = ChatRequest {
            model: self.config.ai.model.clone(),
            messages,
            temperature: 0.7,
            max_tokens: 4096,
        };

        let url = format!("{}/chat/completions", self.config.ai.copilot_api_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .header("Copilot-Integration-Id", "food-planner-cli")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!(
                "API request failed with status {}: {}",
                status,
                body
            ));
        }

        let chat_response: ChatResponse = response.json().await?;
        chat_response
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| anyhow!("No response from AI"))
    }

    fn build_system_prompt(&self) -> String {
        let family = &self.config.family;
        format!(
            r#"You are a specialized South Indian nutritionist and meal planner assistant for a Chennai-based developer family.

Family Profile:
- {} adults (ages 30-37 years)
- {} kids (ages 7-10 years)  
- {} infant (age 2 years)
- Cuisine: {} ({}), specifically Chennai style

Dietary Preferences & Rules:
- Protein sources allowed: {}
- Diet focus: balanced nutrition appropriate for each age group
- Breakfast: South Indian breakfast for everyone
- Saturday morning: Pongal or Poori (traditional Saturday special)
- Sunday: Non-vegetarian special dinner
- Mid-week (Wednesday/Thursday): Chicken dinner
- Mushroom dish: Include once per week
- Kids' snacks: Mostly seasonal fruits
- Kids' lunch: Nutritious, child-friendly South Indian meals
- Dinner: South Indian style for the whole family
- Infant (2yr): Soft, easily digestible foods adapted from family meals

Meal Planning Guidelines:
- Use traditional Chennai/Tamil Nadu recipes
- Include a variety of dal/sambar, rice dishes, and curries
- Balance between vegetarian and non-vegetarian meals
- Include traditional breakfast items: idli, dosa, upma, pongal, poori, etc.
- Use seasonal vegetables and local ingredients
- Ensure adequate protein, calcium (dairy for kids), iron, and fiber

Output Format:
When generating a weekly plan, provide a structured JSON response with days of the week, each containing:
- breakfast: meal name and ingredients
- kids_snack: fruit-based snack
- kids_lunch: child-friendly meal
- dinner: family dinner
- special_notes: any special preparation notes

Always end with a consolidated shopping list grouped by category."#,
            family.adults_count,
            family.kids_count,
            family.infant_count,
            family.cuisine,
            family.region,
            family.protein_preferences.join(", ")
        )
    }
}
