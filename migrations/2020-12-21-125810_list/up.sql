-- Your SQL goes here

create table todo_list (
    id serial primary key not null,
    user_id integer not null,
    title varchar(150) not null,
    foreign key (user_id) references user_(id)
)