import { GamePlay, GridPoint, Session } from "battleship_game_lib";
import { memory } from "battleship_game_lib/battleship_game_lib_bg.wasm";

console.log("Starting game")

const game = GamePlay.initialize();
const play = game.start_new("Adetayo");

const session = play.get_session_as_mut();
console.log(session.display_ships_location())
session.shoot_ship(GridPoint.new(1,1))


session.shoot_ship(GridPoint.new(2,2));

session.shoot_ship(GridPoint.new(3,3));

session.shoot_ship(GridPoint.new(4,4));

session.shoot_ship(GridPoint.new(5,5));

session.shoot_ship(GridPoint.new(6,6));

session.shoot_ship(GridPoint.new(7,7));

session.shoot_ship(GridPoint.new(8,8));

session.shoot_ship(GridPoint.new(9,9));

session.shoot_ship(GridPoint.new(10,10));

console.log(session.get_points())


console.log("Ending game")