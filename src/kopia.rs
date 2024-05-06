use std::mem;
use serde::Deserialize;
use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Deserialize, Debug)]
pub struct Stats {
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    #[serde(rename = "errorCount")]
    pub error_count: u64,
}

#[derive(Deserialize,Hash, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct Source {
    pub host: String,
    #[serde(rename = "userName")]
    pub user: String,
    pub path: String,
}

impl Source {
    pub fn take(&mut self) -> Self {
        mem::take(self)
    }
}

#[derive(Deserialize, Debug)]
pub struct Backup {
    pub source: Source,
    #[serde(rename = "endTime")]
    pub end_time: DateTime<Utc>,
    pub stats: Stats,
}


pub type Backups = Vec<Backup>;
pub trait BackupsFromJson {
    fn from_json(json_string: &String) -> Backups;
}

impl BackupsFromJson for Backups {
    fn from_json(json_string: &String) -> Backups {
        serde_json::from_str(json_string).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        let user = String::from("testUser");
        let host = String::from("testHost");
        let path = String::from("/testPath");
        let expected_timestamp = 1431648000;
        let end_time = DateTime::from_timestamp(expected_timestamp, 0).unwrap();
        let test_json = format!(r#"[
            {{
                "source": {{"host": "{}", "userName": "{}", "path": "{}"}},
                "endTime": "{}",
                "stats": {{ "totalSize": 123, "errorCount": 531 }}
            }}
        ]"#, host, user, path, end_time.to_rfc3339());
        println!("{:?}", test_json);
        let kopia: Backups = Backups::from_json(&test_json);
        println!("{:?}", kopia);
        assert_eq!(kopia[0].source.host, host);
        assert_eq!(kopia[0].source.user, user);
        assert_eq!(kopia[0].source.path, path);
        assert_eq!(kopia[0].stats.total_size, 123);
        assert_eq!(kopia[0].stats.error_count, 531);
        assert_eq!(kopia[0].end_time, end_time);
    }

    #[test]
    fn test_take() {
        let host = String::from("testHost");
        let source = Source {
            host: host.clone(),
            user: String::from("testUser"),
            path: String::from("testPath"),
        };
        let mut sources: Vec<Source> = vec![source];
        assert_eq!(sources.len(), 1, "Sources vector has 1 object");
        assert_eq!(sources.first().unwrap().host, host, "The source in our vector has the correct host");
        let source_taken = sources[0].take();

        assert_eq!(source_taken.host, host, "After take(), the taken source has the correct host");
        assert_eq!(sources.first().unwrap(), &Source::default(), "After take(), the remaining source is empty");
    }

}
