create table users(
    name text not null,
    username text not null primary key,
    password text not null,
    email_addr text not null,
    public_key text not null,
    description text not null,
    private_key text not null
);

create table actors(
    name text not null,
    actor_type text not null,
    host text not null,
    liked text not null,
    inbox text not null,
    outbox text not null,
    following text not null,
    followers text not null,
    username text not null primary key,
    description text not null,
    public_key text not null
);

create table instance_information(
    host text not null primary key
);
