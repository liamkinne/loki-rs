mod push;

/// The Loki client
pub struct Loki {
    /// The URI at which the Loki server resides
    address: String
}

impl Loki {

    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn push_log(&self, stream_name: String, stream_value: String, log_messages: Vec<[String; 2]>) {
        push::push_log(self.address.clone(), stream_name, stream_value, log_messages);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn simple_push() {
        let client = Loki::new("http://localhost:3100".to_string());

        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        client.push_log("foo".to_string(), "bar".to_string(),  vec![[timestamp.to_string(), "Test log!".to_string()]]);
        assert!(true);
    }
}
