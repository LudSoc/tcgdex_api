//! Get illustrators list using [`IllustratorApi`].

use crate::query::URL_BASE;

const OBJECT_NAME: &str = "illustrators";

/// Gives access to module functions.
pub struct IllustratorApi<'a>(pub(crate) &'a reqwest::blocking::Client, pub(crate) String);

impl IllustratorApi<'_> {
    /// Get all existing Pokémon illustrators.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tcgdex_api::{Tcgdex, Lang};
    /// let tcgdex = Tcgdex::new();
    /// println!("illustrators = {:?}", tcgdex.illustrators().fetch().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// If TCGDEX API is updated with URL or JSON format modifications.
    pub fn fetch(&self) -> reqwest::Result<Vec<String>> {
        let client: &reqwest::blocking::Client = self.0;
        let illustrators: Vec<String> = client
            .get(format!("{URL_BASE}{}/{OBJECT_NAME}", self.1))
            .send()?
            .json()?;
        Ok(illustrators)
    }
}
