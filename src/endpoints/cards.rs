use crate::endpoints::sets::SetBrief;
use crate::errors;
use crate::is_empty::IsEmpty;
use serde::Deserialize;

use crate::query::{Response, URL_BASE};

const OBJECT_NAME: &str = "cards";

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Variants {
    /// Card available without any shines.
    pub normal: bool,

    /// Card available in Reverse.
    pub reverse: bool,

    /// Card available in Holo.
    pub holo: bool,

    /// Card has a small 1st edition in the middle left.
    pub first_edition: bool,
}

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct Attack {
    /// The name of the attack.
    pub name: String,

    /// The effect of the attack.
    pub effect: String,

    /// The damages of the attack.
    pub damage: u16,
}

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct Item {
    /// The Item name.
    pub name: String,

    /// The Item effect.
    pub effect: String,
}

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct Weakness {
    /// The weakness type.
    pub _type: String,

    /// The weakness value.
    pub value: String,
}

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CardBrief {
    /// Unique identifier for the object.
    pub id: String,

    /// Card local ID.
    pub local_id: String,

    /// The name of the serie.
    pub name: String,

    /// The url to the card image.
    #[serde(default)]
    pub image: String,
}

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default, rename_all = "camelCase")]
pub struct Card {
    /// Unique identifier for the object.
    pub id: String,

    /// Card local ID.
    pub local_id: String,

    /// The name of the card.
    pub name: String,

    /// The url to the card image.
    pub image: String,

    /// Card category.
    pub category: String,

    /// Card illustrator.
    pub illustrator: String,

    /// Card rarity.
    pub rarity: String,

    /// The possible variants of this card.
    pub variants: Variants,

    /// Basic information about the card set.
    pub set: SetBrief,

    // Next fields are only for Pokémon cards.
    /// The National Pokedex ID of the Pokémon on the card.
    pub dex_id: Vec<u16>,

    /// The Pokémon HP.
    pub hp: u16,

    /// The types of the Pokémon.
    pub types: Vec<String>,

    /// The Pokémon name it evolve from.
    pub evolve_from: String,

    /// The card description.
    pub description: String,

    /// The Pokémon level (if it’s a lv.X the level is X).
    pub level: String,

    /// The Pokémon stage.
    pub stage: String,

    /// The card suffix.
    pub suffix: String,

    /// The Pokémon item.
    pub item: Item,

    /// The Pokémon attacks.
    pub attacks: Vec<Attack>,

    /// The Pokémon weaknesses.
    pub weaknesses: Vec<Weakness>,

    /// The Pokémon regulation mark.
    pub regulation_mark: String,

    // Next fields are only for Trainer cards.
    /// The trainer card effect.
    pub effect: String,

    /// The type of trainer card.
    pub trainer_type: String,

    // Next fields are only for Energy cards.
    /// The type of energy card.
    pub energy_type: String,
}

impl IsEmpty for Card {
    fn is_empty(&self) -> bool {
        self.name.is_empty() && self.id.is_empty()
    }
}

impl IsEmpty for Vec<CardBrief> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// Gives access to module functions.
pub struct CardApi<'a>(pub(crate) &'a reqwest::blocking::Client, pub(crate) String);

impl CardApi<'_> {
    /// Get cards.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tcgdex_api::{Tcgdex, Lang};
    /// use tcgdex_api::endpoints::cards::CardBrief;
    /// let tcgdex = Tcgdex::new();
    /// let cards: Vec<CardBrief> = tcgdex.cards().fetch(None).unwrap();
    /// println!("cards = {:?}", cards);
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
            separator = String::from('?');
        }

        let url = format!("{URL_BASE}{}/{OBJECT_NAME}{}{url_query}", self.1, separator);

        let response: Response<T> = client.get(url).send()?.json()?;
        errors::set_error(response)
    }
}
