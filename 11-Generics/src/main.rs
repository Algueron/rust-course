/*
    Let's model a real-time chat system where users can
    share audio and video files.
*/

/*
    Define a DigitalContent enum with two variants:
    AudioFile and VideoFile. Derive a Debug implementation.
*/
#[derive(Debug)]
enum DigitalContent {
    #[allow(dead_code)]
    AudioFile,
    VideoFile
}

/*
    Define a ChatMessage struct with two fields: `content`
    and `time`. The struct should define one generic type, T,
    which will be the type of the `content` field.
    The `time` field should always be a String.
    Derive a Debug implementation.
*/
#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String
}

/*
    Add an impl block for ChatMessage structs whose T type
    is a DigitalContent enum. Define a `consume_entertainment`
    method that prints out the value of the `content` field in
    Debug format. For example, "Watching the AudioFile".
*/
impl ChatMessage<DigitalContent> {

    fn consume_entertainment(&self) {
        println!("Watching the {:?}", self.content);
    }
}

/*
    Add an impl block for ChatMessage structs with any type T.
    Define a `retrieve_time` method that returns a String.
    It should return a clone of the `time` field from
    the struct.
*/
impl<T> ChatMessage<T> {

    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    /*
        In `main`, create a ChatMessage with `content` set to a
        string slice.
    */
    let str_chat_message = ChatMessage{
        content: "str", 
        time: String::from("2008")
    };

    /*
        Create another ChatMessage with `content` set to a String.
    */
    let string_chat_message = ChatMessage {
        content: String::from("String"),
        time: String::from("2012")
    };

    /*
        Create another ChatMessage with `content' set to a
        DigitalContent variant.
    */
    let digital_content_chat_message = ChatMessage {
        content: DigitalContent::VideoFile,
        time: String::from("2019")
    };

    /*
        Invoke the `consume_entertainment` method on the
        ChatMessage storing a DigitalContent enum.
    */
    digital_content_chat_message.consume_entertainment();

    /*
        Invoke the `retrieve_time` method on all 3 ChatMessage
        instances and print out each String's content.
    */
    println!("{}", str_chat_message.retrieve_time());
    println!("{}", string_chat_message.retrieve_time());
    println!("{}", digital_content_chat_message.retrieve_time());
}
