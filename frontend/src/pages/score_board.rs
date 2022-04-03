use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Game {
    pub GameDate: i64,
    pub gameType: String,
    pub gameNumber: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
}


