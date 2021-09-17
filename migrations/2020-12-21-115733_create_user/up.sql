-- Your SQL goes here
create table user_ (
    id uuid primary key not null default gen_random_uuid(),
    username varchar(150) unique not null ,
    pword varchar(150) not null
)