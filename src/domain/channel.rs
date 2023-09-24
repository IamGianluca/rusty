pub struct Channel {
    pub name: String,
    pub messages: Vec<Message>,
}

impl Channel {
    pub fn new(name: String) -> Channel {
        Channel {
            name,
            messages: Vec::new(),
        }
    }
    pub fn add_message(&mut self, sender: String, text: String) {
        let msg = Message::new(sender, text);
        self.messages.push(msg);
    }
}

pub struct Message {
    pub sender: String,
    pub text: String,
}

impl Message {
    pub fn new(sender: String, text: String) -> Message {
        Message { sender, text }
    }
}
