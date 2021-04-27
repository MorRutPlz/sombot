#[macro_use]
mod logger;
mod config;
mod handler;
mod update;

use serenity::client::{bridge::gateway::GatewayIntents, Client};
use std::io::ErrorKind;

use crate::{config::Counter, handler::Handler};
use crate::{
    config::{Config, Discord, Role},
    update::update_loop,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = match std::fs::read_to_string("config.toml") {
        Ok(n) => match toml::from_str::<Config>(&n) {
            Ok(n) => n,
            Err(e) => panic!("failed to parse config.toml: {}", e),
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                let config = Config {
                    discord: Discord {
                        application_id: 836611210383720468,
                        token: "".to_string(),
                        guild_id: 836524537042042910,
                    },
                    counter: Counter {
                        total_member_id: 836542574050672640,
                    },
                    roles: vec![
                        Role {
                            role_id: 836526326479061003,
                            role_name: "Addu High School".to_string(),
                            counter_id: 836543036846112829,
                        },
                        Role {
                            role_id: 836526615336976445,
                            role_name: "Sharafuddin".to_string(),
                            counter_id: 836548271261351956,
                        },
                    ],
                };

                std::fs::write("config.toml", toml::to_string_pretty(&config).unwrap()).unwrap();

                config
            }
            _ => panic!("failed to read config.toml: {}", e),
        },
    };

    let mut client = Client::builder(&config.discord.token)
        .event_handler(Handler)
        .intents(GatewayIntents::all())
        .application_id(config.discord.application_id)
        .await
        .expect("Error creating client");

    update_loop(config, client.cache_and_http.http.clone());

    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}