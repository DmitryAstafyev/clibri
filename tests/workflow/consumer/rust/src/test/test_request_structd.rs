use super::{controller, samples, stat, ClientError, Consumer, StatEvent};
use tokio::sync::mpsc::UnboundedSender;

pub async fn execute(
    consumer: &mut Consumer<ClientError>,
    tx_stat: &UnboundedSender<StatEvent>,
) -> Result<(), String> {
    let mut struct_a = false;
    let mut struct_c = false;
    while !struct_a || !struct_c {
        let response = consumer
            .structd(samples::struct_d::get())
            .await
            .map_err(|e| e.to_string())?;
        tx_stat.send(StatEvent::Inc(stat::Alias::TestRequestStructD));
        match response {
            controller::StructDResponse::Response(res) => {
                if !samples::struct_a::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_a = true;
            }
            controller::StructDResponse::Err(res) => {
                if !samples::struct_c::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_c = true;
            }
        };
    }
    Ok(())
}
