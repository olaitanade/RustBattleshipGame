use battleship_game_lib::{GamePlay, runtime::GridPoint};

fn main() {
    let mut game = GamePlay::initialize();
    let play = game.start_new(String::from("Adetayo"));

    play.get_session_as_mut().shoot_ship(GridPoint { x: 1 , y:  1});
    play.get_session_as_mut().shoot_ship(GridPoint { x: 2 , y:  2});
    play.get_session_as_mut().shoot_ship(GridPoint { x: 3 , y:  3});
    play.get_session_as_mut().shoot_ship(GridPoint { x: 4 , y:  4});
    play.get_session_as_mut().shoot_ship(GridPoint { x: 5 , y:  5});
    play.get_session_as_mut().shoot_ship(GridPoint { x: 6 , y:  6});
    play.get_session_as_mut().shoot_ship(GridPoint { x: 7 , y:  7});
    play.get_session_as_mut().shoot_ship(GridPoint { x: 8 , y:  8});
    play.get_session_as_mut().shoot_ship(GridPoint { x: 9 , y:  9});
    
    println!("{:?}", play.get_session_as_ref().get_destroyed_ships());
}
