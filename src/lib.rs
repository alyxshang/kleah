/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

/// Declaring the "modules"
/// directory as a module.
pub mod modules;

/// Re-exporting the module
/// containing functions for 
/// communicating with the
/// database.
pub use modules::rw::*;

/// Re-exporting the module
/// containing this app's tiny
/// CLI.
pub use modules::cli::*;

/// Re-exporting the module
/// containing this app's
/// error-handling structure.
pub use modules::err::*;

/// Re-exporting the module
/// containing this app's
/// API routes.
pub use modules::api::*;

/// Re-exporting the module
/// containing this app's
/// functions for getting
/// timestamps.
pub use modules::time::*;

/// Re-exporting the module
/// containing this app's
/// utility functions.
pub use modules::utils::*;

/// Re-exporting the module
/// containing this app's
/// central structures.
pub use modules::units::*;

/// Re-exporting the 
/// module containing
/// a function for
/// sending an email.
pub use modules::email::*;

/// Re-exporting the module
/// containing this app's
/// app runner function.
pub use modules::runner::*;