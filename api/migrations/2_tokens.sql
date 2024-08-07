create table if not exists tokens (
  id serial primary key,
  token varchar(255) unique not null,
  created_at timestamptz not null default now(),
  user_id integer references users (id)
);
