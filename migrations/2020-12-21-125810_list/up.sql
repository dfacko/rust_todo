-- Your SQL goes here

create table todo_list (
    id uuid primary key not null default gen_random_uuid(),
    user_id uuid not null,
    title varchar(150) not null,
    foreign key (user_id) references user_(id) on delete cascade
)