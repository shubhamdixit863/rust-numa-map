use numaflow::map;
use std::error::Error;
use numaflow::map::{Mapper, MapRequest, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    map::Server::new(Cat).start().await
}


struct Cat;

#[tonic::async_trait]
impl Mapper for Cat{
    async fn map(&self, _input: MapRequest) -> Vec<Message> {
        let my_string = String::from("Hello, world!");
        let byte_vector = my_string.as_bytes().to_vec();
        let message=Message::new_message(byte_vector);
        let message_to_drop=Message::message_to_drop();
       let modified_message= message_to_drop.with_keys(vec!["shouldn't be there".to_string()]);

        vec![message,modified_message]
    }
}