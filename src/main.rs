use clap::{Parser, Subcommand};
use dcm_archive_find::db;
use dirs;
use print_bytes::println_lossy;
use std::fs;
use std::path::PathBuf;
use tracing::Level;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(help = "Patient ID to search for")]
    patient_id: Option<String>,
    #[arg(long, help = "Path to the SQLite database")]
    db: Option<String>,
    #[arg(long, default_value_t = false, help = "Enable logging at INFO level")]
    verbose: bool,
    #[arg(long, default_value_t = false, help = "Enable logging at DEBUG level")]
    debug: bool,
    #[arg(long, default_value_t = false, help = "Enable logging at TRACE level")]
    trace: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a directory path to the database. This path will be used to search for patient records.
    Add {
        #[arg(help = "Directory path")]
        path: String,
    },
    /// Initialize a new database in which the directory archive paths are stored.
    Init,
    /// List all directory paths in the database used to search for patient records.
    List,
    /// Remove a directory path from the database. This path will no longer be used to search for patient records.
    Remove {
        #[arg(help = "Directory path")]
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let level = if cli.trace {
        Level::TRACE
    } else if cli.debug {
        Level::DEBUG
    } else if cli.verbose {
        Level::INFO
    } else {
        Level::WARN
    };
    tracing_subscriber::fmt()
        .with_thread_ids(true)
        .with_target(true)
        .with_max_level(level)
        .init();

    let db_path = match cli.db {
        Some(path) => PathBuf::from(path),
        None => {
            let mut path = dirs::home_dir().expect("Could not find the home directory");
            path.push(".config");
            path.push("dcm_archive_find");
            fs::create_dir_all(&path).expect("Failed to create config directory");
            path.push("db.sqlite");
            path
        }
    };

    let command = cli.command.as_ref().unwrap();
    match command {
        Commands::Add { path } => {
            let conn = db::connect(&db_path).expect("Failed to connect to the database");
            db::path::add(&conn, &path).expect("Failed to add a filepath");
        }
        Commands::Remove { path } => {
            let conn = db::connect(&db_path).expect("Failed to connect to the database");
            db::path::delete(&conn, &path).expect("Failed to remove a filepath");
        }
        Commands::List => {
            let conn = db::connect(&db_path).expect("Failed to connect to the database");
            let list = db::path::list(&conn).expect("Failed to list paths");
            for path in list {
                println!("{}", path);
            }
        }
        Commands::Init => db::init(&db_path).expect("Failed to initialize a database"),
    }

    if let Some(patient_id) = cli.patient_id {
        let conn = db::connect(&db_path).expect("Failed to connect to the database");
        let v = dcm_archive_find::find::by_patient_id(&conn, &patient_id)
            .expect("Unable to find the patient due to an error");
        for p in v {
            println_lossy(&p);
        }
    }
}
