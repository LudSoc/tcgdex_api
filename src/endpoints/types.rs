//! Get types list using [`TypeApi`].
//!
//! Types is like fire or psychic for example.

use crate::query::URL_BASE;

const OBJECT_NAME: &str = "types";

/// Gives access to module functions.
pub struct TypeApi<'a>(pub(crate) &'a reqwest::blocking::Client, pub(crate) String);

impl TypeApi<'_> {
    /// Get all existing Pokémon types.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tcgdex_api::{Tcgdex, Lang};
    /// let tcgdex = Tcgdex::new();
    /// println!("types = {:?}", tcgdex.types().fetch().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// If TCGDEX API is updated with URL or JSON format modifications.
    pub fn fetch(&self) -> reqwest::Result<Vec<String>> {
        let client: &reqwest::blocking::Client = self.0;
        let types: Vec<String> = client
            .get(format!("{URL_BASE}{}/{OBJECT_NAME}", self.1))
            .send()?
            .json()?;
        Ok(types)
    }
}
