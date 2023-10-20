use code_map::AnyError;
use code_map::metrics::churn_per_file::print_git_churn;

fn main() -> Result<(), AnyError> {
    print_git_churn()
}
