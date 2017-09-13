use ::*;

#[derive(Debug)]
pub struct Player {
    id: ID,
    name: String,
    scores: Vec<(usize, i32)>,
}

impl Player {
    fn new(player: &raw::Player) -> Self {
        Self {
            id: player.id,
            name: player.name.as_ref().unwrap().clone(),
            scores: Vec::new(),
        }
    }
    fn add_tick(&mut self, tick: usize, player: raw::Player) {
        if let Some(score) = player.score {
            self.scores.push((tick, score));
        }
    }
    fn get_score(&self, tick: usize) -> i32 {
        match self.scores.binary_search_by_key(&tick, |x| x.0) {
            Ok(index) => self.scores[index].1,
            Err(index) => self.scores[index - 1].1,
        }
    }
}

#[derive(Debug)]
pub struct Players {
    players: VecMap<ID, Player>,
}

impl Players {
    pub fn new(players: &Vec<raw::Player>) -> Self {
        Self {
            players: {
                let mut map = VecMap::new();
                for player in players {
                    let player = Player::new(player);
                    map.insert(player.id, player);
                }
                map
            }
        }
    }
    pub fn get_names(&self) -> (&str, &str) {
        (&self.players.get(&1).unwrap().name, &self.players.get(&2).unwrap().name)
    }
    pub fn get_scores(&self, tick: usize) -> (i32, i32) {
        let score1 = self.players.get(&1).unwrap().get_score(tick);
        let score2 = self.players.get(&2).unwrap().get_score(tick);
        (score1, score2)
    }
    pub fn add_tick(&mut self, tick: usize, players: Option<Vec<raw::Player>>) {
        if let Some(players) = players {
            for player in players {
                self.players.get_mut(&player.id).unwrap().add_tick(tick, player);
            }
        }
    }
}