use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::prelude::{Activity, OnlineStatus, Ready},
};

use crate::typemap::TypeMapConfig;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        info!("Ready event fired");

        let data = ctx.data.read().await;
        let config = data.get::<TypeMapConfig>().unwrap();

        let activity = Activity::playing(&config.discord.status);
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(activity), status).await;
    }
}
