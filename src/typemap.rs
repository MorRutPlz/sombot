use serenity::prelude::TypeMapKey;

use crate::config::Config;

pub struct TypeMapConfig;

impl TypeMapKey for TypeMapConfig {
    type Value = Config;
}
