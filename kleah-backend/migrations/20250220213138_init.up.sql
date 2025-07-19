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
    FOREIGN KEY (instance_admin) REFERENCES actors(user_id) ON DELETE CASCADE
);

