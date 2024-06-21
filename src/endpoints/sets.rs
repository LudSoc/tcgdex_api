use crate::endpoints::cards::CardBrief;
use crate::endpoints::series::SerieBrief;
use crate::errors;
use crate::is_empty::IsEmpty;
use serde::Deserialize;

use crate::query::{Response, URL_BASE};

const OBJECT_NAME: &str = "sets";

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
pub struct CardCountBrief {
    /// The total amount of cards in set including hidden.
    pub total: u16,

    /// The amount of cards in set.
    pub official: u16,
}

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CardCount {
    /// The total amount of cards in set including hidden.
    pub total: u16,

    /// The amount of cards in set.
    pub official: u16,

    /// The amount of cards in this set that can be found in reverse holo.
    pub reverse: u16,

    /// The amount of cards in this set that can be found in holo.
    pub holo: u16,

    /// The amount of cards in this set that can be found with the firstEdition Tag.
    pub first_ed: u16,
}

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Legal {
    /// Ability to use this set in standard competitions.
    pub standard: bool,

    /// Ability to use this set in expanded competitions.
    pub expanded: bool,
}

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SetBrief {
    /// Unique identifier for the object.
    pub id: String,

    /// The name of the set.
    pub name: String,

    /// The url to the logo image.
    #[serde(default)]
    pub logo: String,

    /// The url to the symbol image.
    #[serde(default)]
    pub symbol: String,

    /// Number of cards in the set.
    pub card_count: CardCountBrief,
}

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    /// Unique identifier for the object.
    pub id: String,

    /// The name of the serie.
    pub name: String,

    /// The url to the logo image.
    #[serde(default)]
    pub logo: String,

    /// The url to the symbol image.
    #[serde(default)]
    pub symbol: String,

    /// Number of cards in the set.
    pub card_count: CardCount,

    /// Information about the serie.
    pub serie: SerieBrief,

    /// Pokémon TCG Online Set code.
    #[serde(default)]
    pub tcg_online: String,

    /// The set release date in the form yyyy-mm-dd.
    pub release_date: String,

    /// Set usability in competitions.
    pub legal: Legal,

    /// List of cards in this set.
    pub cards: Vec<CardBrief>,
}

impl IsEmpty for Set {
    fn is_empty(&self) -> bool {
        self.name.is_empty() && self.id.is_empty()
    }
}

impl IsEmpty for Vec<SetBrief> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// Gives access to module functions.
pub struct SetApi<'a>(pub(crate) &'a reqwest::blocking::Client, pub(crate) String);

impl SetApi<'_> {
    /// Get sets.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tcgdex_api::{Tcgdex, Lang};
    /// use tcgdex_api::endpoints::sets::SetBrief;
    /// let tcgdex = Tcgdex::new();
    /// let sets: Vec<SetBrief> = tcgdex.sets().fetch(None).unwrap();
    /// println!("sets = {:?}", sets);
    /// ```
    ///
    /// # Errors
    ///
    /// If TCGDEX API is updated with URL or JSON format modifications.
    pub fn fetch<T>(&self, query: Option<String>) -> errors::Result<T>
    where
        T: for<'a> Deserialize<'a> + IsEmpty,
    {
        let client: &reqwest::blocking::Client = self.0;
        let mut url_query = String::new();
        let mut separator = String::from("/");

        // if query is used.
        if let Some(f) = query {
            url_query = f;
        }

        // if query is used to filtering, sorting or pagination.
        if url_query.contains('&') || url_query.contains('=') {
            separator = String::from('?')
        }

        let url = format!("{URL_BASE}{}/{OBJECT_NAME}{}{url_query}", self.1, separator);
        println!("{url}");

        let response: Response<T> = client.get(url).send()?.json()?;

        errors::set_error(response)
    }
}
