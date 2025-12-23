use crate::{EditorError, Result};
use git2::{Repository, RepositoryInitOptions};
use std::fs;
use std::path::Path;

use super::repository::GitHistoryManager;

impl GitHistoryManager {
    pub fn export_history(&self, project_path: &Path, export_path: &Path) -> Result<()> {
        let source_repo = self.open_repository(project_path)?;

        if export_path.exists() {
            return Err(EditorError::InvalidOperation(format!(
                "Export path {} already exists",
                export_path.display()
            )));
        }

        fs::create_dir_all(export_path).map_err(EditorError::Io)?;

        let mut init_opts = RepositoryInitOptions::new();
        init_opts.bare(false);
        let dest_repo = Repository::init_opts(export_path, &init_opts)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let mut source_walk = source_repo
            .revwalk()
            .map_err(|e| EditorError::Git(e.to_string()))?;
        source_walk
            .push_head()
            .map_err(|e| EditorError::Git(e.to_string()))?;
        source_walk
            .set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let commits: Vec<_> = source_walk
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| EditorError::Git(e.to_string()))?;

        for commit_oid in commits {
            let source_commit = source_repo
                .find_commit(commit_oid)
                .map_err(|e| EditorError::Git(e.to_string()))?;

            let source_tree = source_commit
                .tree()
                .map_err(|e| EditorError::Git(e.to_string()))?;

            let mut dest_index = dest_repo
                .index()
                .map_err(|e| EditorError::Git(e.to_string()))?;

            dest_index
                .clear()
                .map_err(|e| EditorError::Git(e.to_string()))?;

            self.copy_tree_to_index(&source_repo, &dest_repo, &source_tree, &mut dest_index, "")?;

            dest_index
                .write()
                .map_err(|e| EditorError::Git(e.to_string()))?;

            let tree_oid = dest_index
                .write_tree()
                .map_err(|e| EditorError::Git(e.to_string()))?;
            let tree = dest_repo
                .find_tree(tree_oid)
                .map_err(|e| EditorError::Git(e.to_string()))?;

            let signature = source_commit.author();

            let parent_commits: Vec<_> = if dest_repo
                .is_empty()
                .map_err(|e| EditorError::Git(e.to_string()))?
            {
                vec![]
            } else {
                let head = dest_repo
                    .head()
                    .map_err(|e| EditorError::Git(e.to_string()))?;
                let head_commit = head
                    .peel_to_commit()
                    .map_err(|e| EditorError::Git(e.to_string()))?;
                vec![head_commit]
            };

            let parent_refs: Vec<&git2::Commit> = parent_commits.iter().collect();

            dest_repo
                .commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    source_commit.message().unwrap_or(""),
                    &tree,
                    &parent_refs,
                )
                .map_err(|e| EditorError::Git(e.to_string()))?;
        }

        dest_repo
            .checkout_head(Some(git2::build::CheckoutBuilder::new().force()))
            .map_err(|e| EditorError::Git(e.to_string()))?;

        Ok(())
    }

    pub fn import_history(&self, project_path: &Path, import_path: &Path) -> Result<()> {
        if !import_path.exists() {
            return Err(EditorError::InvalidOperation(format!(
                "Import path {} does not exist",
                import_path.display()
            )));
        }

        let source_repo =
            Repository::open(import_path).map_err(|e| EditorError::Git(e.to_string()))?;

        let dest_repo = self.open_repository(project_path)?;

        let mut source_walk = source_repo
            .revwalk()
            .map_err(|e| EditorError::Git(e.to_string()))?;
        source_walk
            .push_head()
            .map_err(|e| EditorError::Git(e.to_string()))?;
        source_walk
            .set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let commits: Vec<_> = source_walk
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| EditorError::Git(e.to_string()))?;

        for commit_oid in commits {
            let source_commit = source_repo
                .find_commit(commit_oid)
                .map_err(|e| EditorError::Git(e.to_string()))?;

            let source_tree = source_commit
                .tree()
                .map_err(|e| EditorError::Git(e.to_string()))?;

            let mut dest_index = dest_repo
                .index()
                .map_err(|e| EditorError::Git(e.to_string()))?;

            dest_index
                .clear()
                .map_err(|e| EditorError::Git(e.to_string()))?;

            self.copy_tree_to_index(&source_repo, &dest_repo, &source_tree, &mut dest_index, "")?;

            dest_index
                .write()
                .map_err(|e| EditorError::Git(e.to_string()))?;

            let tree_oid = dest_index
                .write_tree()
                .map_err(|e| EditorError::Git(e.to_string()))?;
            let tree = dest_repo
                .find_tree(tree_oid)
                .map_err(|e| EditorError::Git(e.to_string()))?;

            let signature = source_commit.author();

            let parent_commits: Vec<_> = if dest_repo
                .is_empty()
                .map_err(|e| EditorError::Git(e.to_string()))?
            {
                vec![]
            } else {
                let head = dest_repo
                    .head()
                    .map_err(|e| EditorError::Git(e.to_string()))?;
                let head_commit = head
                    .peel_to_commit()
                    .map_err(|e| EditorError::Git(e.to_string()))?;
                vec![head_commit]
            };

            let parent_refs: Vec<&git2::Commit> = parent_commits.iter().collect();

            dest_repo
                .commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    source_commit.message().unwrap_or(""),
                    &tree,
                    &parent_refs,
                )
                .map_err(|e| EditorError::Git(e.to_string()))?;
        }

        Ok(())
    }

    fn copy_tree_to_index(
        &self,
        source_repo: &Repository,
        dest_repo: &Repository,
        tree: &git2::Tree,
        index: &mut git2::Index,
        prefix: &str,
    ) -> Result<()> {
        for entry in tree.iter() {
            let name = entry.name().ok_or_else(|| {
                EditorError::InvalidOperation("Invalid tree entry name".to_string())
            })?;
            let path = if prefix.is_empty() {
                name.to_string()
            } else {
                format!("{}/{}", prefix, name)
            };

            if let Some(git2::ObjectType::Tree) = entry.kind() {
                let subtree = source_repo
                    .find_tree(entry.id())
                    .map_err(|e| EditorError::Git(e.to_string()))?;
                self.copy_tree_to_index(source_repo, dest_repo, &subtree, index, &path)?;
            } else {
                let source_blob = source_repo
                    .find_blob(entry.id())
                    .map_err(|e| EditorError::Git(e.to_string()))?;

                let dest_oid = dest_repo
                    .blob(source_blob.content())
                    .map_err(|e| EditorError::Git(e.to_string()))?;

                let index_entry = git2::IndexEntry {
                    ctime: git2::IndexTime::new(0, 0),
                    mtime: git2::IndexTime::new(0, 0),
                    dev: 0,
                    ino: 0,
                    mode: entry.filemode() as u32,
                    uid: 0,
                    gid: 0,
                    file_size: source_blob.size() as u32,
                    id: dest_oid,
                    flags: 0,
                    flags_extended: 0,
                    path: path.as_bytes().to_vec(),
                };

                index
                    .add(&index_entry)
                    .map_err(|e| EditorError::Git(e.to_string()))?;
            }
        }

        Ok(())
    }
}
