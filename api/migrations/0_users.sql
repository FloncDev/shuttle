create table if not exists users (
  id serial primary key,
  name varchar(255) not null,
  created_at timestamptz not null default now()
);
