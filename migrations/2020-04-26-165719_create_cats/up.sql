
create table cats (
    id serial primary key,
    name text not null unique,
    color text not null,
    age integer not null
);