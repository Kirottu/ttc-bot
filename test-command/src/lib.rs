use poise::Command;
use ttc_bot_common::types::{Context, Data, Error};
// ----------------------
// General group commands
// ----------------------

#[no_mangle]
fn setup() -> Command<Data, Error> {
    pong()
}

#[poise::command(slash_command)]
async fn pong(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;

    Ok::<(), Error>(())
}
