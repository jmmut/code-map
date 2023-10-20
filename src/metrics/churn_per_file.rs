use git2::{Repository, Tree};
use std::collections::HashMap;

use crate::AnyError;

pub fn git_churn() -> Result<(), AnyError> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut files_changed_count = HashMap::new();
    fn add_file(path: String, files_changed_count: &mut HashMap<String, u32>) {
        let count = files_changed_count.entry(path).or_insert(0);
        *count += 1;
    }

    fn add_diff(
        commit_tree: &Tree,
        parent_tree: Option<&Tree>,
        repo: &Repository,
        files_changed_count: &mut HashMap<String, u32>,
    ) -> Result<(), AnyError> {
        let mut diff = repo.diff_tree_to_tree(parent_tree, Some(commit_tree), None)?;
        diff.find_similar(None)?;
        for delta in diff.deltas() {
            let new_file = delta.new_file();
            let bytes = new_file.path_bytes().unwrap();
            let path_string = String::from_utf8(bytes.to_vec())?;
            add_file(path_string, files_changed_count);
        }
        Ok(())
    }

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let tree = commit.tree()?;

        for parent in commit.parents() {
            let parent_tree = parent.tree()?;
            add_diff(&tree, Some(&parent_tree), &repo, &mut files_changed_count)?;
        }

        if commit.parent_count() == 0 {
            add_diff(&tree, None, &repo, &mut files_changed_count)?;
        }
    }

    let mut sorted_files = files_changed_count.iter().collect::<Vec<_>>();
    sorted_files.sort_by(|a, b| a.1.cmp(b.1));
    for (path, count) in sorted_files {
        println!("{} {}", path, count);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_churn_tree_creation() {}

    #[test]
    fn test_print_git_churn() {
        git_churn().unwrap();
    }
}
