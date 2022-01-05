use serenity::{
    builder::{CreateEmbed, CreateMessage},
    client::Context,
    framework::standard::{CommandError, CommandResult},
    model::{channel::Message, id::ChannelId},
    utils::Color,
};
use std::{sync::Arc, time::Duration};

// Helper function for fast and easy embed messages
pub async fn embed_msg(
    ctx: &Context,
    channel_id: &ChannelId,
    title: Option<&str>,
    description: Option<&str>,
    color: Option<Color>,
    autodelete: Option<Duration>,
) -> CommandResult<Message> {
    let mut embed = CreateEmbed::default();

    match title {
        Some(title) => {
            embed.title(title);
        }
        None => (),
    }
    match description {
        Some(description) => {
            embed.description(description);
        }
        None => (),
    }
    match color {
        Some(color) => {
            embed.color(color);
        }
        None => (),
    }

    let msg = channel_id.send_message(ctx, |m| m.set_embed(embed)).await?;

    match autodelete {
        Some(duration) => {
            tokio::time::sleep(duration).await;
            msg.delete(ctx).await?;
        }
        None => (),
    }

    Ok(msg)
}

// Function for waiting for the author of msg to send a message
pub async fn wait_for_message(
    ctx: &Context,
    msg: &Message,
    timeout: Duration,
) -> CommandResult<Arc<Message>> {
    let message = match msg.author.await_reply(ctx).timeout(timeout).await {
        Some(msg) => msg,
        None => {
            embed_msg(
                ctx,
                &msg.channel_id,
                Some("Timeout!"),
                Some(&format!(
                    "No reply sent in {} minutes and {} seconds",
                    timeout.as_secs() / 60,
                    timeout.as_secs() % 60,
                )),
                Some(Color::RED),
                None,
            )
            .await?;
            return Err(CommandError::from(
                "No reply received for problem description",
            ));
        }
    };

    Ok(message)
}

// Helper function for asking for user input after a message from the bot
pub async fn get_message_reply<'a, F>(
    ctx: &Context,
    msg: &Message,
    question_msg_f: F,
    timeout: Duration,
) -> CommandResult<String>
where
    for<'b> F: FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>,
{
    // Ask the user first
    let question_msg = msg.channel_id.send_message(ctx, question_msg_f).await?;

    // Get the reply message
    // The loops are for making sure there is at least some text content in the message
    let msg = loop {
        let new_msg = match wait_for_message(ctx, msg, timeout).await {
            Ok(msg) => msg,
            Err(why) => {
                return Err(CommandError::from(format!(
                    "User took too long to respond: {}",
                    why
                )));
            }
        };
        if new_msg.content != "" {
            break new_msg;
        }
        embed_msg(
            ctx,
            &msg.channel_id,
            None,
            Some("Please send a message with text content."),
            Some(Color::RED),
            Some(Duration::from_secs(3)),
        )
        .await?;
    };

    // Get the message content
    let content = msg.content_safe(ctx).await;

    // Clean up messages
    match msg
        .channel_id
        .delete_messages(ctx, vec![question_msg.id, msg.id])
        .await
    {
        Ok(_) => (),
        Err(why) => log::error!("Error deleting messages: {}", why),
    }

    // Return content
    Ok(content)
}
