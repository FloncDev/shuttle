create table if not exists players (
  id serial primary key,
  name varchar(255) not null,
  rating float8 not null default 25.0,
  uncertainty float8 not null default 25/3
);
