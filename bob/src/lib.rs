pub fn reply(message: &str) -> &str {
    //    unimplemented!("have Bob reply to the incoming message: {}", message)

    let bob;
    let msg: String = message
        .trim()
        .chars()
        .filter(|x| x.is_alphabetic() || *x == '?')
        .collect();

    if message.trim() == "" {
        bob = "Fine. Be that way!";
    } else if msg.len() > 1 && msg.ends_with('?') && msg.to_uppercase() == msg {
        bob = "Calm down, I know what I'm doing!";
    } else if msg.ends_with('?') {
        bob = "Sure.";
    } else if !msg.is_empty() && msg.to_uppercase() == msg {
        bob = "Whoa, chill out!";
    } else {
        bob = "Whatever.";
    }

    bob
}
