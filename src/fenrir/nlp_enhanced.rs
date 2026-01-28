// ============================================================================
// FENRIR NLP v3.0 - ENHANCED SEARCH INDEX MATCHING
// ============================================================================
// Integrates dark_index with improved fuzzy matching algorithms
// Provides bidirectional keyword<->tool matching with typo tolerance

use crate::dark_index;
use std::collections::HashMap;
use regex::Regex;

// ============================================================================
// ENHANCED MATCHING STRUCTURES
// ============================================================================

#[derive(Debug, Clone)]
pub struct ToolMatch {
    pub tool: String,
    pub score: f32,
    pub matched_keywords: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct KeywordMatch {
    pub keyword: String,
    pub score: f32,
    pub tools: Vec<String>,
    pub confidence: f32,
}

// ============================================================================
// LEVENSHTEIN DISTANCE FOR TYPO TOLERANCE
// ============================================================================

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let len1 = chars1.len();
    let len2 = chars2.len();

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if chars1[i - 1] == chars2[j - 1] { 0 } else { 1 };
            matrix[i][j] = *[
                matrix[i - 1][j] + 1,      // deletion
                matrix[i][j - 1] + 1,      // insertion
                matrix[i - 1][j - 1] + cost, // substitution
            ].iter().min().unwrap();
        }
    }

    matrix[len1][len2]
}

fn normalized_levenshtein_similarity(s1: &str, s2: &str) -> f32 {
    let max_len = s1.len().max(s2.len());
    if max_len == 0 {
        return 1.0;
    }
    let distance = levenshtein_distance(s1, s2);
    1.0 - (distance as f32 / max_len as f32)
}

// ============================================================================
// JARO-WINKLER SIMILARITY (better for short strings)
// ============================================================================

fn jaro_similarity(s1: &str, s2: &str) -> f32 {
    if s1 == s2 {
        return 1.0;
    }

    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 || len2 == 0 {
        return 0.0;
    }

    let match_distance = len1.max(len2) / 2 - 1;
    if match_distance < 0 {
        return 0.0;
    }

    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    let mut s1_matches = vec![false; len1];
    let mut s2_matches = vec![false; len2];

    let mut matches = 0;
    for i in 0..len1 {
        let start = if i.saturating_sub(match_distance) > 0 { i - match_distance } else { 0 };
        let end = (i + match_distance + 1).min(len2);

        for j in start..end {
            if !s2_matches[j] && chars1[i] == chars2[j] {
                s1_matches[i] = true;
                s2_matches[j] = true;
                matches += 1;
                break;
            }
        }
    }

    if matches == 0 {
        return 0.0;
    }

    let mut transpositions = 0;
    let mut k = 0;
    for i in 0..len1 {
        if s1_matches[i] {
            while !s2_matches[k] {
                k += 1;
            }
            if chars1[i] != chars2[k] {
                transpositions += 1;
            }
            k += 1;
        }
    }

    let m = matches as f32;
    (m / len1 as f32 + m / len2 as f32 + (m - (transpositions / 2) as f32) / m) / 3.0
}

fn jaro_winkler_similarity(s1: &str, s2: &str) -> f32 {
    let jaro = jaro_similarity(s1, s2);

    let mut prefix = 0;
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    for i in 0..4.min(s1.len()).min(s2.len()) {
        if chars1[i] == chars2[i] {
            prefix += 1;
        } else {
            break;
        }
    }

    let jaro_winkler = jaro + (0.1 * prefix as f32 * (1.0 - jaro));
    jaro_winkler.min(1.0)
}

// ============================================================================
// TOKEN-BASED MATCHING WITH NORMALIZATION
// ============================================================================

fn normalize_token(token: &str) -> String {
    token.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ')
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn tokenize(input: &str) -> Vec<String> {
    // Split by whitespace and common delimiters
    let re = Regex::new(r"[,\s\-_\.]+").unwrap();
    re.split(input)
        .map(|s| normalize_token(s))
        .filter(|s| !s.is_empty() && s.len() > 2) // Skip short tokens
        .collect()
}

// ============================================================================
// ENHANCED FUZZY MATCHER
// ============================================================================

pub fn enhanced_fuzzy_match(input: &str, pattern: &str) -> f32 {
    let input_norm = normalize_token(input);
    let pattern_norm = normalize_token(pattern);

    // Exact match
    if input_norm == pattern_norm {
        return 1.0;
    }

    // Contains match
    if input_norm.contains(&pattern_norm) || pattern_norm.contains(&input_norm) {
        return 0.95;
    }

    // Jaro-Winkler for short strings (better for typos)
    if input_norm.len() < 10 || pattern_norm.len() < 10 {
        let jw_score = jaro_winkler_similarity(&input_norm, &pattern_norm);
        if jw_score > 0.8 {
            return jw_score * 0.9;
        }
    }

    // Levenshtein for longer strings
    let lev_score = normalized_levenshtein_similarity(&input_norm, &pattern_norm);
    if lev_score > 0.7 {
        return lev_score * 0.85;
    }

    // Token overlap for phrases
    let input_tokens: std::collections::HashSet<&str> =
        input_norm.split_whitespace().collect();
    let pattern_tokens: std::collections::HashSet<&str> =
        pattern_norm.split_whitespace().collect();

    let intersection = input_tokens.intersection(&pattern_tokens).count();
    let union = input_tokens.union(&pattern_tokens).count();

    if union > 0 {
        let jaccard = intersection as f32 / union as f32;
        if jaccard > 0.5 {
            return jaccard * 0.7;
        }
    }

    0.0
}

// ============================================================================
// BIDIRECTIONAL INDEX MATCHING
// ============================================================================

/// Match user input to keywords using dark_index + nlp definitions
pub fn match_keywords_from_input(input: &str) -> Vec<KeywordMatch> {
    let tokens = tokenize(input);
    let dark_index = dark_index::create_dark_index();
    let mut matches: HashMap<String, KeywordMatch> = HashMap::new();

    // 1. Direct tool name matching from dark_index
    for (tool, keywords) in &dark_index {
        let tool_lower = tool.to_lowercase();
        let input_lower = input.to_lowercase();

        // Check if tool name is mentioned
        if input_lower.contains(&tool_lower) {
            // Extract the most relevant keywords for this tool
            for keyword in keywords.iter().take(5) {
                let entry = matches.entry(keyword.clone()).or_insert_with(|| KeywordMatch {
                    keyword: keyword.clone(),
                    score: 0.0,
                    tools: Vec::new(),
                    confidence: 0.0,
                });
                entry.score = entry.score.max(0.95);
                entry.tools.push(tool.clone());
                entry.confidence = entry.confidence.max(0.9);
            }
        }
    }

    // 2. Reverse lookup: match input against all keywords in dark_index
    for (tool, keywords) in &dark_index {
        for keyword in keywords {
            let score = enhanced_fuzzy_match(input, keyword);
            if score >= 0.6 {
                // Find which high-level category this keyword belongs to
                let category = categorize_keyword(keyword);
                let entry = matches.entry(category.clone()).or_insert_with(|| KeywordMatch {
                    keyword: category.clone(),
                    score: 0.0,
                    tools: Vec::new(),
                    confidence: 0.0,
                });

                if entry.score < score {
                    entry.score = score;
                }
                if !entry.tools.contains(tool) {
                    entry.tools.push(tool.clone());
                }
                entry.confidence = entry.confidence.max(score * 0.9);
            }
        }
    }

    // 3. Token-based matching
    for token in &tokens {
        for (tool, keywords) in &dark_index {
            for keyword in keywords {
                let keyword_tokens = tokenize(keyword);
                for keyword_token in keyword_tokens {
                    let score = enhanced_fuzzy_match(token, &keyword_token);
                    if score >= 0.7 {
                        let category = categorize_keyword(keyword);
                        let entry = matches.entry(category.clone()).or_insert_with(|| KeywordMatch {
                            keyword: category.clone(),
                            score: 0.0,
                            tools: Vec::new(),
                            confidence: 0.0,
                        });

                        if entry.score < score * 0.8 {
                            entry.score = score * 0.8;
                        }
                        if !entry.tools.contains(tool) {
                            entry.tools.push(tool.clone());
                        }
                    }
                }
            }
        }
    }

    // Convert to sorted vector
    let mut result: Vec<KeywordMatch> = matches.into_values().collect();
    result.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Return top matches
    result.truncate(10);
    result
}

/// Match user input to specific tools (reverse direction)
pub fn match_tools_from_input(input: &str) -> Vec<ToolMatch> {
    let tokens = tokenize(input);
    let dark_index = dark_index::create_dark_index();
    let mut tool_scores: HashMap<String, (f32, Vec<String>)> = HashMap::new();

    for (tool, keywords) in &dark_index {
        let mut max_score: f32 = 0.0;
        let mut matched_keywords = Vec::new();

        // Check if tool name is directly mentioned
        let tool_lower = tool.to_lowercase();
        if input.to_lowercase().contains(&tool_lower) {
            max_score = max_score.max(1.0);
            matched_keywords.push(tool.clone());
        }

        // Match against keywords
        for keyword in keywords {
            let score = enhanced_fuzzy_match(input, keyword);
            if score >= 0.6 {
                if max_score < score {
                    max_score = score;
                }
                matched_keywords.push(keyword.clone());
            }

            // Also match tokens
            for token in &tokens {
                let token_score = enhanced_fuzzy_match(token, keyword);
                if token_score >= 0.7 && token_score > max_score {
                    max_score = token_score * 0.9;
                    matched_keywords.push(keyword.clone());
                }
            }
        }

        if max_score > 0.5 {
            tool_scores.insert(tool.clone(), (max_score, matched_keywords));
        }
    }

    // Convert to ToolMatch structs
    let mut result: Vec<ToolMatch> = tool_scores
        .into_iter()
        .map(|(tool, (score, keywords))| {
            let reason = if score >= 0.95 {
                format!("Direct tool name match or exact keyword match")
            } else if score >= 0.8 {
                format!("Strong keyword similarity: {}", keywords[0])
            } else {
                format!("Partial keyword match: {}", keywords.join(", "))
            };

            ToolMatch {
                tool,
                score,
                matched_keywords: keywords,
                reason,
            }
        })
        .collect();

    result.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    result.truncate(15);
    result
}

// ============================================================================
// KEYWORD CATEGORIZATION
// ============================================================================

fn categorize_keyword(keyword: &str) -> String {
    let keyword_lower = keyword.to_lowercase();

    // Password related
    if keyword_lower.contains("password") || keyword_lower.contains("crack") ||
       keyword_lower.contains("brute") || keyword_lower.contains("hash") ||
       keyword_lower.contains("credential") || keyword_lower.contains("auth") {
        return "password".to_string();
    }

    // Scanning related
    if keyword_lower.contains("scan") || keyword_lower.contains("port") ||
       keyword_lower.contains("recon") || keyword_lower.contains("enum") ||
       keyword_lower.contains("discover") || keyword_lower.contains("fingerprint") {
        return "scan".to_string();
    }

    // Web related
    if keyword_lower.contains("web") || keyword_lower.contains("http") ||
       keyword_lower.contains("sql") || keyword_lower.contains("xss") ||
       keyword_lower.contains("injection") || keyword_lower.contains("directory") {
        return "web".to_string();
    }

    // Exploitation
    if keyword_lower.contains("exploit") || keyword_lower.contains("payload") ||
       keyword_lower.contains("shellcode") || keyword_lower.contains("rce") ||
       keyword_lower.contains("vulnerability") {
        return "exploit".to_string();
    }

    // Network/Packet
    if keyword_lower.contains("packet") || keyword_lower.contains("sniff") ||
       keyword_lower.contains("mitm") || keyword_lower.contains("arp") ||
       keyword_lower.contains("capture") {
        return "sniff".to_string();
    }

    // Wireless
    if keyword_lower.contains("wireless") || keyword_lower.contains("wifi") ||
       keyword_lower.contains("wpa") || keyword_lower.contains("wep") {
        return "wireless".to_string();
    }

    // Forensics
    if keyword_lower.contains("forensic") || keyword_lower.contains("memory") ||
       keyword_lower.contains("image") || keyword_lower.contains("carve") {
        return "forensic".to_string();
    }

    // Default: return original keyword
    keyword.to_string()
}

// ============================================================================
// SMART SUGGESTIONS
// ============================================================================

pub fn generate_smart_suggestions(input: &str, keyword_matches: &[KeywordMatch], tool_matches: &[ToolMatch]) -> Vec<String> {
    let mut suggestions = Vec::new();

    // If no matches at all
    if keyword_matches.is_empty() && tool_matches.is_empty() {
        suggestions.push("💡 Try being more specific about what you want to do".to_string());
        suggestions.push("💡 Examples: \"scan 192.168.1.1\", \"crack password user@target.com\"".to_string());
        return suggestions;
    }

    // If we have keyword matches but no tools
    if !keyword_matches.is_empty() {
        let top_keyword = &keyword_matches[0];
        suggestions.push(format!(
            "🎯 Detected: {} (confidence: {:.0}%)",
            top_keyword.keyword,
            top_keyword.confidence * 100.0
        ));

        if top_keyword.tools.len() <= 5 {
            suggestions.push(format!(
                "🔧 Recommended tools: {}",
                top_keyword.tools.join(", ")
            ));
        } else {
            suggestions.push(format!(
                "🔧 Top tools: {} (and {} more)",
                top_keyword.tools[..5].join(", "),
                top_keyword.tools.len() - 5
            ));
        }

        // Suggest related keywords
        if keyword_matches.len() > 1 {
            let related: Vec<String> = keyword_matches[1..3.min(keyword_matches.len())]
                .iter()
                .map(|k| k.keyword.clone())
                .collect();
            if !related.is_empty() {
                suggestions.push(format!("🔗 Also consider: {}", related.join(", ")));
            }
        }
    }

    // If we have specific tool matches
    if !tool_matches.is_empty() {
        let top_tools: Vec<String> = tool_matches[..3.min(tool_matches.len())]
            .iter()
            .map(|t| t.tool.clone())
            .collect();

        if !top_tools.is_empty() {
            suggestions.push(format!(
                "🎯 Best matching tools: {}",
                top_tools.join(", ")
            ));
        }

        // Explain why tools matched
        if let Some(top_match) = tool_matches.first() {
            if !top_match.matched_keywords.is_empty() {
                suggestions.push(format!(
                    "ℹ️  Matched because: {}",
                    top_match.matched_keywords[0]
                ));
            }
        }
    }

    // Check for missing target
    let ip_regex = Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b").unwrap();
    let domain_regex = Regex::new(r"\b[a-zA-Z0-9][-a-zA-Z0-9]*\.[a-zA-Z]{2,}\b").unwrap();
    let email_regex = Regex::new(r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b").unwrap();

    let has_ip = ip_regex.is_match(input);
    let has_domain = domain_regex.is_match(input);
    let has_email = email_regex.is_match(input);

    if !has_ip && !has_domain && !has_email {
        suggestions.push("⚠️  No target detected. Add an IP, domain, or email for better results".to_string());
    }

    suggestions
}

// ============================================================================
// UNIFIED SEARCH FUNCTION
// ============================================================================

pub struct SearchResults {
    pub keywords: Vec<KeywordMatch>,
    pub tools: Vec<ToolMatch>,
    pub suggestions: Vec<String>,
    pub confidence: f32,
}

pub fn search_user_input(input: &str) -> SearchResults {
    let keywords = match_keywords_from_input(input);
    let tools = match_tools_from_input(input);
    let suggestions = generate_smart_suggestions(input, &keywords, &tools);

    // Calculate overall confidence
    let keyword_confidence = keywords.first().map(|k| k.confidence).unwrap_or(0.0);
    let tool_confidence = tools.first().map(|t| t.score).unwrap_or(0.0);
    let confidence = keyword_confidence.max(tool_confidence);

    SearchResults {
        keywords,
        tools,
        suggestions,
        confidence,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("password", "pasword"), 1);
        assert_eq!(levenshtein_distance("scan", "scn"), 1);
    }

    #[test]
    fn test_jaro_winkler() {
        let score = jaro_winkler_similarity("password", "pasword");
        assert!(score > 0.9);

        let score2 = jaro_winkler_similarity("nmap", "nmp");
        assert!(score2 > 0.8);
    }

    #[test]
    fn test_enhanced_fuzzy_match() {
        assert!(enhanced_fuzzy_match("password", "password") == 1.0);
        assert!(enhanced_fuzzy_match("password", "pasword") > 0.9);
        assert!(enhanced_fuzzy_match("scan", "scanning") > 0.8);
        assert!(enhanced_fuzzy_match("xyz", "password") < 0.5);
    }

    #[test]
    fn test_match_keywords() {
        let results = match_keywords_from_input("scan for vulnerabilities");
        assert!(!results.is_empty());
        assert!(results.iter().any(|k| k.keyword == "scan"));
    }

    #[test]
    fn test_match_tools() {
        let results = match_tools_from_input("password cracking with john");
        assert!(!results.is_empty());
        assert!(results.iter().any(|t| t.tool.to_lowercase().contains("john")));
    }

    #[test]
    fn test_unified_search() {
        let results = search_user_input("scan 192.168.1.1 with nmap");
        assert!(!results.keywords.is_empty() || !results.tools.is_empty());
        assert!(!results.suggestions.is_empty());
        assert!(results.confidence > 0.5);
    }

    #[test]
    fn test_typo_tolerance() {
        let results = match_keywords_from_input("crak pasword for target");
        assert!(!results.is_empty());
        assert!(results.iter().any(|k| k.keyword == "password" || k.keyword == "crack"));
    }
}
