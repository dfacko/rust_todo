-- Your SQL goes here
create table todo_item (
    id serial primary key,
    list_id integer not null,
    task varchar(150) not null,
    finished boolean not null default false,
    foreign key (list_id) references todo_list(id)
)