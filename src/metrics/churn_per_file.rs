use git2::{Repository, Tree};
use std::collections::HashMap;

use crate::AnyError;

/// Represents a file and how many times it was changed in the whole git repo history
pub struct FileChurn {
    pub path: String,
    pub count: u32,
}

pub fn print_git_churn() -> Result<(), AnyError> {
    let mut files_and_counts = git_churn()?;
    files_and_counts.sort_by(|a, b| a.count.cmp(&b.count));
    for FileChurn{path, count} in files_and_counts {
        println!("{:>5} {}", count, path);
    }
    Ok(())
}

pub fn git_churn() -> Result<Vec<FileChurn>, AnyError> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut files_changed_count = HashMap::new();

    for oid in revwalk {
        let commit = repo.find_commit(oid?)?;
        let tree = commit.tree()?;

        // I couldn't find any way to list the modified files in a commit without doing
        // an explicit diff with the parent(s). This makes sense if the rumour that git
        // stores the whole tree in each commit is true. This seems to be the case by a quick read
        // of https://git-scm.com/book/en/v2/Git-Internals-Git-Objects. Mindblown.
        for parent in commit.parents() {
            let parent_tree = parent.tree()?;
            add_diff(&tree, Some(&parent_tree), &repo, &mut files_changed_count)?;
        }

        if commit.parent_count() == 0 {
            add_diff(&tree, None, &repo, &mut files_changed_count)?;
        }
    }
    Ok(files_changed_count.into_iter().map(|(p,c) | FileChurn {path: p, count: c}).collect::<Vec<_>>())
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

fn add_file(path: String, files_changed_count: &mut HashMap<String, u32>) {
    let count = files_changed_count.entry(path).or_insert(0);
    *count += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_churn_tree_creation() {}

    #[test]
    #[ignore]
    fn test_print_git_churn() {
        print_git_churn().unwrap();
    }
}
