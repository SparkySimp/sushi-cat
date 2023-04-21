// commands.rs - Command handling for the bot

// import modules required for slash command handling and bot actions

use serenity::{
    async_trait,
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::{
        channel::Message,
        interactions::{
            application_command::{
                ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
            },
            Interaction,
        },
    },
}; 

// slash command that returns the bot's ping to the user
#[command]
#[description = "Returns the bot's ping to the user"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    // respond with the bot's gateway latency
    msg.channel_id.say(&ctx.http, format!("Pong! {}ms", ctx.cache.current_user().await.unwrap().ping(&ctx.http).await.unwrap().as_millis())).await?;
    Ok(())
}

// mutes someone and makes the category (1095394322456784916) the only channel set they can see
// only works in the "Off Topic Gods" server (1038474639359348746)
// adds the role "Jailed"(1095367228762898472)

#[command]
#[description = "Mutes someone and makes the \"Full-Wave rektifier\" the only channel set they can see"]
#[only_in("guilds")]
#[required_permissions(MUTE_MEMBERS)]
// specifies the only server it is allowed to be used in
#[only_in("1038474639359348746")]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    // gets the user to be muted from the message
    let user = &msg.mentions[0];
    // gets the guild the message was sent in
    let guild = msg.guild(&ctx.cache).await.unwrap();
    // gets the role "Jailed" from the guild
    let role = guild.role_by_name("Jailed").unwrap();
    // gets the member to be muted from the guild
    let member = guild.member(&ctx.http, user.id).await.unwrap();
    // adds the role "Jailed" to the member
    member.add_role(&ctx.http, role.id).await?;
    // responds with a message saying the user has been muted
    msg.channel_id.say(&ctx.http, format!("{} has been muted", user.name)).await?;
    Ok(())
}