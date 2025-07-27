-- Kleah by Alyx Shang.
-- Licensed under the FSL v1.

-- The table for confidential
-- information about an actor.
CREATE TABLE private_actors(
    username TEXT NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    verified BOOLEAN NOT NULL,
    privileged BOOLEAN NOT NULL,
    private_key TEXT NOT NULL,
    public_key TEXT NOT NULL,
    user_password TEXT NOT NULL,
    default_primary TEXT NOT NULL,
    default_secondary TEXT NOT NULL,
    default_tertiary TEXT NOT NULL
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
    FOREIGN KEY (user_id) REFERENCES private_actors(username) ON DELETE CASCADE
);

-- A table for storing API tokens
-- belonging to a user.
CREATE TABLE user_api_tokens(
    token TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES private_actors(username) ON DELETE CASCADE
);

-- A table for storing instance
-- information.
CREATE TABLE instance_info(
    instance_name TEXT NOT NULL PRIMARY KEY,
    instance_host TEXT NOT NULL,
    instance_smtp TEXT NOT NULL,
    instance_pass TEXT NOT NULL,
    instance_admin TEXT NOT NULL,
    instance_description TEXT NOT NULL,
    default_primary TEXT NOT NULL,
    default_secondary TEXT NOT NULL,
    default_tertiary TEXT NOT NULL,
    FOREIGN KEY (instance_admin) REFERENCES actors(user_id) ON DELETE CASCADE
);

-- A table for storing
-- user notes.
CREATE TABLE notes(
    note_id TEXT NOT NULL PRIMARY KEY,
    author TEXT NOT NULL,
    published TEXT NOT NULL,
    content TEXT NOT NULL,
    like_count INTEGER NOT NULL,
    boost_count INTEGER NOT NULL,
    share_count INTEGER NOT NULL,
    is_reply BOOLEAN NOT NULL,
    reply_to TEXT NOT NULL,
    sensitive BOOLEAN NOT NULL,
    FOREIGN KEY (author) REFERENCES actors(user_id) ON DELETE CASCADE
);

-- A table for storing 
-- user activities.
CREATE TABLE user_acts (
  activity_id TEXT NOT NULL PRIMARY KEY,
  activity_type TEXT NOT NULL,
  activity_author TEXT NOT NULL,
  published_at TEXT NOT NULL,
  object_id TEXT NOT NULL,
  FOREIGN KEY (activity_author) REFERENCES actors(user_id) ON DELETE CASCADE
);

-- A table for storing
-- user files.
CREATE TABLE user_files(
    file_id TEXT NOT NULL PRIMARY KEY,
    file_owner TEXT NOT NULL,
    file_path TEXT NOT NULL,
    visibility BOOLEAN NOT NULL,
    FOREIGN KEY (file_owner) REFERENCES actors(user_id) ON DELETE CASCADE
);
