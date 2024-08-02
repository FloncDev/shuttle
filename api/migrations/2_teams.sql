-- A team can be either 1 or 2 people
create table if not exists teams (
  id serial primary key,
  score integer not null,
  player_one integer references users (id) not null,
  player_two integer references users (id)  -- Can be null if solos
)
