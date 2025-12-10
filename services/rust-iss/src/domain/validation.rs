use serde::Deserialize;
use serde_valid_derive::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct OsdrItemValidation {
    #[validate(min_length = 1)]
    pub dataset_id: Option<String>,
    #[validate(min_length = 1)]
    pub title: Option<String>,
    #[validate(min_length = 1)]
    pub status: Option<String>,
}
