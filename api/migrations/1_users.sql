create table if not exists users (
  id serial primary key,
  name varchar(255) not null,
  -- Can be null, not everyone may have an account
  player_id integer references players (id) not null,
  created_at timestamptz not null default now()
);
