use std::io::stdin;

pub fn read_line() -> Option<String> {
    let mut buffer = String::new();
    if stdin().read_line(&mut buffer).ok()? == 0 {
        return None;
    }
    if buffer == "\n" {
        return None;
    }
    return Some(buffer);
}
