use super::{controller, samples, stat, ClientError, Consumer, StatEvent};
use tokio::sync::mpsc::UnboundedSender;

pub async fn execute(
    consumer: &mut Consumer<ClientError>,
    tx_stat: &UnboundedSender<StatEvent>,
) -> Result<(), String> {
    let mut struct_f = false;
    let mut struct_e = false;
    while !struct_f || !struct_e {
        let response = consumer
            .structf(samples::struct_f::get())
            .await
            .map_err(|e| e.to_string())?;
        tx_stat.send(StatEvent::Inc(stat::Alias::TestRequestStructF));
        match response {
            controller::StructFResponse::Response(res) => {
                if !samples::struct_f::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_f = true;
            }
            controller::StructFResponse::Err(res) => {
                if !samples::struct_e::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_e = true;
            }
        };
    }
    Ok(())
}
