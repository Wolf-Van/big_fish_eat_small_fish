use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRecord {
    pub id: u32,
    pub score: i32,
    pub timestamp: DateTime<Local>,
    pub player_size: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameDatabase {
    pub records: Vec<GameRecord>,
    pub next_id: u32,
}

impl GameDatabase {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            next_id: 1,
        }
    }

    pub fn load() -> Self {
        let db_path = "game_records.json";
        if Path::new(db_path).exists() {
            match fs::read_to_string(db_path) {
                Ok(content) => {
                    match serde_json::from_str::<GameDatabase>(&content) {
                        Ok(db) => db,
                        Err(_) => Self::new(),
                    }
                }
                Err(_) => Self::new(),
            }
        } else {
            Self::new()
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let db_path = "game_records.json";
        let content = serde_json::to_string_pretty(self)?;
        fs::write(db_path, content)?;
        Ok(())
    }

    pub fn add_record(&mut self, score: i32, player_size: f32) {
        let record = GameRecord {
            id: self.next_id,
            score,
            timestamp: Local::now(),
            player_size,
        };
        self.records.push(record);
        self.next_id += 1;
    }

    pub fn delete_record(&mut self, id: u32) -> bool {
        if let Some(pos) = self.records.iter().position(|r| r.id == id) {
            self.records.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_records(&self) -> &Vec<GameRecord> {
        &self.records
    }

}
