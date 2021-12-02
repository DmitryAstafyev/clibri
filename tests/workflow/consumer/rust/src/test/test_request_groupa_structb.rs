use super::{controller, samples, stat, ClientError, Consumer, StatEvent};
use tokio::sync::mpsc::UnboundedSender;

pub async fn execute(
    consumer: &mut Consumer<ClientError>,
    tx_stat: &UnboundedSender<StatEvent>,
) -> Result<(), String> {
    let mut struct_a = false;
    let mut struct_ac = false;
    let mut struct_b = false;
    while !struct_a || !struct_b || !struct_ac {
        let response = consumer
            .groupa_structb(samples::group_a::struct_b::get())
            .await
            .map_err(|e| e.to_string())?;
        tx_stat.send(StatEvent::Inc(stat::Alias::TestRequestGroupAStructB));
        match response {
            controller::GroupAStructBResponse::GroupBStructA(res) => {
                if !samples::group_b::struct_a::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_a = true;
            }
            controller::GroupAStructBResponse::GroupBGroupCStructA(res) => {
                if !samples::group_b::group_c::struct_a::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_ac = true;
            }
            controller::GroupAStructBResponse::Err(res) => {
                if !samples::group_a::struct_b::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_b = true;
            }
        };
    }
    Ok(())
}
