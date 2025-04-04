pub type AnyError = Box<dyn std::error::Error>;

pub mod arrangements {
    pub mod binary;
    pub mod golden;
    pub mod linear;
}
pub mod git_churn;
pub mod metrics;
pub mod tree;
pub mod ui;
