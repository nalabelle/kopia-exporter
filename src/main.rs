use std::io;
use std::io::Read;

mod stats;
mod kopia;

use crate::kopia::BackupsFromJson;
use crate::stats::BackupsCollect;

fn read_input() -> String {
    let mut buf = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buf).unwrap();
    buf
}

fn main() {
    let input = read_input();
    let mut kopia_backups: kopia::Backups = kopia::Backups::from_json(&input);
    let mut latest: stats::Backups = stats::Backups::new();

    for kopia_backup in kopia_backups.iter_mut() {
        let source = kopia_backup.source.take();
        let details = stats::Stats {
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
    latest.collect(&mut io::stdout());
}
