/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// containing functions
/// to read from and write
/// to the database.
pub use modules::db::*;

/// Re-exporting the module
/// containing the structure
/// for catching and handling
/// errors.
pub use modules::err::*;

/// Re-exporting the module
/// containing service functions
/// for the backend's REST API
/// service functions.
pub use modules::api::*;

/// Re-exporting the module
/// containing utility
/// entities.
pub use modules::units::*;

/// Re-exporting the module
/// containing utility
/// functions.
pub use modules::utils::*;

/// Re-exporting the module
/// containing structures
/// modelling data in the
/// database.
pub use modules::models::*;

/// Re-exporting the module
/// containing structures
/// for accepting JSON
/// payloads.
pub use modules::payloads::*;
