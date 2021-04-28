use async_channel::Sender;
use serenity::prelude::TypeMapKey;

use crate::config::Config;

pub struct TypeMapConfig;
pub struct TypeMapSender;

impl TypeMapKey for TypeMapConfig {
    type Value = Config;
}

impl TypeMapKey for TypeMapSender {
    type Value = Sender<()>;
}
