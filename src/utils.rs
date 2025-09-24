use anyhow::{anyhow, Result};
use std::str::FromStr;

pub fn format_size(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, units[unit_index])
    } else {
        format!("{:.1} {}", size, units[unit_index])
    }
}

pub fn parse_size(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim().to_uppercase();
    
    if let Ok(bytes) = u64::from_str(&size_str) {
        return Ok(bytes);
    }

    let (number_part, unit_part) = if size_str.ends_with("TB") {
        size_str.split_at(size_str.len() - 2)
    } else if size_str.ends_with("GB") {
        size_str.split_at(size_str.len() - 2)
    } else if size_str.ends_with("MB") {
        size_str.split_at(size_str.len() - 2)
    } else if size_str.ends_with("KB") {
        size_str.split_at(size_str.len() - 2)
    } else if size_str.ends_with("B") {
        size_str.split_at(size_str.len() - 1)
    } else {
        return Err(anyhow!("Invalid size format: {}. Use formats like '100MB', '1GB', etc.", size_str));
    };

    let number: f64 = number_part.parse()
        .map_err(|_| anyhow!("Invalid number in size: {}", number_part))?;

    let multiplier = match unit_part {
        "B" => 1,
        "KB" => 1024,
        "MB" => 1024 * 1024,
        "GB" => 1024 * 1024 * 1024,
        "TB" => 1024u64.pow(4),
        _ => return Err(anyhow!("Invalid size unit: {}", unit_part)),
    };

    Ok((number * multiplier as f64) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(1048576), "1.0 MB");
    }

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size("1024").unwrap(), 1024);
        assert_eq!(parse_size("1KB").unwrap(), 1024);
        assert_eq!(parse_size("1.5MB").unwrap(), 1572864);
        assert_eq!(parse_size("2GB").unwrap(), 2147483648);
    }
}