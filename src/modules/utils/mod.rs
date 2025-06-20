/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting this 
/// crate's error-handling
/// structure.
pub mod err;

/// Exporting the module
/// for verifying a user's
/// mail address.
pub mod email;

/// Exporting the module
/// containing some
/// utility functions.
pub mod utils;

/// Exporting the module
/// for verifying a link
/// a user has submitted.
pub mod links;

/// Exporting the module
/// for federating info
/// from other instances.
pub mod foreign;

/// Exporting the module
/// for detecting mentions.
pub mod mentions;