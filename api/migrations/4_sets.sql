-- A game is (usually) first to 21 - but win by 2, with 2-3 of them being a match
create table if not exists sets (
  id serial primary key,
  game_id integer references games (id) not null,
  team_one_score integer not null,
  team_two_score integer not null
)
