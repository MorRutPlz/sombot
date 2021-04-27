use serde_json::{json, Map};
use serenity::http::Http;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

use crate::config::Config;

pub fn update_loop(config: Config, http: Arc<Http>) {
    tokio::spawn(async move {
        loop {
            debug!("Sleeping for 15 seconds");

            sleep(Duration::from_secs(15)).await;

            let member_roles = match http
                .get_guild_members(config.discord.guild_id, None, None)
                .await
            {
                Ok(n) => n,
                Err(e) => {
                    error!(
                        "Error getting guild members for {}: {}",
                        config.discord.guild_id, e
                    );

                    continue;
                }
            };

            let member_roles = member_roles
                .into_iter()
                .filter(|x| !x.user.bot)
                .map(|x| x.roles.iter().map(|x| x.0).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let mut role_count = config
                .roles
                .iter()
                .map(|x| (x.role_id, (0, x.role_name.clone(), x.counter_id)))
                .collect::<HashMap<_, (usize, _, _)>>();

            for roles in member_roles.iter() {
                for key in config.roles.iter().map(|x| x.role_id) {
                    if roles.contains(&key) {
                        let (count, role_name, counter_id) =
                            role_count.get(&key).unwrap().to_owned();
                        role_count.insert(key, (count + 1, role_name, counter_id));
                    }
                }
            }

            debug!("Total member count: {}", member_roles.len());
            debug!(
                "Members with roles: {}",
                role_count
                    .iter()
                    .map(|(_, (count, _, _))| *count)
                    .sum::<usize>()
            );

            update_counter(
                &http,
                "Total Members".to_string(),
                config.counter.total_member_id,
                member_roles.len(),
            )
            .await;

            for (_, (count, name, channel)) in role_count.into_iter() {
                update_counter(&http, name, channel, count).await;
            }
        }
    });
}

async fn update_counter(http: &Arc<Http>, counter_name: String, counter_id: u64, count: usize) {
    let channel_name = format!("{}: {}", counter_name, count);

    match http
        .edit_channel(
            counter_id,
            &Map::from_iter(
                [("name".to_string(), json!(channel_name))]
                    .iter()
                    .map(|x| x.to_owned()),
            ),
        )
        .await
    {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to update counter channel name: {}", e);
        }
    }
}
