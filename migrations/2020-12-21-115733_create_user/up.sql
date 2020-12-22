-- Your SQL goes here
create table user_ (
    id serial primary key,
    username varchar(150)  unique not null,
    pword varchar(150) not null
)