use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandGroup, CommandResult,
    },
    model::channel::Message,
};

#[no_mangle]
pub fn get_group() -> &'static CommandGroup {
    return &GENERAL_GROUP;
}

#[group]
#[commands(ping)]
struct General;

// ----------------------
// General group commands
// ----------------------

#[command]
#[description("Ping!")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply_ping(ctx, "pong").await?;

    Ok(())
}
