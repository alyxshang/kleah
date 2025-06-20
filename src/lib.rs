/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// containing various utilities.
pub use modules::utils::*;

/// Re-exporting the module
/// containing this crate's
/// data structures.
pub use modules::units::*;

/// Re-exporting the module
/// containing this crate's
/// functions for reading and
/// writing info from and to 
/// the database.
pub use modules::database::*;

/// Re-exporting the module
/// containing this crate's
/// service functions for the
/// API services.
pub use modules::services::*;