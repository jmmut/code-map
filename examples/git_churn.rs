use code_map::metrics::churn_per_file::git_churn;
use code_map::AnyError;

fn main() -> Result<(), AnyError> {
    git_churn()
}
