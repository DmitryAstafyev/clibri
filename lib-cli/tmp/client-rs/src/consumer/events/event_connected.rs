use super::{controller, protocol, Consumer, Context};
use fiber::client;

pub async fn handler<E: client::Error>(context: &mut Context, mut consumer: Consumer<E>) {
    context.reinit();
    println!("Consumer is connected.");
    println!("Please type your login:");
    let username = match context.get_username().await {
        Ok(username) => username.trim().to_owned(),
        Err(err) => {
            eprintln!("Fail to get user input: {}", err);
            return;
        }
    };
    println!("Sending login request...");
    match consumer
        .request_userlogin(protocol::UserLogin::Request {
            username: username.clone(),
        })
        .await
    {
        Ok(response) => match response {
            controller::RequestUserLoginResponse::Accepted(_) => {
                println!("You are in!");
                match consumer
                    .request_messages(protocol::Messages::Request {})
                    .await
                {
                    Ok(response) => match response {
                        controller::RequestMessagesResponse::Response(response) => {
                            for message in response.messages {
                                println!(
                                    "[{}] {}: {}",
                                    context.get_localtime(message.timestamp),
                                    message.user,
                                    message.message.trim()
                                );
                            }
                        }
                        controller::RequestMessagesResponse::Err(msg) => {
                            println!("Fail get messages because: {}", msg.error);
                        }
                    },
                    Err(err) => {
                        eprintln!("Fail to send Messages request: {}", err);
                    }
                };
                context.listen::<E>(username, consumer).await;
            }
            controller::RequestUserLoginResponse::Denied(_) => {
                println!("Access is denied!");
            }
            controller::RequestUserLoginResponse::Err(msg) => {
                println!("Login is fail because: {}", msg.error);
            }
        },
        Err(err) => {
            eprintln!("Fail to send UserLogin request: {}", err);
        }
    };
}
