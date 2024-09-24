//! To construct a query.

use crate::errors::TcgdexError;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

/// Constant part of the URL for queries.
pub(crate) const URL_BASE: &str = "https://api.tcgdex.net/v2/";

#[derive(Debug)]
pub enum Order {
    ASC,
    DESC,
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// To build a query with specified parameters.
///
/// Check [TCGdex API reference](https://tcgdex.dev) for details about query parameters.
///
/// If id is already set, set filtering, sorting or pagination have no effect.
///
/// If at least one of filtering, sorting and pagination is already set, set id have no effect.
///
/// # Example
///
/// ```
/// # use tcgdex_api::query::{Order, Query};
///
/// // to get a filtered card list
/// let query = Query::new().with_filtering(vec!["hp=100"]).with_sorting("name", &Order::ASC).to_string();
///
/// // to get a specific card with its id
/// let query = Query::new().with_id("swsh3-136").to_string();
/// ```

pub struct Query {
    id: String,
    filtering: String,
    pagination: String,
    sorting: String,
}

impl Query {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: String::new(),
            filtering: String::new(),
            pagination: String::new(),
            sorting: String::new(),
        }
    }

    #[must_use]
    pub fn with_id(mut self, id: &str) -> Self {
        if self.filtering.is_empty() && self.sorting.is_empty() && self.pagination.is_empty() {
            self.id = id.to_string();
        }
        self
    }

    #[must_use]
    pub fn with_filtering(mut self, filter: Vec<&str>) -> Self {
        if self.id.is_empty() {
            let mut fixed_filter: Vec<&str> = Vec::new();
            for item in filter {
                fixed_filter.push(item.split_whitespace().collect::<Vec<&str>>()[0]);
            }
            self.filtering = fixed_filter.join("&");
        }
        self
    }

    #[must_use]
    pub fn with_pagination(mut self, page: u8, items_per_page: u16) -> Self {
        if self.id.is_empty() {
            self.pagination =
                format!("pagination:page={page}&pagination:itemsPerPage={items_per_page}");
        }
        self
    }

    #[must_use]
    pub fn with_sorting(mut self, field: &str, order: &Order) -> Self {
        if self.id.is_empty() {
            self.sorting = format!("sort:field={field}&sort:order={order}");
        }
        self
    }
}
impl Display for Query {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let query: Vec<String> = [&self.id, &self.filtering, &self.pagination, &self.sorting]
            .into_iter()
            .filter(|v| !v.is_empty())
            .map(std::string::ToString::to_string)
            .collect();
        write!(f, "{}", query.join("&"))
    }
}

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}

/// Request response can be a T data structure in case of success
///
/// or can be an error structure in some cases of failure.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum Response<T> {
    Error(TcgdexError),
    Data(T),
}
