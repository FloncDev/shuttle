create table if not exists teams (
  id serial primary key,
  player_one integer references users (id) not null,
  player_two integer references users (id) not null
)
