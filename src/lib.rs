//! This library allow to interact with [TCGdex API](https://tcgdex.dev).
//!
//! You can get information about Pokémon cards, cards sets and sets series accessible through a powerful filter.
//! You can also get a list of all existing card types, categories, Hp, illustrators, retreat costs and rarities.

pub mod endpoints;
pub mod errors;
mod is_empty;
pub mod query;

use crate::endpoints::cards::CardApi;
use crate::endpoints::categories::CategoryApi;
use crate::endpoints::hps::HpApi;
use crate::endpoints::illustrators::IllustratorApi;
use crate::endpoints::rarities::RarityApi;
use crate::endpoints::retreats::RetreatApi;
use crate::endpoints::series::SerieApi;
use crate::endpoints::sets::SetApi;
use crate::endpoints::types::TypeApi;
use std::fmt::Display;

// Re-exports
pub use crate::endpoints::{
    cards::{Attack, Card, CardBrief, Variants, Weakness},
    series::{Serie, SerieBrief},
    sets::{Set, SetBrief},
};
pub use crate::query::Query;

/// Available langages for data.
#[derive(Debug)]
pub enum Lang {
    /// English
    EN,

    /// French
    FR,

    /// Deutsch
    DE,

    /// Italian
    IT,

    /// Portuguese
    PT,

    /// Spanish
    ES,
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Generic structure containing a unique REST client and selected langage.
///
/// Create only one instance and use it for all requests.
///
/// Langage can be changed. English is default value.
///
/// You have to get module interface before using a module.
#[derive(Debug)]
pub struct Tcgdex {
    client: reqwest::blocking::Client,
    lang: Lang,
}

impl Default for Tcgdex {
    fn default() -> Self {
        Self::new()
    }
}

impl Tcgdex {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            lang: Lang::EN,
        }
    }

    #[must_use]
    pub fn set_lang(&mut self, lang: Lang) -> &mut Self {
        self.lang = lang;
        self
    }

    /// Get an interface to types module.
    #[must_use]
    pub fn types(&self) -> TypeApi {
        TypeApi(&self.client, self.lang.to_string().to_lowercase())
    }

    /// Get an interface to categories module.
    #[must_use]
    pub fn categories(&self) -> CategoryApi {
        CategoryApi(&self.client, self.lang.to_string().to_lowercase())
    }

    /// Get an interface to hps module.
    #[must_use]
    pub fn hps(&self) -> HpApi {
        HpApi(&self.client, self.lang.to_string().to_lowercase())
    }

    /// Get an interface to illustrators module.
    #[must_use]
    pub fn illustrators(&self) -> IllustratorApi {
        IllustratorApi(&self.client, self.lang.to_string().to_lowercase())
    }

    /// Get an interface to rarities module.
    #[must_use]
    pub fn rarities(&self) -> RarityApi {
        RarityApi(&self.client, self.lang.to_string().to_lowercase())
    }

    /// Get an interface to retreats module.
    #[must_use]
    pub fn retreats(&self) -> RetreatApi {
        RetreatApi(&self.client, self.lang.to_string().to_lowercase())
    }

    /// Get an interface to retreats module.
    #[must_use]
    pub fn series(&self) -> SerieApi {
        SerieApi(&self.client, self.lang.to_string().to_lowercase())
    }

    /// Get an interface to retreats module.
    #[must_use]
    pub fn sets(&self) -> SetApi {
        SetApi(&self.client, self.lang.to_string().to_lowercase())
    }

    /// Get an interface to retreats module.
    #[must_use]
    pub fn cards(&self) -> CardApi {
        CardApi(&self.client, self.lang.to_string().to_lowercase())
    }
}
