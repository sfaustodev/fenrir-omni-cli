//! # Validation Module
//!
//! Validadores customizados para parâmetros CLI usando callbacks do clap.

use clap::builder::{StringValueParser, TypedValueParser};
use std::net::{IpAddr, Ipv4Addr};
use colored::Colorize;

/// Validador para alvos de scan (IP, hostname, URL)
pub fn target_validator() -> impl TypedValueParser<Value = String> {
    StringValueParser::new().try_map(|s| {
        if is_valid_target(&s) {
            Ok(s)
        } else {
            Err(format!(
                "❌ Invalid target '{}'. Expected: IP address (192.168.1.1), hostname (example.com), or URL (https://example.com)",
                s.red()
            ))
        }
    })
}

/// Validador para range de portas
pub fn port_range_validator() -> impl TypedValueParser<Value = String> {
    StringValueParser::new().try_map(|s| {
        if is_valid_port_range(&s) {
            Ok(s)
        } else {
            Err(format!(
                "❌ Invalid port range '{}'. Expected: single port (80), comma-separated (80,443,8080), or range (1-1000)",
                s.red()
            ))
        }
    })
}

/// Validador para tipo de scan
pub fn scan_type_validator() -> impl TypedValueParser<Value = String> {
    StringValueParser::new().try_map(|s| {
        if is_valid_scan_type(&s) {
            Ok(s)
        } else {
            Err(format!(
                "❌ Invalid scan type '{}'. Valid types: quick, full, stealth",
                s.red()
            ))
        }
    })
}

/// Verifica se o target é válido
fn is_valid_target(target: &str) -> bool {
    // URL validation
    if target.starts_with("http://") || target.starts_with("https://") {
        return url::Url::parse(target).is_ok();
    }

    // IPv4 validation
    if target.parse::<Ipv4Addr>().is_ok() {
        return true;
    }

    // IPv6 validation
    if target.parse::<IpAddr>().is_ok() {
        return true;
    }

    // Hostname validation (RFC 1035)
    is_valid_hostname(target)
}

/// Verifica se hostname é válido
fn is_valid_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.len() > 253 {
        return false;
    }

    let parts: Vec<&str> = hostname.split('.').collect();

    // Deve ter pelo menos 2 partes (ex: example.com)
    if parts.len() < 2 {
        return false;
    }

    for part in parts {
        // Cada label deve ter entre 1-63 caracteres
        if part.is_empty() || part.len() > 63 {
            return false;
        }

        // Não pode começar ou terminar com hífen
        if part.starts_with('-') || part.ends_with('-') {
            return false;
        }

        // Apenas caracteres alfanuméricos e hífen
        if !part.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return false;
        }
    }

    true
}

/// Verifica se range de portas é válido
fn is_valid_port_range(port_range: &str) -> bool {
    if port_range.is_empty() {
        return false;
    }

    for part in port_range.split(',') {
        let part = part.trim();

        if part.contains('-') {
            // Range validation (80-443)
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                return false;
            }

            let start: Result<u16, _> = range_parts[0].parse();
            let end: Result<u16, _> = range_parts[1].parse();

            if let (Ok(start), Ok(end)) = (start, end) {
                if start == 0 || end == 0 || start > end {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            // Single port validation
            if let Ok(port) = part.parse::<u16>() {
                if port == 0 {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    true
}

/// Verifica se tipo de scan é válido
fn is_valid_scan_type(scan_type: &str) -> bool {
    matches!(scan_type.to_lowercase().as_str(), "quick" | "full" | "stealth")
}

/// Verifica se timeout é válido
pub fn is_valid_timeout(timeout: u32) -> bool {
    timeout >= 1 && timeout <= 300 // 1-300 segundos
}

/// Verifica se número de threads é válido
pub fn is_valid_threads(threads: u32) -> bool {
    threads >= 1 && threads <= 1000 // 1-1000 threads
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_targets() {
        assert!(is_valid_target("192.168.1.1"));
        assert!(is_valid_target("example.com"));
        assert!(is_valid_target("https://example.com"));
        assert!(is_valid_target("2001:db8::1"));
    }

    #[test]
    fn test_invalid_targets() {
        assert!(!is_valid_target("invalid..hostname"));
        assert!(!is_valid_target("256.256.256.256"));
        assert!(!is_valid_target(""));
        assert!(!is_valid_target("not a url"));
    }

    #[test]
    fn test_valid_port_ranges() {
        assert!(is_valid_port_range("80"));
        assert!(is_valid_port_range("80,443,8080"));
        assert!(is_valid_port_range("1-1000"));
        assert!(is_valid_port_range("80,443-445,8080-8090"));
    }

    #[test]
    fn test_invalid_port_ranges() {
        assert!(!is_valid_port_range("0"));
        assert!(!is_valid_port_range("65536"));
        assert!(!is_valid_port_range("80-443"));
        assert!(!is_valid_port_range("80,invalid,443"));
    }
}