//! Get rarities list using [`RarityApi`].
//!
//! Rarity is like common or rare for example.

use crate::query::URL_BASE;

const OBJECT_NAME: &str = "rarities";

/// Gives access to module functions.
#[derive(Debug)]
pub struct RarityApi<'a>(pub(crate) &'a reqwest::blocking::Client, pub(crate) String);

impl RarityApi<'_> {
    /// Get all existing Pokémon rarities.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tcgdex_api::{Tcgdex, Lang};
    /// let tcgdex = Tcgdex::new();
    /// println!("rarities = {:?}", tcgdex.rarities().fetch().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// If TCGDEX API is updated with URL or JSON format modifications.
    pub fn fetch(&self) -> reqwest::Result<Vec<String>> {
        let client: &reqwest::blocking::Client = self.0;
        let rarities: Vec<String> = client
            .get(format!("{URL_BASE}{}/{OBJECT_NAME}", self.1))
            .send()?
            .json()?;
        Ok(rarities)
    }
}
