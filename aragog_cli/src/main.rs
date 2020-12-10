use clap::{load_yaml, App};

pub use config::log;

use crate::config::Config;
use crate::error::MigrationError;
use crate::log_level::LogLevel;
use crate::migration::Migration;
use crate::migration_manager::MigrationManager;
use crate::versioned_database::VersionedDatabase;

mod config;
mod error;
mod log_level;
mod migration;
mod migration_data;
mod migration_manager;
mod migration_operation;
mod versioned_database;

#[derive(Debug)]
pub enum MigrationDirection {
    Up,
    Down(usize),
}

fn migrate(
    direction: MigrationDirection,
    db: &mut VersionedDatabase,
    manager: MigrationManager,
) -> Result<(), MigrationError> {
    match direction {
        MigrationDirection::Up => {
            manager.migrations_up(db)?;
        }
        MigrationDirection::Down(count) => {
            manager.migrations_down(count, db)?;
        }
    };
    Ok(())
}

fn main() -> Result<(), MigrationError> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let config = Config::new(&matches)?;
    let schema_path = config.schema_path.clone();

    let mut db = VersionedDatabase::init(&config)?;

    match matches.subcommand() {
        Some(("check", _args)) => {
            MigrationManager::new(&schema_path)?;
        }
        Some(("migrate", _args)) => {
            let manager = MigrationManager::new(&schema_path)?;
            migrate(MigrationDirection::Up, &mut db, manager)?;
        }
        Some(("rollback", args)) => {
            let manager = MigrationManager::new(&schema_path)?;
            let count = match args.value_of("COUNT").unwrap_or("1").parse() {
                Ok(val) => val,
                Err(_error) => {
                    return Err(MigrationError::InvalidParameter {
                        name: "COUNT".to_string(),
                        message: "Must be a valid number".to_string(),
                    })
                }
            };
            migrate(MigrationDirection::Down(count), &mut db, manager)?;
        }
        Some(("create_migration", args)) => {
            Migration::new(args.value_of("MIGRATION_NAME").unwrap(), &schema_path)?;
        }
        Some(("truncate_database", _args)) => {
            for info in db.accessible_collections()?.iter() {
                if info.is_system {
                    continue;
                }
                log(
                    format!("Dropping Collection {}", &info.name),
                    LogLevel::Info,
                );
                db.drop_collection(&info.name)?;
            }
            log(format!("Truncated database collections."), LogLevel::Info);
        }
        _ => log(format!("No usage found, use --help"), LogLevel::Info),
    };
    Ok(())
}