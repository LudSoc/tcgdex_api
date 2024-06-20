//! Get categories list using [`CategoryApi`].
//!
//! Category is like Pokémon or trainer for example.

use crate::query::URL_BASE;

const OBJECT_NAME: &str = "categories";

/// Gives access to module functions.
pub struct CategoryApi<'a>(pub(crate) &'a reqwest::blocking::Client, pub(crate) String);

impl CategoryApi<'_> {
    /// Get all existing Pokémon categories.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tcgdex_api::{Tcgdex, Lang};
    /// let tcgdex = Tcgdex::new();
    /// println!("categories = {:?}", tcgdex.categories().fetch().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// If TCGDEX API is updated with URL or JSON format modifications.
    pub fn fetch(&self) -> reqwest::Result<Vec<String>> {
        let client: &reqwest::blocking::Client = self.0;
        let categories: Vec<String> = client
            .get(format!("{URL_BASE}{}/{OBJECT_NAME}", self.1))
            .send()?
            .json()?;
        Ok(categories)
    }
}
