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
    let mut kopia_backups: kopia::Backups = kopia::Backups::from_json(&read_input());
    let latest: stats::Backups = stats::Backups::from_kopia(&mut kopia_backups);
    latest.collect(&mut io::stdout());
}
