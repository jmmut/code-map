pub type AnyError = Box<dyn std::error::Error>;

pub mod arrangements {
    pub mod binary;
    pub mod linear;
}
pub mod metrics {
    pub mod bytes_per_file;
    pub mod churn_per_file;
    pub mod lines;
    pub mod word_mentions;
}
pub mod tree;
pub mod ui;
