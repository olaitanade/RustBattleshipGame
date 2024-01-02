use std::collections::HashMap;
use crate::runtime::Play;



pub struct Score {
    pub name: String,
    pub point: i32
}

pub struct Store {
    plays: HashMap<String, Play>,
    scores: Vec<Score>,
}

impl Store{
    pub fn build() -> Store {
        Store { plays: HashMap::new(), scores: Vec::new() }
    }

    pub fn build_with(plays: HashMap<String, Play>, scores: Vec<Score>) -> Store{
        Store { plays, scores }
    }

    pub fn save_play(&mut self, play: Play){
        self.add_score(Score { name: play.get_session_as_ref().get_player_name(), point: play.get_session_as_ref().get_points() });
        self.plays.insert(play.get_session_as_ref().get_player_name(), play);
    }

    pub fn pop_play(&mut self, player_name: &String) -> Option<Play> {
        self.plays.remove(player_name)
    }

    pub fn get_play(&self, player_name: &String) -> Option<Play>{
        self.plays.get(player_name).cloned()
    }

    pub fn add_score(&mut self, score: Score){
        self.scores.push(score);
    }

    pub fn remove_score(&mut self, score: Score){
        self.scores.retain(|x| x.point != score.point && x.name != score.name);
    }

    pub fn get_player_names(&self) -> Vec<String> {
        self.plays.keys().cloned().collect()
    }

}

#[cfg(test)]
mod tests {

    use crate::runtime::GridPoint;

    use super::*;

    #[test]
    fn test_save_session() {
        let mut store = Store::build();
        let mut play = Play::init(String::from("Adetayo"));
        let mut game_session = play.get_session_as_mut();

        game_session.shoot_ship(GridPoint { x: 2 , y:  2});
        game_session.shoot_ship(GridPoint { x: 3 , y:  3});
        game_session.shoot_ship(GridPoint { x: 4 , y:  4});
        game_session.shoot_ship(GridPoint { x: 5 , y:  5});
        game_session.shoot_ship(GridPoint { x: 6 , y:  6});
        game_session.shoot_ship(GridPoint { x: 7 , y:  7});
        game_session.shoot_ship(GridPoint { x: 8 , y:  8});


        println!("{}", game_session.get_remaining_shots());
        println!("{}", game_session.display_ships_location());
        println!("{:?}", game_session.get_destroyed_ships());
        
        println!("---------------------------------------");
        store.save_play(play);

        play = store.pop_play(&String::from("Adetayo")).unwrap();
        game_session = play.get_session_as_mut();

        println!("---------------------------------------");
        println!("{}", game_session.get_remaining_shots());
        println!("{}", game_session.display_ships_location());
        println!("{:?}", game_session.get_destroyed_ships());


    }
}