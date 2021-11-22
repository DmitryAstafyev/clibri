use super::{controller, samples, ClientError, Consumer};

pub async fn execute(consumer: &mut Consumer<ClientError>) -> Result<(), String> {
    let mut case_b = false;
    let mut case_f = false;
    let mut case_d = false;
    let mut case_err = false;
    while !case_b || !case_f || !case_d || !case_err {
        let response = consumer
            .structc(samples::struct_c::get())
            .await
            .map_err(|e| e.to_string())?;
        match response {
            controller::StructCResponse::CaseB(res) => {
                if !samples::struct_b::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                case_b = true;
            }
            controller::StructCResponse::CaseD(res) => {
                if !samples::struct_d::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                case_d = true;
            }
            controller::StructCResponse::CaseF(res) => {
                if !samples::struct_f::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                case_f = true;
            }
            controller::StructCResponse::Err(res) => {
                if !samples::struct_e::equal(res.clone()) {
                    return Err(format!("Invalid data received: {:?}", res));
                }
                case_err = true;
            }
        };
    }
    Ok(())
}
