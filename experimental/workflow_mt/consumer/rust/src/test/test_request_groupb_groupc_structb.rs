use super::{controller, samples, stat, ClientError, Consumer, StatEvent};
use tokio::sync::mpsc::UnboundedSender;

pub async fn execute(
    consumer: &mut Consumer<ClientError>,
    tx_stat: &UnboundedSender<StatEvent>,
) -> Result<(), String> {
    let mut case_b = false;
    let mut case_d = false;
    let mut case_c = false;
    let mut struct_a = false;
    while !struct_a || !case_b || !case_d || !case_c {
        let response = consumer
            .groupb_groupc_structb(samples::group_b::group_c::struct_b::get())
            .await
            .map_err(|e| e.to_string())?;
        tx_stat
            .send(StatEvent::Inc(stat::Alias::TestRequestGroupBGroupCStructB))
            .map_err(|e| e.to_string())?;
        match response {
            controller::GroupBGroupCStructBResponse::CaseB(res) => {
                if !samples::struct_b::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                case_b = true;
            }
            controller::GroupBGroupCStructBResponse::CaseC(res) => {
                if !samples::struct_c::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                case_c = true;
            }
            controller::GroupBGroupCStructBResponse::CaseD(res) => {
                if !samples::struct_d::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                case_d = true;
            }
            controller::GroupBGroupCStructBResponse::Err(res) => {
                if !samples::group_b::group_c::struct_a::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                struct_a = true;
            }
        };
    }
    Ok(())
}
