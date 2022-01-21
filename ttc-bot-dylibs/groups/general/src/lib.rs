use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandGroup, CommandResult,
    },
    model::channel::Message,
};
use tokio_global::{AutoRuntime, Runtime};

#[no_mangle]
pub fn setup() -> &'static CommandGroup {
    let _guard = Runtime::default();
    let runner = std::thread::spawn(|| {
        Runtime::run();
    });

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
