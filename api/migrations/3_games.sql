-- 2 or 3 sets
create table if not exists games (
  id serial primary key,
  created_at timestamptz not null default now(),
  first_to integer not null,
  team_one integer references teams (id) not null,
  team_two integer references teams (id) not null
)
