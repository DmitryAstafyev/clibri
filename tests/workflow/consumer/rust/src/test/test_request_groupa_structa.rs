use super::{controller, samples, stat, ClientError, Consumer, StatEvent};
use tokio::sync::mpsc::UnboundedSender;

pub async fn execute(
    consumer: &mut Consumer<ClientError>,
    tx_stat: &UnboundedSender<StatEvent>,
) -> Result<(), String> {
    let mut struct_a = false;
    let mut struct_ba = false;
    let mut struct_b = false;
    while !struct_a || !struct_b || !struct_ba {
        let response = consumer
            .groupa_structa(samples::group_a::struct_a::get())
            .await
            .map_err(|e| e.to_string())?;
        tx_stat.send(StatEvent::Inc(stat::Alias::TestRequestGroupAStructA));
        match response {
            controller::GroupAStructAResponse::RootA(res) => {
                if !samples::struct_a::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_a = true;
            }
            controller::GroupAStructAResponse::RootB(res) => {
                if !samples::struct_b::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_b = true;
            }
            controller::GroupAStructAResponse::Err(res) => {
                if !samples::group_a::struct_b::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_ba = true;
            }
        };
    }
    Ok(())
}
