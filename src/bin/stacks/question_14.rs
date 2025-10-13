// Valide se tags HTML estão corretamente aninhadas com uma pilha.

fn parse_tags(s: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < s.len() {
        if bytes[i] == b'<' {
            let mut j = i + 1;
            while j < s.len() && bytes[j] != b'>' {
                j += 1;
            }
            if j == s.len() {
                break;
            } // tag malformada
            let tag = &s[i + 1..j];
            tags.push(tag.trim().to_string());
            i = j + 1;
        } else {
            i += 1;
        }
    }
    tags
}

fn is_html_balanced(s: &str) -> bool {
    let mut stack: Vec<String> = Vec::new();
    let tags = parse_tags(s);
    for t in tags {
        if t.starts_with("!") || t.starts_with("?") {
            continue;
        } // ignorar comentários/declarações simples
        if t.ends_with('/') {
            // self-closing, ignora
            continue;
        }
        if t.starts_with('/') {
            let name = t[1..].split_whitespace().next().unwrap_or("");
            if let Some(top) = stack.pop() {
                if top != name {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            let name = t.split_whitespace().next().unwrap_or("");
            stack.push(name.to_string());
        }
    }
    stack.is_empty()
}

fn main() {
    let examples = [
        "<div><p>Olá</p></div>",
        "<div><span></div></span>",
        "<br/>",
        "<ul><li>1<li>2</ul>",
    ];
    for e in &examples {
        println!("'{}' -> {}", e, is_html_balanced(e));
    }
}
