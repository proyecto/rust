use rusqlite::{params, Result};
use crate::libs::database;

#[derive(Debug)]
pub struct NewsItem {
    pub id: i32,
    pub date: String,
    pub category: String,
    pub title: String,
    pub description: String,
}

impl NewsItem {
    fn ensure_table_exists() -> Result<()> {
        let conn = database::get_connection().lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS news_feed (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                date TEXT NOT NULL,
                category TEXT NOT NULL,
                title TEXT NOT NULL,
                description TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn insert(&self) -> Result<()> {
        Self::ensure_table_exists()?;
        let conn = database::get_connection().lock().unwrap();
        conn.execute(
            "INSERT INTO news_feed (date, category, title, description)
                   VALUES (?1, ?2, ?3, ?4)",
            params![self.date, self.category, self.title, self.description],
        )?;
        Ok(())
    }

    pub fn delete(id: i32) -> Result<()> {
        let conn = database::get_connection().lock().unwrap();
        conn.execute("DELETE FROM news_feed WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn get_all(limit: usize) -> Result<Vec<NewsItem>> {
        Self::ensure_table_exists()?;
        let conn = database::get_connection().lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, date, category, title, description
             FROM news_feed ORDER BY date DESC LIMIT ?1"
        )?;
        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(NewsItem {
                id: row.get(0)?,
                date: row.get(1)?,
                category: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
            })
        })?;

        Ok(rows.filter_map(Result::ok).collect())
    }
}