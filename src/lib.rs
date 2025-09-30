pub mod db;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to open the database")]
    DatabaseError(#[from] rusqlite::Error),
    #[error("Invalid patient ID format. Expected format: XXXXXX XXXAXX where X are digits")]
    InvalidPatientIdFormat,
}

pub type Result<T> = std::result::Result<T, Error>;

pub mod find {
    use crate::{db, Error};
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::path::PathBuf;

    lazy_static! {
        static ref PATIENT_ID_RE: Regex = Regex::new(r"^\d{6}\s\d{3}A\d{2}$").unwrap();
    }
    pub fn by_patient_id(
        conn: &rusqlite::Connection,
        patient_id: &str,
    ) -> crate::Result<Vec<PathBuf>> {
        let patient_id = patient_id.trim();
        if !PATIENT_ID_RE.is_match(&patient_id) {
            return Err(Error::InvalidPatientIdFormat);
        }
        let list = db::path::list(&conn).expect("Failed to list paths");
        let month_day = &patient_id[2..6];
        let pid = patient_id.replace(" ", "_");
        let mut v = Vec::new();
        for path in list {
            let p = PathBuf::from(&path).join(month_day).join(&pid);
            if p.exists() {
                v.push(p);
            }
        }
        Ok(v)
    }
}
