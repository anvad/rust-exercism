// ends in [?, ? && CAPS] ["Sure", "Calm down, I know what I'm doing!"]
//  CAPS ['Whoa, chill out!']
//  nothing ["Fine. Be that way!"]
//  _ ["Whatever."]

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if m.ends_with("?") && is_all_caps(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        m if is_all_caps(m) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

fn is_all_caps(message: &str) -> bool {
    message
        .chars()
        .filter(|&ch| ch.is_ascii_alphabetic())
        .count()
        > 0
        && message
            .chars()
            .filter(|&ch| ch.is_ascii_alphabetic())
            .all(|ch| ch.is_ascii_uppercase())
}
