use std::error::Error;
use reqwest::blocking::get;
use serde::Deserialize;
use serde_xml_rs::from_str;
use crate::libs::database;



use crate::models::player::Player;
use crate::traits::Action;

#[derive(Debug)]
pub struct ListPlayers;

#[derive(Debug, Deserialize)]
struct PlayersWrapper {
    #[serde(rename = "Player")]
    players: Vec<XmlPlayer>,
}

#[derive(Debug, Deserialize)]
struct XmlPlayer {
    #[serde(rename = "PlayerID")]
    player_id: u32,
    #[serde(rename = "PlayerName")]
    name: String,
    #[serde(rename = "Age")]
    age: u8,
    #[serde(rename = "AgeDays")]
    age_days: u8,
    #[serde(rename = "TSI")]
    tsi: u32,
    #[serde(rename = "PlayerForm")]
    form: u8,
    #[serde(rename = "StaminaSkill")]
    stamina: u8,
    #[serde(rename = "KeeperSkill")]
    keeper: u8,
    #[serde(rename = "PlaymakerSkill")]
    playmaker: u8,
    #[serde(rename = "ScorerSkill")]
    scorer: u8,
    #[serde(rename = "PassingSkill")]
    passing: u8,
    #[serde(rename = "WingerSkill")]
    winger: u8,
    #[serde(rename = "DefenderSkill")]
    defender: u8,
    #[serde(rename = "SetPiecesSkill")]
    set_pieces: u8,
    #[serde(rename = "Experience")]
    experience: u8,
    #[serde(rename = "Loyalty")]
    loyalty: u8,
    #[serde(rename = "MotherClubBonus")]
    mother_club_bonus: String,
    #[serde(rename = "InjuryLevel")]
    injury_level: u8,
    #[serde(rename = "IsInjured")]
    is_injured: String,
    #[serde(rename = "Specialty")]
    specialty: String,
    #[serde(rename = "Salary")]
    salary: u32,
    #[serde(rename = "IsAbroad")]
    is_abroad: String,
    #[serde(rename = "CountryID")]
    country_id: u32,
    #[serde(rename = "CountryName")]
    country_name: String,
}

impl From<XmlPlayer> for Player {
    fn from(x: XmlPlayer) -> Self {
        Player {
            player_id: x.player_id,
            name: x.name,
            age: x.age,
            age_days: x.age_days,
            tsi: x.tsi,
            form: x.form,
            stamina: x.stamina,
            keeper: x.keeper,
            playmaker: x.playmaker,
            scorer: x.scorer,
            passing: x.passing,
            winger: x.winger,
            defender: x.defender,
            set_pieces: x.set_pieces,
            experience: x.experience,
            loyalty: x.loyalty,
            mother_club_bonus: x.mother_club_bonus == "Yes",
            injury_level: x.injury_level,
            is_injured: x.is_injured == "Yes",
            specialty: if x.specialty == "None" { None } else { Some(x.specialty) },
            salary: x.salary,
            is_abroad: x.is_abroad == "Yes",
            country_id: x.country_id,
            country_name: x.country_name,
        }
    }
}

impl Action for ListPlayers {
    fn render_view(&self) {
        // Aquí puedes implementar la lógica para renderizar la vista
        // utilizando el objeto `content_view`.
        // Por ejemplo, podrías agregar un botón o una etiqueta.
        println!("Renderizando vista en ListPlayers");
    }
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let xml = get("https://custm.es/players.xml")?.text()?;
        let wrapper: PlayersWrapper = from_str(&xml)?;
        let conn = database::get_connection().lock().unwrap();

        conn.execute_batch(r#"
            CREATE TABLE IF NOT EXISTS players (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                age INTEGER,
                age_days INTEGER,
                tsi INTEGER,
                form INTEGER,
                stamina INTEGER,
                keeper INTEGER,
                playmaker INTEGER,
                scorer INTEGER,
                passing INTEGER,
                winger INTEGER,
                defender INTEGER,
                set_pieces INTEGER,
                experience INTEGER,
                loyalty INTEGER,
                mother_club_bonus INTEGER,
                injury_level INTEGER,
                is_injured INTEGER,
                specialty TEXT,
                salary INTEGER,
                is_abroad INTEGER,
                country_id INTEGER,
                country_name TEXT,
                insert_date DATETIME DEFAULT (DATETIME('now', 'localtime'))
            );
        "#)?;

        for xml_player in wrapper.players {
            let player: Player = xml_player.into();
            player.save(&conn)?;
        }
        
        Ok(())
    }
}
