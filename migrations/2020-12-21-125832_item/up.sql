-- Your SQL goes here
create table todo_item (
    id uuid primary key not null default gen_random_uuid(),
    list_id uuid not null,
    task varchar(150) not null,
    finished boolean not null default false,
    foreign key (list_id) references todo_list(id) on delete cascade
)