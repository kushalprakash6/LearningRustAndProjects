#[derive(Debug)]
enum DigitalConsent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalConsent> {
    fn consume_entertainment(&self) {
        println!("Wathcing the {:?}", self.content)
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()

    }
}
fn main() {

    let ex1 = ChatMessage {
        content: "String slice",
        time: String::from("12:34"),
    };
    println!("{}",ex1.retrieve_time());

    let ex2 = ChatMessage {
        content: String::from("String slice"),
        time: String::from("11:34"),
    };
    println!("{}",ex2.retrieve_time());

    let ex3 = ChatMessage {
        content: DigitalConsent::VideoFile,
        time: String::from("14:23"),
    };
    println!("{}",ex3.retrieve_time());
    ex3.consume_entertainment();

}
