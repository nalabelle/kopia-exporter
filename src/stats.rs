use std::collections::HashMap;
use std::io::Write;
use chrono::DateTime;
use chrono::offset::Utc;

use crate::kopia;

type Source = kopia::Source;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Stats {
    pub end_time: DateTime<Utc>,
    pub error_count: u64,
    pub total_size: u64,
}

pub type Backups = HashMap<Source, Stats>;

pub trait BackupsCollect {
    fn from_kopia(kopia_backups: &mut kopia::Backups) -> Backups;
    fn collect<W: Write>(&self, writer: &mut W);
}

// https://prometheus.io/docs/instrumenting/exposition_formats/#comments-help-text-and-type-information
fn prom_escape(string: &String) -> String {
    string.replace(r#"\"#, r#"\\"#)
}

impl BackupsCollect for Backups {
    fn from_kopia(kopia_backups: &mut kopia::Backups) -> Backups {
        let mut latest: Backups = Backups::new();
        for kopia_backup in kopia_backups.iter_mut() {
            let source = kopia_backup.source.take();
            let details = Stats {
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
        latest
    }

    /// Collects and prints Backups stats in prom metrics format
    fn collect<W: Write>(&self, writer: &mut W) {
        for (source, details) in self.iter() {
            let host = prom_escape(&source.host);
            let user = prom_escape(&source.user);
            let path = prom_escape(&source.path);
            let tags = format!("host=\"{}\",user=\"{}\",path=\"{}\"", host, user, path);
            let end_time_timestamp = details.end_time.timestamp();
            writeln!(writer, "kopia_backup_run{{{}}} {}", tags, end_time_timestamp).unwrap();
            writeln!(writer, "kopia_backup_size{{{}}} {}", tags, details.total_size).unwrap();
            writeln!(writer, "kopia_backup_errors{{{}}} {}", tags, details.error_count).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, BufRead, Seek};

    #[test]
    fn test_collect() {
        let source = Source {
            user: String::from("testUser"),
            host: String::from("testHost"),
            path: String::from(r#"C:\testPath"#),
        };
        let expected_timestamp = 1431648000;
        let end_time = DateTime::from_timestamp(expected_timestamp, 0).unwrap();
        let stats = Stats {
            end_time: end_time,
            error_count: 10,
            total_size: 11,
        };
        let mut latest: Backups = Backups::new();
        latest.insert(source, stats);
        let mut buffer = Cursor::new(Vec::new());
        latest.collect(&mut buffer);
        buffer.rewind().unwrap();
        let lines = buffer.lines().map(|l| l.unwrap());
        let expected_lines = [
            format!(r#"kopia_backup_run{{host="testHost",user="testUser",path="C:\\testPath"}} {}"#, expected_timestamp),
            format!(r#"kopia_backup_size{{host="testHost",user="testUser",path="C:\\testPath"}} {}"#, 11),
            format!(r#"kopia_backup_errors{{host="testHost",user="testUser",path="C:\\testPath"}} {}"#, 10),
        ];
        for (pos, line) in lines.enumerate() {
            assert_eq!(line, expected_lines[pos]);
        }
    }
}
