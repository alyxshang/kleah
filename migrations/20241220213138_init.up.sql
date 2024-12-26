CREATE TABLE users (
    username TEXT NOT NULL PRIMARY KEY,
    display_name TEXT NOT NULL,
    avatar_url TEXT NOT NULL,
    email TEXT NOT NULL,
    pwd TEXT NOT NULL,
    email_token TEXT NOT NULL,
    is_active BOOLEAN NOT NULL
);

CREATE TABLE moods (
    username TEXT NOT NULL PRIMARY KEY,
    is_active BOOLEAN NOT NULL,
    mood TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE
);

CREATE TABLE api_tokens (
    username TEXT NOT NULL PRIMARY KEY,
    token TEXT NOT NULL,
    created_at TEXT NOT NULL,
    is_active BOOLEAN NOT NULL,
    can_change_pwd BOOLEAN NOT NULL,
    can_set_mood BOOLEAN NOT NULL,
    can_delete_user BOOLEAN NOT NULL,
    can_change_email BOOLEAN NOT NULL,
    FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE
);

CREATE TABLE user_files (
    file_id TEXT NOT NULL,
    username TEXT NOT NULL PRIMARY KEY,
    file_name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE
);

CREATE TABLE user_session_tokens (
    username TEXT NOT NULL PRIMARY KEY,
    session_token TEXT NOT NULL,
    FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE
);

CREATE TABLE user_follows (
    follower TEXT NOT NULL PRIMARY KEY,
    followee TEXT NOT NULL,
    FOREIGN KEY (follower) REFERENCES users(username) ON DELETE CASCADE
)