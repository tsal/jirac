//!
//!

// ============================================================================
// External Crates
// ============================================================================
extern crate readonly;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

// ============================================================================
// Public Modules
// ============================================================================
pub mod client;
pub mod credentials;
pub mod v2;

// ============================================================================
// Private Modules
// ============================================================================
mod errors;
mod options;

// ============================================================================
// Use
// ============================================================================
pub use crate::client::*;
pub use crate::credentials::*;
pub use crate::errors::*;
pub use crate::options::*;
pub use crate::serde::{Deserialize, Serialize};

// ============================================================================
// Public Structures
// ============================================================================
/// Represents a response from the API.
pub struct Resp<D> {
    pub data: D,
    pub headers: reqwest::header::HeaderMap,
}

// ============================================================================
// Type
// ============================================================================
pub type Result<T> = std::result::Result<T, Error>;
pub type Response<T> = std::result::Result<Resp<T>, Error>;
