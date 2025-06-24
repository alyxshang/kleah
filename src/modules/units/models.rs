/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "FromRow"
/// derive macro for reading
/// and writing data to and
/// from the database.
use sqlx::FromRow;

/// A model for storing
/// invite codes.
#[derive(FromRow)] // done.
pub struct InviteCode{
    pub code_id: String,
    pub code: String,
    pub admin_id: String
}

/// A model for storing
/// information about 
/// the Kleah instance.
#[derive(FromRow)] // done.
pub struct InstanceInfo{
    pub info_id: String,
    pub instance_host: String,
    pub instance_smtp: String,
    pub instance_pass: String,
    pub instance_admin: String
}

/// A model for storing
/// a user's API tokens.
#[derive(FromRow)] // done.
pub struct UserAPIToken{
    pub token: String,
    pub user_id: String
}

/// A model for storing
/// a relationship
/// of a block.
#[derive(FromRow)] // done.
pub struct ActorBlock{
    pub rel_id: String,
    pub actor_id: String,
    pub blocked_actor: String
}

/// A model for storing
/// different tags for
/// an activity.
#[derive(FromRow)]
pub struct ActivityHashtag{
    pub tag_id: String,
    pub tag_type: String,
    pub href: String,
    pub tag_name: String
}

/// A model for storing
/// a streamed or public
/// activity.
#[derive(FromRow)]
pub struct StreamedActivity {
    pub activity_id: String,
    pub activity_type: String,
    pub actor_id: String,
    pub published: String,
}

/// A model for storing
/// an attachment to an
/// activity.
#[derive(FromRow)]
pub struct ActivityAttachment{
    pub attachment_id: String,
    pub media_id: String,
}

/// A model for storing
/// an activity.
#[derive(FromRow)]
pub struct Activity{
    pub activity_id: String,
    pub activity_type: String,
    pub summary: String,
    pub in_reply_to: String,
    pub published: String,
    pub actor_id: String,
    pub sensitive: String,
    pub context_conversation: String,
    pub content: String,
    pub is_reply: bool,
    pub like_count: usize,
    pub share_count: usize,
}

/// A model for storing
/// a info about a file
/// owned by an actor.
#[derive(FromRow)] // done.
pub struct ActorFile{
    pub media_id: String,
    pub user_id: String,
    pub file_path: String,
    pub file_extension: String,
}

/// A model for storing
/// confidential info
/// of an actor.
#[derive(FromRow)] // done.
pub struct PrivateActor{
    pub username: string,
    pub user_id: String,
    pub email: String,
    pub verified: bool,
    pub privileged: bool,
    pub private_key: String,
    pub public_key: String,
    pub user_password: String,
}

/// A model for storing
/// a following relationship.
#[derive(FromRow)] // done.
pub struct ActorFollower{
    pub rel_id: String,
    pub actor_id: String,
    pub follower_id: String,
}

/// A model for storing
/// a link submitted by
/// an actor.
#[derive(FromRow)]
pub struct ActorLink{
    pub link_id: String,
    pub verified: bool,
    pub actor_id: String,
    pub link_name: String,
    pub link_url: String,
}

/// A model for storing
/// info pertaining to
/// an actor.
#[derive(FromRow)] // done.
pub struct Actor{
    pub user_id: String,
    pub host: String,
    pub user_type: String,
    pub preferred_username: String,
    pub display_name: String,
    pub summary: String,
    pub manually_approves_followers: bool,
    pub discoverable: bool,
    pub indexable: bool,
    pub published: String,
    pub memorial: bool,
    pub banner_id: String,
    pub pfp_id: String,
}
