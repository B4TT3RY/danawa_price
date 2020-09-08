pub struct Sender {
    api_link: String,
    chat_id: String,
}

impl Sender {
    pub fn new(bot_token: &str, chat_id: &str) -> Self {
        Self {
            api_link: format!("https://api.telegram.org/bot{}/", bot_token),
            chat_id: chat_id.to_string(),
        }
    }

    pub async fn send_message(&self, message: &str) {
        let url = format!(
            "{}sendMessage?chat_id={}&text={}&parse_mode=MarkdownV2&disable_web_page_preview=true",
            self.api_link, self.chat_id, message
        );
        let res = reqwest::get(&url).await.unwrap();
        println!("sendMessage: {}", res.status());
    }

    pub async fn set_chat_description(&self, description: &str) {
        let url = format!(
            "{}setChatDescription?chat_id={}&description={}",
            self.api_link, self.chat_id, description
        );
        let res = reqwest::get(&url).await.unwrap();
        println!("setChatDescription: {}", res.status());
    }
}

pub fn escape(message: &str) -> String {
    const ESCAPE: [char; 18] = [
        '_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!',
    ];
    let mut output = String::new();
    for c in message.chars() {
        if ESCAPE.contains(&c) {
            output.push(c);
        }
    }
    output
}
