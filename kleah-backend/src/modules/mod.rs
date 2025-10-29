/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the module
/// containing functions
/// to read from and write
/// to the database.
pub mod db;

/// Exporting the module
/// containing the structure
/// for catching and handling
/// errors.
pub mod err;

/// Exporting the module
/// containing service functions
/// for the backend's REST API
/// service functions.
pub mod api;

/// Decalring the module
/// containing unit tests
/// for this crate.
#[cfg(test)]
pub mod tests;

/// Exporting the module
/// containing utility
/// entities.
pub mod units;

/// Exporting the module
/// containing utility
/// functions.
pub mod utils;

/// Exporting the module
/// containing structures
/// modelling data in the
/// database.
pub mod models;

/// Exporting the module
/// containing structures
/// for accepting JSON
/// payloads.
pub mod payloads;

/// Exporting the module
/// containing structures
/// for sending JSON
/// payloads.
pub mod responses;
