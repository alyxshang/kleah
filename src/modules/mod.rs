/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the module
/// containing functions for 
/// communicating with the
/// database.
pub mod rw;

/// Exporting the module
/// containing this app's tiny
/// CLI.
pub mod cli;

/// Exporting the module
/// containing this app's
/// error-handling structure.
pub mod err;

/// Exporting the module
/// containing this app's
/// API routes.
pub mod api;

/// Exporting the module
/// containing this app's
/// functions for getting
/// timestamps.
pub mod time;

/// Exporting the module
/// containing this app's
/// utility functions.
pub mod utils;

/// Exporting the module
/// containing tests.
#[cfg(test)]
pub mod tests;

/// Exporting the module
/// containing this app's
/// central structures.
pub mod units;

/// Exporting the 
/// module containing
/// a function for
/// sending an email.
pub mod email;

/// Exporting the module
/// containing this app's
/// app runner function.
pub mod runner;