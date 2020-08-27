// types of socket messages
enum SocketMessage {
  CLIENT_CONNECT,
  CLIENT_DISCONNECT,
  HEARTBEAT,
  CHAT(String), // message
}

// can add methods to enums!
impl SocketMessage {
  // acknowledge a socket message and print
  // it to the output!!!!
  fn acknowledge(&self) {
    match self {
      SocketMessage::CLIENT_CONNECT => println!("Client is connecting..."),
      SocketMessage::CLIENT_DISCONNECT => println!("Client is disconnecting..."),
      SocketMessage::HEARTBEAT => println!("Received heartbeat from client. Keeping them online"),
      SocketMessage::CHAT(message) => println!("Received message: {}", message),
    }
  }
}

fn double(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(x) => Some(x * 2),
  }
}

fn main() {
  // fake connect event
  SocketMessage::CLIENT_CONNECT.acknowledge();
  // fake chat event
  let chat = SocketMessage::CHAT(String::from("hello world!"));
  chat.acknowledge();
  // finally a fake disconnect event
  SocketMessage::CLIENT_DISCONNECT.acknowledge();

  // options!
  // aka, possibly null values!

  let some_definite_val: Option<i32> = Some(11);

  let some_number: Option<i32> = Some(5);
  let null_number: Option<i32> = None;

  if let Some(45) = some_definite_val {
    println!("11!!!!");
  } else {
    println!("not 11 :(");
  }

  // Option<i32> doubled!
  let some_doubled = double(some_number);

  // still None
  let null_doubled = double(null_number);
}
