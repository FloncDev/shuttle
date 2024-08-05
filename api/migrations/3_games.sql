-- 2 or 3 sets
create table if not exists duo_games (
  id serial primary key,
  created_at timestamptz not null default now(),
  first_to integer not null,
  team_one integer references teams (id) not null,
  team_two integer references teams (id) not null,
  sets json not null
);

create table if not exists solo_games (
  id serial primary key,
  created_at timestamptz not null default now(),
  first_to integer not null,
  player_one integer references users (id) not null,
  player_two integer references users (id) not null,
  sets json not null
)
