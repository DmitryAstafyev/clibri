use super::{controller, samples, stat, ClientError, Consumer, StatEvent};
use tokio::sync::mpsc::UnboundedSender;

pub async fn execute(
    consumer: &mut Consumer<ClientError>,
    tx_stat: &UnboundedSender<StatEvent>,
) -> Result<(), String> {
    let mut struct_a = false;
    let mut struct_b = false;
    while !struct_a || !struct_b {
        let response = consumer
            .structempty(samples::struct_empty::get())
            .await
            .map_err(|e| e.to_string())?;
        tx_stat.send(StatEvent::Inc(stat::Alias::TestRequestStructRmpty));
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
