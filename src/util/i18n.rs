use std::{collections::HashMap, sync::Arc};

use fluent_templates::{fluent_bundle::FluentValue, static_loader, Loader};
use serenity::{
    client::Context,
    model::id::GuildId,
    prelude::{Mutex, TypeMapKey},
};
use unic_langid::{langid, LanguageIdentifier};

use crate::Database;

const ENGLISH: LanguageIdentifier = langid!("en");

static_loader! {
    pub static LOCALES = {
        locales: "./lang",
        fallback_language: "en",
        customise: |bundle| bundle.set_use_isolating(false),
        core_locales: "./lang/core.ftl",
    };
}

/// A prettier way to construct a hash map of args for Fluent
#[macro_export]
macro_rules! i18n_args {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            use std::collections::HashMap;
            use fluent_templates::fluent_bundle::FluentValue;

            let mut map: HashMap<String, FluentValue>  = HashMap::new();

            $(map.insert($key.into(), $value.into());)*

            map
        }
    }
}

pub struct I18n {
    db: Arc<Mutex<Database>>,
}

impl TypeMapKey for I18n {
    type Value = Arc<I18n>;
}

impl I18n {
    pub fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db }
    }

    pub async fn get(&self, guild_id: &Option<GuildId>, key: &str) -> String {
        let locale = self.get_guild_locale(guild_id).await;
        LOCALES.lookup(&locale, key)
    }

    pub async fn replace(
        &self,
        guild_id: &Option<GuildId>,
        key: &str,
        args: HashMap<String, FluentValue<'_>>,
    ) -> String {
        let locale = self.get_guild_locale(guild_id).await;

        LOCALES.lookup_with_args(&locale, &key, &args)
    }

    fn string_to_langid(&self, lang_id: &str) -> LanguageIdentifier {
        match lang_id.to_lowercase().as_str() {
            "en" => ENGLISH,
            _ => ENGLISH,
        }
    }

    async fn get_guild_locale(
        &self,
        guild_id: &Option<GuildId>,
    ) -> LanguageIdentifier {
        if let Some(locale) =
            self.db.lock().await.get_guild_locale(guild_id).await
        {
            self.string_to_langid(&locale)
        } else {
            ENGLISH
        }
    }

    pub async fn get_from_typemap(ctx: &Context) -> Arc<Self> {
        let data_read = ctx.data.read().await;
        data_read.get::<Self>().unwrap().clone()
    }
}
