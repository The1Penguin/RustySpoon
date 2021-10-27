use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

#[command]
async fn temp (ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    Ok(())
}
