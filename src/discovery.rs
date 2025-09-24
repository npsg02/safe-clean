use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::task;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FileItem {
    pub path: PathBuf,
    pub size: u64,
    pub item_count: Option<usize>,
    pub is_dir: bool,
}

pub struct DirAnalyzer;

impl DirAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub async fn analyze_directory(&self, path: &Path, include_subdirs: bool) -> Result<Vec<FileItem>> {
        let path = path.to_owned();
        
        task::spawn_blocking(move || {
            let mut items = Vec::new();
            
            if !path.exists() {
                return Ok(items);
            }

            for entry in WalkDir::new(&path)
                .max_depth(if include_subdirs { 1 } else { 0 })
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.path() == path {
                    continue;
                }

                let metadata = match entry.metadata() {
                    Ok(meta) => meta,
                    Err(_) => continue,
                };

                let size = if metadata.is_dir() {
                    calculate_dir_size(entry.path())?
                } else {
                    metadata.len()
                };

                let item_count = if metadata.is_dir() {
                    Some(count_items(entry.path())?)
                } else {
                    None
                };

                items.push(FileItem {
                    path: entry.path().to_owned(),
                    size,
                    item_count,
                    is_dir: metadata.is_dir(),
                });
            }

            // Sort by size (largest first)
            items.sort_by(|a, b| b.size.cmp(&a.size));
            Ok(items)
        }).await?
    }
}

fn calculate_dir_size(path: &Path) -> Result<u64> {
    let mut total_size = 0;
    
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                total_size += metadata.len();
            }
        }
    }
    
    Ok(total_size)
}

fn count_items(path: &Path) -> Result<usize> {
    Ok(WalkDir::new(path).into_iter().count().saturating_sub(1)) // Subtract 1 for the root directory
}

pub struct LargeFileFinder;

impl LargeFileFinder {
    pub fn new() -> Self {
        Self
    }

    pub async fn find_large_files(&self, path: &Path, min_size: u64) -> Result<Vec<FileItem>> {
        let path = path.to_owned();
        
        task::spawn_blocking(move || {
            let mut large_files = Vec::new();
            
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() && metadata.len() >= min_size {
                        large_files.push(FileItem {
                            path: entry.path().to_owned(),
                            size: metadata.len(),
                            item_count: None,
                            is_dir: false,
                        });
                    }
                }
            }

            // Sort by size (largest first)
            large_files.sort_by(|a, b| b.size.cmp(&a.size));
            Ok(large_files)
        }).await?
    }
}

pub struct DevArtifactFinder;

impl DevArtifactFinder {
    pub fn new() -> Self {
        Self
    }

    pub async fn find_artifacts(&self, path: &Path) -> Result<Vec<FileItem>> {
        let path = path.to_owned();
        
        task::spawn_blocking(move || {
            let mut artifacts = Vec::new();
            let target_dirs = ["node_modules", ".venv", "venv", "__pycache__", ".tox", "target", "build", "dist"];
            
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(dir_name) = entry.path().file_name() {
                            if let Some(name_str) = dir_name.to_str() {
                                if target_dirs.contains(&name_str) {
                                    let size = calculate_dir_size(entry.path())?;
                                    let item_count = count_items(entry.path())?;
                                    
                                    artifacts.push(FileItem {
                                        path: entry.path().to_owned(),
                                        size,
                                        item_count: Some(item_count),
                                        is_dir: true,
                                    });
                                }
                            }
                        }
                    }
                }
            }

            // Sort by size (largest first)
            artifacts.sort_by(|a, b| b.size.cmp(&a.size));
            Ok(artifacts)
        }).await?
    }
}