use serde::{Deserialize, Serialize};
use axum::http::StatusCode;
use serde_json::Value;
use crate::utils::serialization::response::{serialize_status_code, deserialize_status_code};

/// `ApiResponse` struct that represents a generic response from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    /// the response status
    #[serde(serialize_with = "serialize_status_code", deserialize_with = "deserialize_status_code")]
    pub status: StatusCode,
    /// (optional) an error message
    pub error: Option<String>,
    /// an additional message, often explaining either the error or the success of the response
    pub message: String,
    /// an optional data field, used to store the data of the response
    /// 
    /// note that some requests, especially POST requests, may not have `data` available
    pub data: Option<Value>,
    /// an optional pagination field, used to store pagination data
    pub pagination: Option<Pagination>,
    /// the current version of the API
    pub version: u8,
}

/// `Pagination` is a struct used to store data that uses pagination.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    /// represents the total amount of items in the entire dataset/collection
    pub total_items: usize,
    /// the current page of the dataset/collection that is being paginated
    pub page: usize,
    /// the amount of items per page
    pub page_size: usize,
}