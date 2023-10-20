use clap::builder::PossibleValue;
use clap::ValueEnum;

pub mod bytes_per_file;
pub mod churn_per_file;
pub mod lines;
pub mod word_mentions;

#[derive(Copy, Clone, Debug)]
pub enum Metrics {
    BytesPerFile,
    ChurnPerFile,
    LinesPerFile,
    WordMentions,
}

const METRICS: [Metrics; 4] = [
    Metrics::BytesPerFile,
    Metrics::ChurnPerFile,
    Metrics::LinesPerFile,
    Metrics::WordMentions,
];

impl ValueEnum for Metrics {
    fn value_variants<'a>() -> &'a [Self] {
        &METRICS
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Metrics::BytesPerFile => Some(PossibleValue::new("bytes-per-file").alias("b")),
            Metrics::ChurnPerFile => Some(PossibleValue::new("churn-per-file").alias("c")),
            Metrics::LinesPerFile => Some(PossibleValue::new("lines-per-file").alias("l")),
            Metrics::WordMentions => Some(PossibleValue::new("word-mentions").alias("w")),
        }
    }
}
