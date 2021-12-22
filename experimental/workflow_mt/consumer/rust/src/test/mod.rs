use super::{
    consumer::{implementation::controller, protocol, Consumer},
    stat,
    stat::StatEvent,
};
use clibri_transport_client::errors::Error as ClientError;
use std::future::Future;
use tokio::{
    select,
    time::{sleep, Duration},
};
pub mod samples;
pub mod test_request_groupa_structa;
pub mod test_request_groupa_structb;
pub mod test_request_groupb_groupc_structa;
pub mod test_request_groupb_groupc_structb;
pub mod test_request_groupb_structa;
pub mod test_request_structa;
pub mod test_request_structc;
pub mod test_request_structd;
pub mod test_request_structempty;
pub mod test_request_structf;

pub async fn executor(
    name: &str,
    timeout: u64,
    task: impl Future<Output = Result<(), String>>,
) -> Result<(), String> {
    select! {
        res = task => {
            if let Err(err) = res.as_ref() {
                println!("Task \"{}\" error: {}", name, err);
            }
            res
        },
        _ = async {
            sleep(Duration::from_millis(timeout)).await;
        } => {
            let err_msg = format!("Timeout ({}) error; task \"{}\"", timeout, name);
            println!("{}", err_msg);
            Err(err_msg)
        }
    }
}

pub async fn executor_no_res(
    name: &str,
    timeout: u64,
    task: impl Future<Output = ()>,
) -> Result<(), String> {
    select! {
        _ = task => {
            Ok(())
        },
        _ = async {
            sleep(Duration::from_millis(timeout)).await;
        } => {
            let err_msg = format!("Timeout ({}) error; task \"{}\"", timeout, name);
            println!("{}", err_msg);
            Err(err_msg)
        }
    }
}
