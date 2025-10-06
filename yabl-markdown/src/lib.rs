use regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub content: String,
    pub inferred_locale: Option<String>,
    pub extraction_type: ExtractionType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtractionType {
    FencedCodeBlock,
    ContextualBlock,
}

lazy_static! {
    // Match fenced code blocks with yCard language
    static ref FENCED_YCARD: Regex = Regex::new(
        r"(?m)^```\s*(?i:ycard)\s*\n((?s:.*?))^```\s*$"
    ).unwrap();
    
    // Match contact headings in various languages
    static ref CONTACT_HEADINGS: Regex = Regex::new(
        r"(?mi)^#{1,6}\s+(Contact|Contacts|Kontakt|连络|連絡先|Contacto|Contactos)\s*$"
    ).unwrap();
    
    // Match YAML-like content after headings
    static ref YABL_BLOCK: Regex = Regex::new(
        r"(?m)^([a-zA-Z_\u4e00-\u9fff\u3040-\u309f\u30a0-\u30ff\u00c0-\u017f][^:\n]*:\s*[^\n]*(?:\n(?:  [^\n]+|\s*$))*)"
    ).unwrap();
}

/// Extract yCard blocks from fenced code blocks
pub fn extract_ycard_fenced(md: &str) -> Vec<Span> {
    let mut spans = Vec::new();
    
    for capture in FENCED_YCARD.captures_iter(md) {
        if let Some(content_match) = capture.get(1) {
            let full_match = capture.get(0).unwrap();
            spans.push(Span {
                start: full_match.start(),
                end: full_match.end(),
                content: content_match.as_str().trim().to_string(),
                inferred_locale: None, // Would infer from surrounding context
                extraction_type: ExtractionType::FencedCodeBlock,
            });
        }
    }
    
    spans
}

/// Extract yCard blocks from contextual headings
pub fn extract_ycard_context(md: &str, locale_hint: Option<&str>) -> Vec<Span> {
    let mut spans = Vec::new();
    let lines: Vec<&str> = md.lines().collect();
    
    for (i, line) in lines.iter().enumerate() {
        if CONTACT_HEADINGS.is_match(line) {
            // Look for YAML-like content after this heading
            let mut content_lines = Vec::new();
            let mut j = i + 1;
            
            while j < lines.len() {
                let current_line = lines[j];
                
                // Stop at next heading of same or higher level
                if is_heading(current_line) && get_heading_level(current_line) <= get_heading_level(line) {
                    break;
                }
                
                // Check if line looks like YAML key-value
                if is_yaml_like_line(current_line) {
                    content_lines.push(current_line);
                } else if current_line.trim().is_empty() {
                    // Empty line - continue
                } else if !content_lines.is_empty() {
                    // Non-YAML content after we found some - stop
                    break;
                }
                
                j += 1;
            }
            
            if !content_lines.is_empty() {
                let content = content_lines.join("\n");
                let start_byte = md.lines().take(i + 1).map(|l| l.len() + 1).sum::<usize>();
                let end_byte = start_byte + content.len();
                
                spans.push(Span {
                    start: start_byte,
                    end: end_byte,
                    content,
                    inferred_locale: locale_hint.map(|s| s.to_string()),
                    extraction_type: ExtractionType::ContextualBlock,
                });
            }
        }
    }
    
    spans
}

fn is_heading(line: &str) -> bool {
    line.trim_start().starts_with('#')
}

fn get_heading_level(line: &str) -> usize {
    line.trim_start().chars().take_while(|&c| c == '#').count()
}

fn is_yaml_like_line(line: &str) -> bool {
    let trimmed = line.trim();
    
    // Check for key: value pattern
    if let Some(colon_pos) = trimmed.find(':') {
        let key = trimmed[..colon_pos].trim();
        
        // Key should be valid identifier (simplified check)
        if !key.is_empty() && !key.contains(' ') {
            return true;
        }
    }
    
    // Check for list item
    if trimmed.starts_with("- ") {
        return true;
    }
    
    // Check for indented content (part of previous key)
    if line.starts_with("  ") && !trimmed.is_empty() {
        return true;
    }
    
    false
}

/// Extract all yCard content from markdown (both fenced and contextual)
pub fn extract_all_ycard(md: &str, locale_hint: Option<&str>) -> Vec<Span> {
    let mut spans = Vec::new();
    
    spans.extend(extract_ycard_fenced(md));
    spans.extend(extract_ycard_context(md, locale_hint));
    
    // Sort by position
    spans.sort_by_key(|span| span.start);
    
    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fenced_extraction() {
        let md = r#"
# Document Title

Here is a contact:

```ycard
name: Jane Doe
mobile: 555-123-4567
```

Some other content.
"#;

        let spans = extract_ycard_fenced(md);
        assert_eq!(spans.len(), 1);
        assert!(spans[0].content.contains("name: Jane Doe"));
        assert_eq!(spans[0].extraction_type, ExtractionType::FencedCodeBlock);
    }

    #[test]
    fn test_contextual_extraction() {
        let md = r#"
# About Me

## Contact
name: John Smith
home: +44 20 7946 0958
email: john@example.com

## Other Section
This is not contact info.
"#;

        let spans = extract_ycard_context(md, Some("en"));
        assert_eq!(spans.len(), 1);
        assert!(spans[0].content.contains("name: John Smith"));
        assert_eq!(spans[0].extraction_type, ExtractionType::ContextualBlock);
        assert_eq!(spans[0].inferred_locale, Some("en".to_string()));
    }

    #[test]
    fn test_localized_headings() {
        let md = r#"
## Kontakt
name: Hans Mueller
telefon: +49 30 12345678
"#;

        let spans = extract_ycard_context(md, Some("de"));
        assert_eq!(spans.len(), 1);
        assert!(spans[0].content.contains("Hans Mueller"));
    }

    #[test]
    fn test_combined_extraction() {
        let md = r#"
# Document

```ycard
name: Alice
```

## Contact  
name: Bob
mobile: 123456789
"#;

        let spans = extract_all_ycard(md, Some("en"));
        assert_eq!(spans.len(), 2);
        
        // Should be sorted by position
        assert!(spans[0].content.contains("Alice"));
        assert!(spans[1].content.contains("Bob"));
    }
}