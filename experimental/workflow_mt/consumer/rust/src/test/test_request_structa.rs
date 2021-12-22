use super::{controller, samples, stat, ClientError, Consumer, StatEvent};
use tokio::sync::mpsc::UnboundedSender;

pub async fn execute(
    consumer: &mut Consumer<ClientError>,
    tx_stat: &UnboundedSender<StatEvent>,
) -> Result<(), String> {
    let mut case_b = false;
    let mut case_c = false;
    let mut case_d = false;
    let mut case_err = false;
    while !case_b || !case_c || !case_d || !case_err {
        let response = consumer
            .structa(samples::struct_a::get())
            .await
            .map_err(|e| e.to_string())?;
        tx_stat
            .send(StatEvent::Inc(stat::Alias::TestRequestStructA))
            .map_err(|e| e.to_string())?;
        match response {
            controller::StructAResponse::CaseB(res) => {
                if !samples::struct_b::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                case_b = true;
            }
            controller::StructAResponse::CaseD(res) => {
                if !samples::struct_d::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                case_d = true;
            }
            controller::StructAResponse::CaseC(res) => {
                if !samples::struct_c::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                case_c = true;
            }
            controller::StructAResponse::Err(res) => {
                if !samples::struct_e::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                case_err = true;
            }
        };
    }
    Ok(())
}
