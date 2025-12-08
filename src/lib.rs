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

#[macro_export]
macro_rules! log_time {
    ($e:expr $(,)?) => {{
        let time_before = std::time::Instant::now();
        let result = $e;
        let time_after = std::time::Instant::now();
        macroquad::prelude::info!("{} took {:?}", stringify!($e), time_after - time_before);
        result
    }};
    ($e:expr, $name:expr $(,)?) => {{
        let time_before = std::time::Instant::now();
        let result = $e;
        let time_after = std::time::Instant::now();
        macroquad::prelude::info!("{} took {:?}", $name, time_after - time_before);
        result
    }};
}
