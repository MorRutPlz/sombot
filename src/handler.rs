use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        guild::Member,
        id::GuildId,
        prelude::{Activity, OnlineStatus, Ready, User},
    },
};

use crate::typemap::{TypeMapConfig, TypeMapSender};

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

    async fn guild_member_addition(&self, ctx: Context, _: GuildId, mut member: Member) {
        info!(
            "A new member has joined: {}#{}",
            member.user.name, member.user.discriminator
        );

        let data = ctx.data.read().await;
        let config = data.get::<TypeMapConfig>().unwrap();
        let sender = data.get::<TypeMapSender>().unwrap();

        if !sender.is_full() {
            match sender.send(()).await {
                Ok(_) => {}
                Err(e) => error!("Failed to tell counter update loop to update: {}", e),
            }
        }

        match member
            .add_role(&ctx.http, config.sombot.people_role_id)
            .await
        {
            Ok(_) => {}
            Err(e) => error!("Failed to add new member to default role: {}", e),
        }
    }

    async fn guild_member_removal(&self, ctx: Context, _: GuildId, user: User, _: Option<Member>) {
        info!("Member left: {}#{}", user.name, user.discriminator);

        let data = ctx.data.read().await;
        let sender = data.get::<TypeMapSender>().unwrap();

        if !sender.is_full() {
            match sender.send(()).await {
                Ok(_) => {}
                Err(e) => error!("Failed to tell counter update loop to update: {}", e),
            }
        }
    }
}
