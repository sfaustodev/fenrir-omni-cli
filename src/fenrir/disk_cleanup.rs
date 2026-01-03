use anyhow::Result;
use chrono::{DateTime, Local};
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::{WalkDir, DirEntry};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CleanupCategory {
    Cache,
    Temp,
    Logs,
    Dev,
    System,
}

#[derive(Debug, Clone)]
pub struct CleanupTarget {
    pub path: PathBuf,
    pub size: u64,
    pub category: CleanupCategory,
    pub description: String,
    pub last_accessed: Option<SystemTime>,
}

#[derive(Debug, Clone)]
pub struct CleanupReport {
    pub total_size: u64,
    pub file_count: u64,
    pub by_category: HashMap<CleanupCategory, u64>,
    pub targets: Vec<CleanupTarget>,
}

pub struct CleanupConfig {
    pub aggressive_mode: bool,
    pub min_file_size_mb: u64,
    pub old_file_days: u64,
    pub dry_run: bool,
    pub exclusions: Vec<PathBuf>,
    pub exclude_patterns: Vec<String>,
}

impl Default for CleanupConfig {
    fn default() -> Self {
        Self {
            aggressive_mode: true,
            min_file_size_mb: 100,
            old_file_days: 90,
            dry_run: true,
            exclusions: vec![
                dirs::home_dir().unwrap().join("Projects"),
                dirs::home_dir().unwrap().join("Documents"),
            ],
            exclude_patterns: vec![
                String::from("*.important"),
                String::from("keep_*"),
                String::from("*.active"),
            ],
        }
    }
}

pub fn analyze_disk_usage(path: Option<&str>, detailed: bool) -> Result<CleanupReport> {
    let base_path = path.unwrap_or(".");
    let home = dirs::home_dir().unwrap_or_default();

    println!("🐺 {} Scanning disk usage in {}", "[ANALYZE]".cyan(), base_path);

    let mut targets = Vec::new();
    let mut _by_category: HashMap<CleanupCategory, u64> = HashMap::new();

    // Scan common cache directories
    let cache_dirs = get_cache_directories(&home);
    for cache_dir in cache_dirs {
        if cache_dir.exists() {
            scan_directory(&cache_dir, CleanupCategory::Cache, &mut targets, detailed)?;
        }
    }

    // Scan temp directories
    let temp_dirs = get_temp_directories(&home);
    for temp_dir in temp_dirs {
        if temp_dir.exists() {
            scan_directory(&temp_dir, CleanupCategory::Temp, &mut targets, detailed)?;
        }
    }

    // Scan log directories
    let log_dirs = get_log_directories(&home);
    for log_dir in log_dirs {
        if log_dir.exists() {
            scan_directory(&log_dir, CleanupCategory::Logs, &mut targets, detailed)?;
        }
    }

    // Scan dev cache directories
    let dev_dirs = get_dev_cache_directories(&home);
    for dev_dir in dev_dirs {
        if dev_dir.exists() {
            scan_directory(&dev_dir, CleanupCategory::Dev, &mut targets, detailed)?;
        }
    }

    // Calculate totals
    let mut total_size = 0u64;
    let mut category_totals: HashMap<CleanupCategory, u64> = HashMap::new();

    for target in &targets {
        total_size += target.size;
        *category_totals.entry(target.category.clone()).or_insert(0) += target.size;
    }

    let file_count = targets.len() as u64;

    let report = CleanupReport {
        total_size,
        file_count,
        by_category: category_totals,
        targets,
    };

    Ok(report)
}

pub fn clean_disk(
    config: &CleanupConfig,
    path: Option<&str>,
) -> Result<CleanupReport> {
    let base_path = path.unwrap_or(".");
    let _home = dirs::home_dir().unwrap_or_default();

    let mode_str = if config.dry_run { "DRY RUN " } else { "CLEAN " };
    println!("🐺 {} Disk cleanup in {} mode", mode_str.yellow(), base_path);

    let report = analyze_disk_usage(Some(base_path), false)?;

    if config.dry_run {
        println!("\n📊 {} Cleanup Summary (dry-run, no files deleted):", "[PREVIEW]".cyan());
    } else {
        println!("\n🧹 {} Cleaning up disk space...", "[CLEANING]".green());
    }

    // Filter targets based on config
    let mut to_delete = Vec::new();
    for target in &report.targets {
        if should_clean(target, config) {
            to_delete.push(target.clone());
        }
    }

    // Show what would be cleaned
    println!("\n🎯 {} Found {} items to clean:", "[TARGETS]".yellow(), to_delete.len());
    for target in &to_delete {
        let size_str = format_size(target.size);
        println!(
            "  {} {} - {} ({})",
            "[DELETE]".red(),
            target.path.display(),
            target.description,
            size_str
        );

        if !config.dry_run {
            // Delete file/directory
            if target.path.is_dir() {
                fs::remove_dir_all(&target.path)?;
            } else {
                fs::remove_file(&target.path)?;
            }
        }
    }

    // Calculate space that would be freed
    let total_freed: u64 = to_delete.iter().map(|t| t.size).sum();
    let freed_str = format_size(total_freed);

    if config.dry_run {
        println!("\n✅ {} Would free up {}", "[DRY RUN]".cyan(), freed_str);
    } else {
        println!("\n✅ {} Freed up {}", "[SUCCESS]".green(), freed_str);
    }

    Ok(report)
}

pub fn list_categories(min_size: Option<u64>) -> Result<()> {
    let home = dirs::home_dir().unwrap_or_default();
    let min_size = min_size.unwrap_or(1024 * 1024); // Default 1MB

    println!("🐺 {} Listing cleanup targets (> {}):", "[SCAN]".cyan(), format_size(min_size));

    let mut targets = Vec::new();

    // Scan all categories
    for (dirs, category) in [
        (get_cache_directories(&home), CleanupCategory::Cache),
        (get_temp_directories(&home), CleanupCategory::Temp),
        (get_log_directories(&home), CleanupCategory::Logs),
        (get_dev_cache_directories(&home), CleanupCategory::Dev),
    ] {
        for dir in dirs {
            if dir.exists() {
                scan_directory(&dir, category.clone(), &mut targets, true)?;
            }
        }
    }

    // Filter by size and sort
    targets.retain(|t| t.size >= min_size);
    targets.sort_by(|a, b| b.size.cmp(&a.size));

    // Display top 20
    println!("\n🎯 {} Top {} largest items:", "[RESULTS]".green(), targets.len().min(20));
    for (i, target) in targets.iter().take(20).enumerate() {
        let size_str = format_size(target.size);
        let category_str = format!("{:?}", target.category);
        println!(
            "  {}. {} {} {} - {}",
            i + 1,
            size_str.bold(),
            category_str.cyan(),
            target.description.dimmed(),
            target.path.display().to_string().bright_black()
        );
    }

    Ok(())
}

fn scan_directory(
    dir: &Path,
    category: CleanupCategory,
    targets: &mut Vec<CleanupTarget>,
    detailed: bool,
) -> Result<()> {
    let walker = WalkDir::new(dir)
        .max_depth(if detailed { 10 } else { 3 })
        .into_iter();

    for entry in walker.filter_map(|e| e.ok()) {
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let size = metadata.len();
        let path = entry.path().to_path_buf();

        let target = CleanupTarget {
            path,
            size,
            category: category.clone(),
            description: format_description(&entry, &category),
            last_accessed: metadata.accessed().ok(),
        };

        targets.push(target);
    }

    Ok(())
}

fn should_clean(target: &CleanupTarget, config: &CleanupConfig) -> bool {
    // Check exclusions
    for exclusion in &config.exclusions {
        if target.path.starts_with(exclusion) {
            return false;
        }
    }

    // Check patterns
    for pattern in &config.exclude_patterns {
        if target.path.to_string_lossy().contains(pattern.replace('*', "").as_str()) {
            return false;
        }
    }

    match target.category {
        CleanupCategory::Cache | CleanupCategory::Temp | CleanupCategory::Logs => true,
        CleanupCategory::Dev | CleanupCategory::System => {
            if !config.aggressive_mode {
                return false;
            }

            // Check file age for aggressive mode
            if let Some(last_accessed) = target.last_accessed {
                let datetime: DateTime<Local> = last_accessed.into();
                let now: DateTime<Local> = Local::now();
                let days_old = (now - datetime).num_days();
                return days_old > config.old_file_days as i64
                    && target.size > config.min_file_size_mb * 1024 * 1024;
            }

            false
        }
    }
}

fn get_cache_directories(home: &Path) -> Vec<PathBuf> {
    vec![
        home.join("Library/Caches"),
        home.join("Library/Caches/pip"),
        home.join("Library/Caches/yarn"),
        home.join("Library/Caches/npm"),
        home.join("Library/Caches/Homebrew"),
        home.join(".npm"),
        home.join(".cache"),
        home.join(".cargo/registry"),
        home.join(".rustup"),
        home.join(".gradle/caches"),
        home.join(".m2/repository"),
        home.join(".ivy2/cache"),
        PathBuf::from("/Library/Caches"),
    ]
}

fn get_temp_directories(home: &Path) -> Vec<PathBuf> {
    vec![
        PathBuf::from("/tmp"),
        PathBuf::from("/var/tmp"),
        home.join(".Trash"),
        home.join("Library/Application Support/com.apple.sharedfilelist/com.apple.LSSharedFileList.ApplicationRecentDocuments"),
    ]
}

fn get_log_directories(home: &Path) -> Vec<PathBuf> {
    vec![
        PathBuf::from("/var/log"),
        home.join("Library/Logs"),
        PathBuf::from("/Library/Logs"),
        home.join(".npm/_logs"),
        home.join(".npm/_cacache/index-v5"),
    ]
}

fn get_dev_cache_directories(home: &Path) -> Vec<PathBuf> {
    vec![
        home.join("node_modules"),
        home.join("target"),
        home.join(".build"),
        home.join("dist"),
        home.join("build"),
        home.join(".next/cache"),
        home.join(".nuxt"),
        home.join(".turbo"),
        home.join(".swc"),
    ]
}

fn format_description(_entry: &DirEntry, category: &CleanupCategory) -> String {
    match category {
        CleanupCategory::Cache => String::from("Cache directory"),
        CleanupCategory::Temp => String::from("Temporary file/folder"),
        CleanupCategory::Logs => String::from("Log file"),
        CleanupCategory::Dev => String::from("Build artifact"),
        CleanupCategory::System => String::from("System file"),
    }
}

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB ", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB ", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB ", bytes as f64 / KB as f64)
    } else {
        format!("{} B ", bytes)
    }
}
