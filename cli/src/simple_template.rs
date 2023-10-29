use std::collections::HashMap;

pub fn expand_template(template: &str, vars: &HashMap<String, String>) -> String {
    let mut expanded = String::new();
    let mut chars = template.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            if chars.peek() == Some(&'$') {
                // Handle $$ escape sequence
                expanded.push('$');
                chars.next();
            } else if chars.peek() == Some(&'{') {
                // Handle variable placeholder ${...}
                chars.next(); // Skip '{'
                let mut key = String::new();
                while let Some(ch) = chars.peek() {
                    if *ch == '}' {
                        break; // End of placeholder
                    }
                    key.push(*ch);
                    chars.next(); // Consume character
                }
                chars.next(); // Skip '}'
                if let Some(value) = vars.get(&key) {
                    // Substitute variable value
                    expanded.push_str(value);
                } else {
                    // Leave placeholder unchanged if key not found
                    expanded.push_str(&format!("${{{}}}", key));
                }
            } else {
                // Handle lone $
                expanded.push('$');
            }
        } else {
            // Copy other characters
            expanded.push(ch);
        }
    }
    expanded
}
