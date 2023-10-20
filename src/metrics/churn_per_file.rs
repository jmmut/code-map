use macroquad::prelude::info;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::git_churn::git_churn;
use crate::AnyError;
use crate::tree::Tree;

pub fn git_churn_per_file(folder: PathBuf) -> Result<Option<Tree>, AnyError> {
    let file_churns = git_churn(folder)?;
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_churn_tree_creation() {}
}
