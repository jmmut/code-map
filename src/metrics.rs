pub mod bytes_per_file;
pub mod churn_per_file;
pub mod lines;
pub mod word_mentions;

#[derive(Copy, Clone)]
pub enum Metrics {
    BytesPerFile,
    ChurnPerFile,
    LinesPerFile,
    WordMentions,
}

const METRIC_NAMES: [(Metrics, &str); 8] = [
    (Metrics::BytesPerFile, "bytes-per-file"),
    (Metrics::BytesPerFile, "b"),
    (Metrics::ChurnPerFile, "churn-per-file"),
    (Metrics::ChurnPerFile, "c"),
    (Metrics::LinesPerFile, "lines-per-file"),
    (Metrics::LinesPerFile, "l"),
    (Metrics::WordMentions, "word-mentions"),
    (Metrics::WordMentions, "w"),
];

impl Metrics {
    pub fn from_str(s: &str) -> Option<Self> {
        for (metric, name) in METRIC_NAMES.iter() {
            if s == *name {
                return Some(*metric);
            }
        }
        return None;
    }
    pub fn metric_names() -> Vec<&'static str> {
        let mut names = Vec::new();
        for (_, name) in METRIC_NAMES.iter() {
            names.push(*name);
        }
        names
    }
}
