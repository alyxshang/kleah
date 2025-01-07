-- Kleah by Alyx Shang.
-- Licensed under the FSL v1.

-- The table for users.
CREATE TABLE users (
    user_id TEXT NOT NULL PRIMARY KEY,
    user_role TEXT NOT NULL,
    username TEXT NOT NULL,
    display_name TEXT NOT NULL,
    avatar_url TEXT NOT NULL,
    banner_url TEXT NOT NULL,
    user_description TEXT NOT NULL,
    email TEXT NOT NULL,
    pwd TEXT NOT NULL,
    host TEXT NOT NULL,
    priv_key TEXT NOT NULL,
    pub_key TEXT NOT NULL,
    is_private BOOLEAN NOT NULL,
    email_token TEXT NOT NULL,
    is_active BOOLEAN NOT NULL,
    rules_accepted BOOLEAN NOT NULL,
    is_admin BOOLEAN NOT NULL
);

-- The table for user posts (charms).
CREATE TABLE charms (
    user_id TEXT NOT NULL,
    charm_id TEXT NOT NULL PRIMARY KEY,
    charm_text TEXT NOT NULL,
    created_at TEXT NOT NULL,
    file_id TEXT NOT NULL,
    is_reply BOOLEAN NOT NULL,
    refers_to TEXT NOT NULL,
    reaction_ids TEXT NULL,
    proclamation_count INT,
    like_count INT,
    reaction_count INT,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- The table for saving instance info.
CREATE TABLE instance_information (
    instance_id TEXT NOT NULL PRIMARY KEY,
    instance_description TEXT NOT NULL,
    instance_name TEXT NOT NULL,
    kleah_version TEXT NOT NULL,
    admin_user_id TEXT NOT NULL,
    instance_rules TEXT NOT NULL
);

-- The table for saving API tokens for
-- users.
CREATE TABLE api_tokens (
    user_id TEXT NOT NULL,
    token TEXT NOT NULL PRIMARY KEY,
    created_at TEXT NOT NULL,
    is_active BOOLEAN NOT NULL,
    can_change_pwd BOOLEAN NOT NULL,
    can_change_username BOOLEAN NOT NULL,
    can_post_charms BOOLEAN NOT NULL,
    can_delete_user BOOLEAN NOT NULL,
    can_change_email BOOLEAN NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- The table for saving instance-specific
-- invite codes.
CREATE TABLE invite_codes (
    user_id TEXT NOT NULL,
    invite_code TEXT NOT NULL PRIMARY KEY,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- The table for saving instance-specific
-- invite codes.
CREATE TABLE user_files (
    file_id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    is_private BOOLEAN NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- The table for saving follower
-- relationships.
CREATE TABLE user_follows (
    relationship_id TEXT NOT NULL PRIMARY KEY,
    follower TEXT NOT NULL,
    followee TEXT NOT NULL,
    FOREIGN KEY (follower) REFERENCES users(user_id) ON DELETE CASCADE
);

-- The table for saving blocking
-- relationships.
CREATE TABLE user_blocks (
    block_id TEXT NOT NULL PRIMARY KEY,
    blocker TEXT NOT NULL,
    blockee TEXT NOT NULL,
    FOREIGN KEY (blocker) REFERENCES users(user_id) ON DELETE CASCADE
);

-- The table for saving charms
-- a user has liked.
CREATE TABLE user_likes (
    like_id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    charm_id TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (charm_id) REFERENCES charms(charm_id) ON DELETE CASCADE
);

-- The table for saving charms
-- a user has liked.
CREATE TABLE user_reactions (
    reaction_id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    charm_id TEXT NOT NULL,
    file_id TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (charm_id) REFERENCES charms(charm_id) ON DELETE CASCADE
);

-- The table for saving charms
-- a user has promoted.
CREATE TABLE user_proclamations (
    proclamation_id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    charm_id TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- The table for saving themes
-- a user has created.
CREATE TABLE user_themes (
    theme_id TEXT NOT NULL PRIMARY KEY,
    theme_owner TEXT NOT NULL,
    theme_name TEXT NOT NULL,
    primary_color TEXT NOT NULL,
    accent_color TEXT NOT NULL,
    FOREIGN KEY (theme_owner) REFERENCES users(user_id) ON DELETE CASCADE
);

-- The table for instance-specific
-- reactions.
CREATE TABLE instance_reactions (
    reaction_name TEXT NOT NULL PRIMARY KEY,
    file_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
);