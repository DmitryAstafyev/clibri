use super::{controller, samples, ClientError, Consumer};

pub async fn execute(consumer: &mut Consumer<ClientError>) -> Result<(), String> {
    let mut struct_a = false;
    let mut struct_b = false;
    while !struct_a || !struct_b {
        let response = consumer
            .structempty(samples::struct_empty::get())
            .await
            .map_err(|e| e.to_string())?;
        match response {
            controller::StructEmptyResponse::Response(res) => {
                if !samples::struct_empty_b::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_a = true;
            }
            controller::StructEmptyResponse::Err(res) => {
                if !samples::struct_empty_a::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_b = true;
            }
        };
    }
    Ok(())
}
