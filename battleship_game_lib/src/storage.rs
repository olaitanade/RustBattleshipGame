use std::collections::HashMap;

use crate::runtime::Session;



pub struct Score {
    pub name: String,
    pub point: i32
}

pub struct Store<'a> {
    sessions: HashMap<String, Session<'a>>,
    scores: Vec<Score>,
}

impl <'a> Store<'a> {
    pub fn build() -> Store<'a>{
        Store { sessions: HashMap::new(), scores: Vec::new() }
    }

    pub fn build_with(sessions: HashMap<String, Session<'a>>, scores: Vec<Score>) -> Store<'a>{
        Store { sessions, scores }
    }

    pub fn save_session(&mut self, session: Session<'a>){
        self.sessions.insert(session.get_player_name(), session);
    }

    pub fn remove_session(&mut self, player_name: &String){
        self.sessions.remove(player_name);
    }

    pub fn get_session(&self, player_name: &String) -> Option<Session<'_>>{
        self.sessions.get(player_name).cloned()
    }

    pub fn add_score(&mut self, score: Score){

    }

    pub fn remove_score(&mut self, score: Score){

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship() {
        
    }
}