//! Get retreat costs list using [`RetreatApi`].

use crate::query::URL_BASE;

const OBJECT_NAME: &str = "retreats";

/// Gives access to module functions.
#[derive(Debug)]
pub struct RetreatApi<'a>(pub(crate) &'a reqwest::blocking::Client, pub(crate) String);

impl RetreatApi<'_> {
    /// Get all existing Pokémon retreat costs.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tcgdex_api::{Tcgdex, Lang};
    /// let tcgdex = Tcgdex::new();
    /// println!("retreats = {:?}", tcgdex.retreats().fetch().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// If TCGDEX API is updated with URL or JSON format modifications.
    pub fn fetch(&self) -> reqwest::Result<Vec<u8>> {
        let client: &reqwest::blocking::Client = self.0;
        let retreats: Vec<u8> = client
            .get(format!("{URL_BASE}{}/{OBJECT_NAME}", self.1))
            .send()?
            .json()?;
        Ok(retreats)
    }
}
