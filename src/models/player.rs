
use rusqlite::{Connection, Row, Result, params};
use crate::models::model::Model;

#[derive(Debug, Clone)]
pub struct Player {
    pub player_id: u32,
    pub name: String,
    pub age: u8,
    pub age_days: u8,
    pub tsi: u32,
    pub form: u8,
    pub stamina: u8,
    pub keeper: u8,
    pub playmaker: u8,
    pub scorer: u8,
    pub passing: u8,
    pub winger: u8,
    pub defender: u8,
    pub set_pieces: u8,
    pub experience: u8,
    pub loyalty: u8,
    pub mother_club_bonus: bool,
    pub injury_level: u8,
    pub is_injured: bool,
    pub specialty: Option<String>,
    pub salary: u32,
    pub is_abroad: bool,
    pub country_id: u32,
    pub country_name: String,
}

impl Model for Player {
    fn table_name() -> &'static str {
        "players"
    }
}

impl Player {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Player {
            player_id: row.get("player_id")?,
            name: row.get("name")?,
            age: row.get("age")?,
            age_days: row.get("age_days")?,
            tsi: row.get("tsi")?,
            form: row.get("form")?,
            stamina: row.get("stamina")?,
            keeper: row.get("keeper")?,
            playmaker: row.get("playmaker")?,
            scorer: row.get("scorer")?,
            passing: row.get("passing")?,
            winger: row.get("winger")?,
            defender: row.get("defender")?,
            set_pieces: row.get("set_pieces")?,
            experience: row.get("experience")?,
            loyalty: row.get("loyalty")?,
            mother_club_bonus: row.get::<_, i32>("mother_club_bonus")? != 0,
            injury_level: row.get("injury_level")?,
            is_injured: row.get::<_, i32>("is_injured")? != 0,
            specialty: {
                let s: String = row.get("specialty")?;
                if s.is_empty() { None } else { Some(s) }
            },
            salary: row.get("salary")?,
            is_abroad: row.get::<_, i32>("is_abroad")? != 0,
            country_id: row.get("country_id")?,
            country_name: row.get("country_name")?,
        })
    }

    pub fn save(&self, conn: &Connection, week: u32) -> Result<()> {
        let fecha = chrono::Utc::now().to_rfc3339();
    
        conn.execute(
            "INSERT OR REPLACE INTO players (
                player_id, name, age, age_days, tsi, form,
                stamina, keeper, playmaker, scorer, passing, winger, defender, set_pieces,
                experience, loyalty, mother_club_bonus, injury_level, is_injured, specialty,
                salary, is_abroad, country_id, country_name, week, fecha
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )",
            params![
                self.player_id, self.name, self.age, self.age_days, self.tsi, self.form,
                self.stamina, self.keeper, self.playmaker, self.scorer, self.passing, self.winger, self.defender, self.set_pieces,
                self.experience, self.loyalty, self.mother_club_bonus as i32, self.injury_level, self.is_injured as i32,
                self.specialty.clone().unwrap_or_default(),
                self.salary, self.is_abroad as i32, self.country_id, self.country_name, week, fecha
            ],
        )?;
        Ok(())
    }
}
