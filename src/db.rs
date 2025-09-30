use rusqlite::Connection;
use tracing::debug;

pub fn connect<P>(path: P) -> crate::Result<Connection>
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();
    debug!("Connecting to database: {:?}", path);
    Ok(Connection::open(path)?)
}

pub fn init<P>(path: P) -> crate::Result<()>
where
    P: AsRef<std::path::Path>,
{
    let path = path.as_ref();

    debug!("Initializing database: {:?}", path);
    let conn = Connection::open(path)?;

    init_migration(&conn)?;
    Ok(())
}

fn init_migration(conn: &Connection) -> crate::Result<()> {
    debug!("Initializing migration");
    init_version(conn)?;
    migrations::init(conn)?;
    Ok(())
}

fn table_exists(conn: &Connection, table_name: &str) -> crate::Result<bool> {
    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?",
        [table_name],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

fn init_version(conn: &Connection) -> crate::Result<()> {
    if !table_exists(conn, "version")? {
        debug!("Creating a version table");
        conn.execute(
            r#"
CREATE TABLE version (version INTEGER NOT NULL)"#,
            [],
        )?;
        conn.execute("INSERT INTO version (version) VALUES (0)", [])?;
    }
    Ok(())
}

fn version(conn: &Connection) -> crate::Result<i64> {
    let version: i64 = conn.query_row("SELECT version FROM version", [], |row| row.get(0))?;
    Ok(version)
}

fn set_version(conn: &Connection, version: i32) -> crate::Result<()> {
    conn.execute("UPDATE version SET version = ?", [version])?;
    Ok(())
}

mod migrations {
    use crate::db::set_version;
    use lazy_static::lazy_static;
    use rusqlite::Connection;
    use std::collections::HashMap;
    use tracing::debug;

    pub type MigrationFn = fn(&Connection) -> crate::Result<()>;

    lazy_static! {
        pub(crate) static ref MIGRATIONS: HashMap<i64, MigrationFn> = {
            let mut m = HashMap::new();
            m.insert(1, v1 as MigrationFn);
            m
        };
    }

    fn v1(conn: &Connection) -> crate::Result<()> {
        conn.execute("CREATE TABLE paths (path TEXT NOT NULL)", [])?;
        set_version(conn, 1)?;
        Ok(())
    }

    pub(crate) fn init(con: &Connection) -> crate::Result<()> {
        debug!("Initializing migrations");
        let mut vers = super::version(con)?;
        loop {
            let next_version = vers + 1;
            if let Some(migration) = MIGRATIONS.get(&next_version) {
                debug!("Running migration v{}", next_version);
                migration(con)?;
                vers = next_version;
            } else {
                break;
            }
        }
        Ok(())
    }
}

pub mod path {
    use rusqlite::Connection;

    pub fn add(conn: &Connection, path: &str) -> crate::Result<()> {
        conn.execute("INSERT INTO paths (path) VALUES (?)", [path])?;
        Ok(())
    }

    pub fn exists(conn: &Connection, path: &str) -> crate::Result<bool> {
        let count: i32 =
            conn.query_row("SELECT COUNT(*) FROM paths WHERE path = ?", [path], |row| {
                row.get(0)
            })?;
        Ok(count > 0)
    }

    pub fn list(conn: &Connection) -> crate::Result<Vec<String>> {
        let mut stmt = conn.prepare("SELECT path FROM paths")?;
        let paths = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(paths)
    }

    pub fn delete(conn: &Connection, path: &str) -> crate::Result<()> {
        conn.execute("DELETE FROM paths WHERE path = ?", [path])?;
        Ok(())
    }
}
