use chatgpt::prelude::*;

#[tokio::main]
async fn main(){
    let key = std::env::args().nth(1).unwrap();
    let client = ChatGPT::new(key).unwrap();

    loop{
        println!("Enter your prompt:");
        let mut input = String::new(); 
        std::io::stdin().read_line(&mut input).unwrap();
        let result = client.send_message(input).await;
        println!("-----------------------");
        match result{
            Ok(response) => {
                println!("Response:");
                println!("{}", response.message().content);
            },
            Err(_) => {
                println!("Damn! The suckers at openai are misbheving again... *sigh*");
                println!("Sorry mate! We're ending the party early...");
                break;
            }
        }
    }
}
