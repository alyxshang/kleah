-- Kleah by Alyx Shang.
-- Licensed under the FSL v1.

-- The table for confidential
-- information about an actor.
CREATE TABLE private_actors(
    user_id TEXT NOT NULL PRIMARY KEY,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    verified BOOLEAN NOT NULL,
    privileged BOOLEAN NOT NULL,
    private_key TEXT NOT NULL,
    public_key TEXT NOT NULL,
    user_password TEXT NOT NULL
);

-- The table for saving
-- info about an actor.
CREATE TABLE actors (
    user_id TEXT NOT NULL PRIMARY KEY,
    host TEXT NOT NULL,
    user_type TEXT NOT NULL,
    preferred_username TEXT NOT NULL,
    display_name TEXT NOT NULL,
    summary TEXT NOT NULL,
    manually_approves_followers BOOLEAN NOT NULL,
    discoverable BOOLEAN NOT NULL,
    indexable BOOLEAN NOT NULL,
    published TEXT NOT NULL,
    memorial BOOLEAN NOT NULL,
    banner_id TEXT NOT NULL,
    pfp_id TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES private_actors(user_id) ON DELETE CASCADE
);

-- A table for saving invite codes.
CREATE TABLE invite_codes(
    code_id TEXT NOT NULL PRIMARY KEY,
    code TEXT NOT NULL,
    admin_id TEXT NOT NULL,
    FOREIGN KEY (admin_id) REFERENCES actors(user_id) ON DELETE CASCADE
);

-- The table for
-- for activities.
CREATE TABLE actor_activities(
    activity_id TEXT NOT NULL PRIMARY KEY,
    activity_type TEXT NOT NULL,
    summary TEXT NOT NULL,
    in_reply_to TEXT,
    published TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    sensitive BOOLEAN NOT NULL,
    context_conversation TEXT NOT NULL,
    content TEXT NOT NULL,
    is_reply BOOLEAN NOT NULL,
    like_count INT,
    share_count INT,
    FOREIGN KEY (actor_id) REFERENCES private_actors(user_id) ON DELETE CASCADE
);

-- The table for hashtags for
-- activities.
CREATE TABLE activity_hashtags(
    tag_id TEXT NOT NULL PRIMARY KEY,
    activity_id TEXT NOT NULL,
    tag_type TEXT NOT NULL,
    href TEXT NOT NULL,
    tag_name TEXT NOT NULL,
    FOREIGN KEY (activity_id) REFERENCES actor_activities(activity_id) ON DELETE CASCADE
);

-- A table for storing instance
-- information.
CREATE TABLE instance_info(
    info_id TEXT NOT NULL PRIMARY KEY,
    instance_host TEXT NOT NULL,
    instance_smtp TEXT NOT NULL,
    instance_pass TEXT NOT NULL,
    instance_admin TEXT NOT NULL,
    FOREIGN KEY (instance_admin) REFERENCES actors(user_id) ON DELETE CASCADE
);


-- A table for storing API tokens
-- belonging to a user.
CREATE TABLE user_api_token(
    token_id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES private_actors(user_id) ON DELETE CASCADE
);

-- The table for saving
-- blocking relationships 
-- between actors.
CREATE TABLE actor_blocks(
    rel_id TEXT NOT NULL PRIMARY KEY,
    actor_id TEXT NOT NULL,
    blocked_actor TEXT NOT NULL,
    FOREIGN KEY (actor_id) REFERENCES actors(user_id) ON DELETE CASCADE
);

-- The table for an activity inside
-- an activity stream.
CREATE TABLE streamed_activities (
    activity_id TEXT NOT NULL PRIMARY KEY,
    activity_type TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    published TEXT NOT NULL,
    FOREIGN KEY (activity_id) REFERENCES actor_activities(activity_id) ON DELETE CASCADE
);

-- The table for files owned
-- by an actor.
CREATE TABLE actor_files(
    media_id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_extension TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES private_actors(user_id) ON DELETE CASCADE
);

-- The table for attachments 
-- for activities.
CREATE TABLE activity_attachments(
    attachment_id TEXT NOT NULL PRIMARY KEY,
    media_id TEXT NOT NULL,
    FOREIGN KEY (media_id) REFERENCES actor_files(media_id) ON DELETE CASCADE
);

-- The table for saving
-- following relationships
-- between actors.
CREATE TABLE actor_followers(
    rel_id TEXT NOT NULL PRIMARY KEY,
    actor_id TEXT NOT NULL,
    follower_id TEXT NOT NULL,
    FOREIGN KEY (actor_id) REFERENCES private_actors(user_id) ON DELETE CASCADE
);

-- The table for saving
-- links and attributes
-- about actors.
CREATE TABLE actor_links(
    link_id TEXT NOT NULL PRIMARY KEY,
    verified BOOLEAN NOT NULL,
    actor_id TEXT NOT NULL,
    link_name TEXT NOT NULL,
    link_url TEXT NOT NULL,
    FOREIGN KEY (actor_id) REFERENCES private_actors(user_id) ON DELETE CASCADE
);
