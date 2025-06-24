/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Exporting the module
/// for reading and writing
/// information about blocks
/// and follows to and
/// from the database.
pub mod rel;

/// Exporting the 
/// module for reading
/// and writing data
/// about users.
pub mod users;

/// Exporting the module
/// for reading and writing
/// information about
/// API tokens to and 
/// from the database.
pub mod token;

/// Exporting the
/// module for
/// reading and
/// writing data
/// about files
/// to and from 
/// the database.
pub mod files;

/// Exporting the 
/// module for reading
/// and writing data
/// about invite codes
/// for a Kleah
/// instance.
pub mod invites;

/// Exporting the function
/// for reading and writing
/// information about activities
/// to and from the database.
pub mod activity;

/// Exporting the 
/// module for reading
/// and writing data
/// about the Kleah
/// instance.
pub mod instance;
