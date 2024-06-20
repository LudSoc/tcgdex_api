//! Get HP list using [`HpApi`].

use crate::query::URL_BASE;

const OBJECT_NAME: &str = "hp";

/// Gives access to module functions.
pub struct HpApi<'a>(pub(crate) &'a reqwest::blocking::Client, pub(crate) String);

impl HpApi<'_> {
    /// Get all existing Pokémon HP.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tcgdex_api::{Tcgdex, Lang};
    /// let tcgdex = Tcgdex::new();
    /// println!("hps = {:?}", tcgdex.hps().fetch().unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// If TCGDEX API is updated with URL or JSON format modifications.
    pub fn fetch(&self) -> reqwest::Result<Vec<u16>> {
        let client: &reqwest::blocking::Client = self.0;
        let hps: Vec<u16> = client
            .get(format!("{URL_BASE}{}/{OBJECT_NAME}", self.1))
            .send()?
            .json()?;
        Ok(hps)
    }
}
