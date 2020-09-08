pub async fn send_message(bot_token: &str, chat_id: &str, message: &str) {
    let url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}&parse_mode=MarkdownV2&disable_web_page_preview=true", bot_token, chat_id, message);
    let res = reqwest::get(&url).await.unwrap();
    println!("sendMessage: {}", res.status());
}

pub async fn set_chat_description(bot_token: &str, chat_id: &str, description: &str) {
    let url = format!("https://api.telegram.org/bot{}/setChatDescription?chat_id={}&description={}", bot_token, chat_id, description);
    let res = reqwest::get(&url).await.unwrap();
    println!("setChatDescription: {}", res.status());
}

pub fn syntax(message: &str) -> String {
    message.replace("_", "\\_")
        .replace("*", "\\*")
        .replace("[", "\\[")
        .replace("]", "\\]")
        .replace("(", "\\(")
        .replace(")", "\\)")
        .replace("~", "\\~")
        .replace("`", "\\`")
        .replace(">", "\\>")
        .replace("#", "\\#")
        .replace("+", "\\+")
        .replace("-", "\\-")
        .replace("=", "\\=")
        .replace("|", "\\|")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace(".", "\\.")
        .replace("!", "\\!")
}
