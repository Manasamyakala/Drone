use chrono::Local;
use rusqlite::{params, Connection, Result};

#[derive(Debug, Clone)]
pub struct DroneRecord {
    pub drone_id: String,
    pub drone_type: String,
    pub confidence: f32,
    pub speed: f32,
    pub distance: f32,
    pub direction: String,
    pub threat_level: String,
    pub threat_score: f32,
    pub counter_measure: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Open or create the SQLite database
    pub fn new() -> Result<Self> {
        let conn = Connection::open("database/afv_system.db")?;

        Ok(Self { conn })
    }

    /// Create required tables
    pub fn initialize(&self) -> Result<()> {

        self.conn.execute(
            "
            CREATE TABLE IF NOT EXISTS drone_records (

                id INTEGER PRIMARY KEY AUTOINCREMENT,

                timestamp TEXT NOT NULL,

                drone_id TEXT NOT NULL,

                drone_type TEXT NOT NULL,

                confidence REAL,

                speed REAL,

                distance REAL,

                direction TEXT,

                threat_level TEXT,

                threat_score REAL,

                counter_measure TEXT

            )
            ",
            [],
        )?;

        Ok(())
    }

    /// Store one drone record
    pub fn insert_detection(
        &self,
        record: &DroneRecord,
    ) -> Result<()> {

        let time = Local::now().to_string();

        self.conn.execute(
            "
            INSERT INTO drone_records
            (
                timestamp,
                drone_id,
                drone_type,
                confidence,
                speed,
                distance,
                direction,
                threat_level,
                threat_score,
                counter_measure
            )
            VALUES
            (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
            ",
            params![
                time,
                record.drone_id,
                record.drone_type,
                record.confidence,
                record.speed,
                record.distance,
                record.direction,
                record.threat_level,
                record.threat_score,
                record.counter_measure
            ],
        )?;

        Ok(())
    }

    /// Retrieve all stored detections
    pub fn fetch_all(&self) -> Result<Vec<DroneRecord>> {

        let mut stmt = self.conn.prepare(
            "
            SELECT
                drone_id,
                drone_type,
                confidence,
                speed,
                distance,
                direction,
                threat_level,
                threat_score,
                counter_measure
            FROM drone_records
            ORDER BY id DESC
            "
        )?;

        let rows = stmt.query_map([], |row| {

            Ok(DroneRecord {

                drone_id: row.get(0)?,

                drone_type: row.get(1)?,

                confidence: row.get(2)?,

                speed: row.get(3)?,

                distance: row.get(4)?,

                direction: row.get(5)?,

                threat_level: row.get(6)?,

                threat_score: row.get(7)?,

                counter_measure: row.get(8)?,
            })

        })?;

        let mut drones = Vec::new();

        for drone in rows {
            drones.push(drone?);
        }

        Ok(drones)
    }

    /// Delete all mission history
    pub fn clear_database(&self) -> Result<()> {

        self.conn.execute(
            "DELETE FROM drone_records",
            [],
        )?;

        Ok(())
    }
}