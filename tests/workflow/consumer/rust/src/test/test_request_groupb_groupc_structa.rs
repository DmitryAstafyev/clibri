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
            .groupb_groupc_structa(samples::group_b::group_c::struct_a::get())
            .await
            .map_err(|e| e.to_string())?;
        tx_stat
            .send(StatEvent::Inc(stat::Alias::TestRequestGroupBGroupCStructA))
            .map_err(|e| e.to_string())?;
        match response {
            controller::GroupBGroupCStructAResponse::Response(res) => {
                if !samples::group_b::group_c::struct_b::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_a = true;
            }
            controller::GroupBGroupCStructAResponse::Err(res) => {
                if !samples::group_a::struct_b::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_b = true;
            }
        };
    }
    Ok(())
}
