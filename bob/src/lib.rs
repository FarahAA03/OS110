pub fn reply(message: &str) -> String {
    if is_empty(message) {
        return "Fine. Be that way!".to_string();
    }

    if is_yelling(message) {
        return "Whoa, chill out!".to_string();
    }

    if is_question(message.trim()) {
        return "Sure.".to_string();
    }

    return "Whatever.".to_string();
}
fn is_yelling(message: &str) -> bool {
    message == message.to_uppercase() && message != message.to_lowercase()
}

fn is_empty(message: &str) -> bool {
   message.trim().is_empty()
}

fn is_question(message: &str) -> bool {
    message.ends_with("?")
}


    

