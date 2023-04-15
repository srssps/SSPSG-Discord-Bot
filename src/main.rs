use commands::command;
use poise::serenity_prelude as serenity;

mod commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {} // Accessible in command functions

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("File .env not found");

    let token = std::env::var("DISCORD_BOT_TOKEN").expect("Discord Bot token not found");

    let intents = serenity::GatewayIntents::non_privileged();
    let commands = vec![command::command()];

    let framework = poise::Framework::builder()
        .token(token)
        .intents(intents)
        .options(poise::FrameworkOptions {
            commands: (commands),
            ..Default::default()
        })
        .setup(|context, _ready, framework| {
            Box::pin(async move {
                // Data construct
                poise::builtins::register_globally(context, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    match framework.run().await {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
