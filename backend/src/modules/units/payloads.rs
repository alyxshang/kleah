/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the 
/// "Deserialize" trait
/// to derive it.
use serde::Deserialize;


/// A structure containing
/// information to submit
/// a payload for creating
/// charms.
#[derive(Deserialize)]
pub struct CreateCharmPayload {
    pub api_token: String,
    pub charm_text: String,
    pub file_id: Option<String>,
    pub is_reply: bool,
    pub refers_to: String,
    pub reaction_ids: String,
    pub like_count: usize,
    pub reaction_count: usize,
}

/// A structure containing
/// information to submit
/// a payload for deleting
/// charms.
#[derive(Deserialize)]
pub struct DeleteCharmPayload {
    pub api_token: String,
    pub charm_id: String
}

/// A structure containing
/// information to submit
/// a payload for creating
/// a user.
#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub user_role: String,
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub banner_url: String,
    pub user_description: String,
    pub email: String,
    pub pwd: String,
    pub is_private: bool,
    pub email_token: String,
    pub is_active: bool,
    pub rules_accepted: bool,
    pub is_admin: bool
}

/// A structure containing
/// information to submit
/// a payload for deleting
/// a user.
#[derive(Deserialize)]
pub struct DeleteUserPayload {
    pub username: String,
    pub api_token: String
}

/// A structure containing
/// information to submit
/// a payload for creating
/// an API token.
#[derive(Deserialize)]
pub struct CreateUserTokenPayload {
    pub username: String,
    pub password: String,
    pub can_change_pwd: bool,
    pub can_change_username: bool,
    pub can_post_charms: bool,
    pub can_delete_user: bool,
    pub can_change_email: bool
}

/// A structure containing
/// information to submit
/// a payload for deleting
/// an API token.
#[derive(Deserialize)]
pub struct DeleteUserTokenPayload {
    pub password: String,
    pub user_id: String,
    pub api_token: String    
}

/// A structure containing
/// information to submit
/// a payload for obtaining
/// all active API tokens.
#[derive(Deserialize)]
pub struct UserTokensPayload {
    pub password: String,
    pub username: String,
}

/// A structure containing
/// information to submit
/// a payload for changing
/// a user's account info.
#[derive(Deserialize)]
pub struct ChangeEntityPayload {
    pub old_entity: String,
    pub new_entity: String,
    pub api_token: String
}

/// A structure containing
/// information to submit
/// a payload for following
/// a user.
#[derive(Deserialize)]
pub struct UserInteractionPayload {
    pub sender_id: String,
    pub receiver_id: String,
    pub api_token: String
}

/// A structure containing
/// information to submit
/// a payload for creating
/// an invite code.
#[derive(Deserialize)]
pub struct CreateInviteCodePayload {
    pub api_token: String,
    pub invite_code: String
}

/// A structure containing
/// information to submit
/// a payload for deleting
/// an invite code.
#[derive(Deserialize)]
pub struct DeleteInviteCodePayload {
    pub invite_code: String
}

/// A structure for submitting a payload
/// that creates the information of the 
/// Kleah instance.
#[derive(Deserialize)]
pub struct CreateInstanceInfoPayload {
    pub api_token: String,
    pub instance_id: String,
    pub instance_description: String,
    pub instance_name: String,
    pub kleah_version: String,
    pub admin_user_id: String,
    pub instance_rules: String
}

/// A structure for submitting a payload
/// that creates a like by a user on a charm.
#[derive(Deserialize)]
pub struct CharmLikePayload {
    pub api_token: String,
    pub charm_id: String,
}

/// A structure for submitting a payload
/// that creates a reaction by a user on a charm.
#[derive(Deserialize)]
pub struct CharmReactPayload {
    pub api_token: String,
    pub reaction_name: String,
    pub charm_id: String,
}

/// A structure for submitting a payload
/// that retrieves profile information for
/// the owner of the token.
#[derive(Deserialize)]
pub struct ProfilePayload{
    pub api_token: String,
}

/// A structure for submitting a payload
/// that retrieves the charm timeline for
/// the owner of the token.
#[derive(Deserialize)]
pub struct TimelinePayload{
    pub api_token: String
}

/// A structure for submitting a payload
/// that retrieves the charm timeline for
/// the owner of the token.
#[derive(Deserialize)]
pub struct CharmDetailPayload{
    pub charm_id: String,
}

/// A structure for submitting a
/// payload for saving
/// a user-created theme.
#[derive(Deserialize)]
pub struct CreateThemePayload {
    pub api_token: String,
    pub theme_name: String,
    pub primary_color: String,
    pub accent_color: String
}

/// A structure for submitting a
/// payload for deleting 
/// a user-created theme.
#[derive(Deserialize)]
pub struct DeleteThemePayload {
    pub api_token: String,
    pub theme_id: String
}

/// A structure for submitting a
/// payload for creating an API
/// token for logging in.
#[derive(Deserialize)]
pub struct LoginTokenPayload{
    pub username: String,
    pub password: String
}

/// A structure for submitting a
/// payload for deleting an API
/// token for logging in.
#[derive(Deserialize)]
pub struct DiscardLoginTokenPayload{
    pub api_token: String,
}

/// A structure for submitting a
/// payload for updating the privacy
/// perms on a user-uploaded file.
#[derive(Deserialize)]
pub struct UpdateFilePermsPayload{
    pub api_token: String,
    pub file_id: String,
    pub is_private: bool,
}

/// A structure for submitting a
/// payload for deleting a
/// user-uploaded file.
#[derive(Deserialize)]
pub struct DeleteFilePayload {
    pub api_token: String,
    pub file_id: String,
}

/// A structure for submitting a
/// payload for creating an 
/// instance-specific reaction.
#[derive(Deserialize)]
pub struct CreateInstanceReactionPayload{
    pub file_id: String,
    pub api_token: String,
    pub reaction_name: String,
}

/// A structure for submitting a
/// payload for deleting an 
/// instance-specific reaction.
#[derive(Deserialize)]
pub struct DeleteInstanceReactionPayload{
    pub api_token: String,
    pub reaction_name: String,
}

/// A structure for submitting a
/// payload for proclaiming a
/// user-created post.
#[derive(Deserialize)]
pub struct ProclamationPayload {
    pub api_token: String,
    pub charm_id: String
}

/// A structure for submitting a
/// payload for gathering all charms
/// a user has liked.
#[derive(Deserialize)]
pub struct LikedCharmsPayload {
    pub api_token: String
}

/// A structure for submitting a
/// payload for gathering all charms
/// a user has reacted to.
#[derive(Deserialize)]
pub struct ReactedCharmsPayload {
    pub api_token: String
}

/// A structure for submitting a
/// payload for gathering all charms
/// a user has proclaimed.
#[derive(Deserialize)]
pub struct ProclaimedCharmsPayload {
    pub api_token: String
}

/// A structure for submitting a
/// payload for verifying an invite
/// code.
#[derive(Deserialize)]
pub struct CheckInvitePayload{
    pub invite_code: String
}

/// A structure for submitting 
/// a payload for retrieving
/// information about a user
/// profile while also
/// respecting viewership rights.
#[derive(Deserialize)]
pub struct UserViewPayload{
    pub api_token: String,
    pub issuer: String
}

/// A structure for submitting 
/// a payload for performing
/// actions that only require
/// an API token.
#[derive(Deserialize)]
pub struct APITokenOnlyPayload{
    pub api_token: String,
}

/// A structure for submitting 
/// a payload for banning a user.
#[derive(Deserialize)]
pub struct BanningPayload{
    pub api_token: String,
    pub target: String
}

/// A structure for submitting 
/// a payload that only requires
/// a username.
#[derive(Deserialize)]
pub struct PublicUserViewPayload{
    pub username: String
}