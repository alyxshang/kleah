/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "Serialize"
/// macro for serializing Rust
/// data structures into JSON.
use serde::Serialize;

/// Importing the "Deserialize"
/// macro for deserializing Rust
/// data structures from JSON.
use serde::Deserialize;

/// An enum to describe the different
/// types an actor can be.
#[derive(Serialize, Deserialize)]
pub enum UserType{
    Group,
    Person,
    Service,
    Application,
    Organization,
}

/// An enum to describe the different
/// types of media an actor can upload.
#[derive(Serialize, Deserialize)]
pub enum ActorMediaType{
    Image,
    Text,
    Other
}

/// An enum to describe the different
/// types of activities an actor can do.
#[derive(Serialize, Deserialize)]
pub enum ActivityType{
    Create,
    Delete,
    Update,
    Add,
    Remove,
    Follow,
}

/// An enum to describe the different
/// types of activities an actor can create.
#[derive(Serialize, Deserialize)]
pub enum ActivityEntityType{
    Note,
}

/// An enum to describe the different
/// types of attachments an actor can
/// make.
#[derive(Serialize, Deserialize)]
pub enum ActorAttachmentType{
    Other,
    PropertyValue
}

/// An enum to describe the different
/// types of collections possible.
#[derive(Serialize, Deserialize)]
pub enum CollectionType{
    Collection,
    CollectionPage,
}