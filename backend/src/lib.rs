/*
Kleah Backend by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// that handles read-write
/// functionality for the 
/// database.
pub use modules::rw::*;

/// Re-exporting the
/// module containing
/// this crate's error
/// structure.
pub use modules::err::*;

/// Re-exporting the
/// module containing
/// this crate's CLI.
pub use modules::cli::*;

/// Re-exporting the
/// module containing
/// this crate's API
/// views.
pub use modules::api::*;

/// Re-exporting the
/// module containing
/// some utility functions.
pub use modules::utils::*;

/// Re-exporting the
/// module containing
/// this crate's structures.
pub use modules::units::*;

/// Re-exporting the
/// module containing
/// the function to run
/// different components
/// together.
pub use modules::runner::*;