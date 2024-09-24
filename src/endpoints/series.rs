use crate::endpoints::sets::SetBrief;
use crate::errors;
use crate::is_empty::IsEmpty;
use serde::Deserialize;

use crate::query::{Response, URL_BASE};

const OBJECT_NAME: &str = "series";

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(default)]
pub struct SerieBrief {
    /// Unique identifier for the object.
    pub id: String,

    /// The name of the serie.
    pub name: String,

    /// The url to the logo image.
    #[serde(default)]
    pub logo: String,
}

#[derive(Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Serie {
    /// Unique identifier for the object.
    pub id: String,

    /// The name of the serie.
    pub name: String,

    /// The url to the logo image.
    #[serde(default)]
    pub logo: String,

    /// The list of sets linked to this serie.
    pub sets: Vec<SetBrief>,
}

impl IsEmpty for Serie {
    fn is_empty(&self) -> bool {
        self.name.is_empty() && self.id.is_empty()
    }
}

impl IsEmpty for Vec<SerieBrief> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// Gives access to module functions.
pub struct SerieApi<'a>(pub(crate) &'a reqwest::blocking::Client, pub(crate) String);

impl SerieApi<'_> {
    /// Get series.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use tcgdex_api::{Tcgdex, Lang};
    /// use tcgdex_api::endpoints::series::SerieBrief;
    /// let tcgdex = Tcgdex::new();
    /// let series: Vec<SerieBrief> = tcgdex.series().fetch(None).unwrap();
    /// println!("series = {:?}", series);
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
