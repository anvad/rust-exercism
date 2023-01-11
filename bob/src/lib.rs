// chrismcg's solution is so much better!
// - changed from is_all_caps to is_yelling -
// - also thinking in terms of business language, rather than implementation detail
// - also avoiding creating a lambda for `.any()`
//    earlier: `.any(|ch| ch.is_alphabetic());`
//    now:     `.any(char::is_alphabetic);`
pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_question = message.ends_with("?");
    let has_alphabet = message.chars().any(char::is_alphabetic);
    let is_yelling = has_alphabet && message == message.to_uppercase();
    match message {
        _ if message.is_empty() => "Fine. Be that way!",
        _ if is_question && is_yelling => "Calm down, I know what I'm doing!",
        _ if is_question => "Sure.",
        _ if is_yelling => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
