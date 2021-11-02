use super::{controller, protocol, Consumer, Context};

pub async fn handler<E: std::error::Error + Clone>(
    context: &mut Context,
    mut consumer: Consumer<E>,
) {
    println!("Consumer is connected.");
    println!("Please type your login:");
    let username = match context.get_username().await {
        Ok(username) => username,
        Err(err) => {
            eprintln!("Fail to get user input: {}", err);
            return;
        }
    };
    println!("Sending login request...");
    match consumer
        .request_userlogin(protocol::UserLogin::Request { username })
        .await
    {
        Ok(response) => match response {
            controller::RequestUserLoginResponse::Accepted(_) => {
                println!("You are in!");
                //context.listen(username, consumer);
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
