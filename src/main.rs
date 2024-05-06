use std::io;
use std::collections::HashMap;
use std::io::Read;
use std::mem;
use serde::Deserialize;
use chrono::DateTime;
use chrono::offset::Utc;


#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct BackupStats {
    end_time: DateTime<Utc>,
    error_count: u64,
    total_size: u64,
}

type Backups = HashMap<Source, BackupStats>;

#[derive(Deserialize)]
struct KopiaStats {
    #[serde(rename = "totalSize")]
    total_size: u64,
    #[serde(rename = "errorCount")]
    error_count: u64,
}

#[derive(Deserialize,Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Source {
    host: String,
    #[serde(rename = "userName")]
    user: String,
    path: String,
}

impl Source {
    fn take(&mut self) -> Self {
        mem::take(self)
    }
}

#[derive(Deserialize)]
struct KopiaBackup {
    source: Source,
    #[serde(rename = "endTime")]
    end_time: DateTime<Utc>,
    stats: KopiaStats,
}

fn read_input() -> String {
    let mut buf = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buf).unwrap();
    buf
}

fn collect(backups: &Backups) {
    for (source, details) in backups.iter() {
        let tags = format!("host=\"{}\",user=\"{}\",path=\"{}\"", source.host, source.user, source.path);
        println!("kopia_backup_run{{{}}} {}", tags, details.end_time);
        println!("kopia_backup_size{{{}}} {}", tags, details.total_size);
        println!("kopia_backup_errors{{{}}} {}", tags, details.error_count);
    }
}

fn main() {
    let input = read_input();
    let mut kopia_backups: Vec<KopiaBackup> = serde_json::from_str(&input).unwrap();
    let mut latest: Backups = HashMap::new();

    for kopia_backup in kopia_backups.iter_mut() {
        let source = kopia_backup.source.take();
        let details = BackupStats {
            end_time: kopia_backup.end_time,
            error_count: kopia_backup.stats.error_count,
            total_size: kopia_backup.stats.total_size,
        };
        match latest.get(&source) {
            Some(stored_details) => {
                if &details > stored_details {
                    latest.insert(source, details);
                }
            },
            None => {
                latest.insert(source, details);
            }
        }
    }
    collect(&latest);
}
