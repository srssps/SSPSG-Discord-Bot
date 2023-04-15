use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn command(context: Context<'_>) -> Result<(), Error> {
    context.say("Command").await?;
    Ok(())
}
