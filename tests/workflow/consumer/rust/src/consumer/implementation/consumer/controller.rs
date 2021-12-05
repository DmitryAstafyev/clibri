use super::{api::Api, error::ConsumerError, protocol, protocol::PackingStruct};
use clibri::client;
use tokio_util::sync::CancellationToken;
use tokio::{
    select,
    time::{sleep, Duration},
};

#[derive(Debug, Clone)]
pub struct Consumer<E: client::Error> {
    api: Api<E>,
    timeout: u64,
}
pub enum StructAResponse {
    CaseB(protocol::StructB),
    CaseC(protocol::StructC),
    CaseD(protocol::StructD),
    Err(protocol::StructE),
}
pub enum StructCResponse {
    CaseB(protocol::StructB),
    CaseF(protocol::StructF),
    CaseD(protocol::StructD),
    Err(protocol::StructE),
}
pub enum StructDResponse {
    Response(protocol::StructA),
    Err(protocol::StructC),
}
pub enum StructFResponse {
    Response(protocol::StructF),
    Err(protocol::StructE),
}
pub enum StructEmptyResponse {
    Response(protocol::StructEmptyB),
    Err(protocol::StructEmptyA),
}
pub enum GroupAStructAResponse {
    RootA(protocol::StructA),
    RootB(protocol::StructB),
    Err(protocol::GroupA::StructB),
}
pub enum GroupAStructBResponse {
    GroupBStructA(protocol::GroupB::StructA),
    GroupBGroupCStructA(protocol::GroupB::GroupC::StructA),
    Err(protocol::GroupA::StructB),
}
pub enum GroupBGroupCStructAResponse {
    Response(protocol::GroupB::GroupC::StructB),
    Err(protocol::GroupA::StructB),
}
pub enum GroupBStructAResponse {
    GroupBStructA(protocol::GroupB::StructA),
    GroupBGroupCStructA(protocol::GroupB::GroupC::StructA),
    Err(protocol::GroupB::GroupC::StructB),
}
pub enum GroupBGroupCStructBResponse {
    CaseB(protocol::StructB),
    CaseC(protocol::StructC),
    CaseD(protocol::StructD),
    Err(protocol::GroupB::GroupC::StructA),
}
impl<E: client::Error> Consumer<E> {
    pub fn new(api: Api<E>, timeout: u64) -> Self {
        Consumer { api, timeout }
    }    
    pub async fn beacon_beacona(
        &self,
        mut beacon: protocol::BeaconA,
    ) -> Result<(), ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let message = self
            .api
            .request(
                sequence,
                &beacon
                    .pack(sequence, uuid)
                    .map_err(ConsumerError::Protocol)?,
        )
        .await?;
        match message {        
            protocol::AvailableMessages::InternalServiceGroup(protocol::InternalServiceGroup::AvailableMessages::BeaconConfirmation(msg)) =>
                if let Some(err) = msg.error {
                    Err(ConsumerError::Broadcast(err.to_owned()))
                } else {
                    Ok(())
                }
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for BeaconA has been gotten wrong response",
            ))),
        }
    }
    pub async fn beacon_beacons_beacona(
        &self,
        mut beacon: protocol::Beacons::BeaconA,
    ) -> Result<(), ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let message = self
            .api
            .request(
                sequence,
                &beacon
                    .pack(sequence, uuid)
                    .map_err(ConsumerError::Protocol)?,
        )
        .await?;
        match message {        
            protocol::AvailableMessages::InternalServiceGroup(protocol::InternalServiceGroup::AvailableMessages::BeaconConfirmation(msg)) =>
                if let Some(err) = msg.error {
                    Err(ConsumerError::Broadcast(err.to_owned()))
                } else {
                    Ok(())
                }
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for Beacons::BeaconA has been gotten wrong response",
            ))),
        }
    }
    pub async fn beacon_beacons_beaconb(
        &self,
        mut beacon: protocol::Beacons::BeaconB,
    ) -> Result<(), ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let message = self
            .api
            .request(
                sequence,
                &beacon
                    .pack(sequence, uuid)
                    .map_err(ConsumerError::Protocol)?,
        )
        .await?;
        match message {        
            protocol::AvailableMessages::InternalServiceGroup(protocol::InternalServiceGroup::AvailableMessages::BeaconConfirmation(msg)) =>
                if let Some(err) = msg.error {
                    Err(ConsumerError::Broadcast(err.to_owned()))
                } else {
                    Ok(())
                }
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for Beacons::BeaconB has been gotten wrong response",
            ))),
        }
    }
    pub async fn beacon_beacons_sub_beacona(
        &self,
        mut beacon: protocol::Beacons::Sub::BeaconA,
    ) -> Result<(), ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let message = self
            .api
            .request(
                sequence,
                &beacon
                    .pack(sequence, uuid)
                    .map_err(ConsumerError::Protocol)?,
        )
        .await?;
        match message {        
            protocol::AvailableMessages::InternalServiceGroup(protocol::InternalServiceGroup::AvailableMessages::BeaconConfirmation(msg)) =>
                if let Some(err) = msg.error {
                    Err(ConsumerError::Broadcast(err.to_owned()))
                } else {
                    Ok(())
                }
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for Beacons::Sub::BeaconA has been gotten wrong response",
            ))),
        }
    }
    pub async fn beacon_beacons_shutdownserver(
        &self,
        mut beacon: protocol::Beacons::ShutdownServer,
    ) -> Result<(), ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let message = self
            .api
            .request(
                sequence,
                &beacon
                    .pack(sequence, uuid)
                    .map_err(ConsumerError::Protocol)?,
        )
        .await?;
        match message {        
            protocol::AvailableMessages::InternalServiceGroup(protocol::InternalServiceGroup::AvailableMessages::BeaconConfirmation(msg)) =>
                if let Some(err) = msg.error {
                    Err(ConsumerError::Broadcast(err.to_owned()))
                } else {
                    Ok(())
                }
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for Beacons::ShutdownServer has been gotten wrong response",
            ))),
        }
    }
    
    pub async fn structa(
        &mut self,
        mut request: protocol::StructA,
    ) -> Result<StructAResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let package = request
            .pack(sequence, uuid)
            .map_err(ConsumerError::Protocol)?;
        let message = select! {
            message = self
            .api
            .request(
                sequence,
                &package,
            ) => message,
            _ = sleep(Duration::from_millis(self.timeout)) => Err(ConsumerError::Timeout)
        }?;
        match message {        
            protocol::AvailableMessages::StructB(msg) =>
                Ok(StructAResponse::CaseB(msg)),
            protocol::AvailableMessages::StructC(msg) =>
                Ok(StructAResponse::CaseC(msg)),
            protocol::AvailableMessages::StructD(msg) =>
                Ok(StructAResponse::CaseD(msg)),
            protocol::AvailableMessages::StructE(msg) =>
                Ok(StructAResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for StructA has been gotten wrong response",
            ))),
        }
    }
    pub async fn structc(
        &mut self,
        mut request: protocol::StructC,
    ) -> Result<StructCResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let package = request
            .pack(sequence, uuid)
            .map_err(ConsumerError::Protocol)?;
        let message = select! {
            message = self
            .api
            .request(
                sequence,
                &package,
            ) => message,
            _ = sleep(Duration::from_millis(self.timeout)) => Err(ConsumerError::Timeout)
        }?;
        match message {        
            protocol::AvailableMessages::StructB(msg) =>
                Ok(StructCResponse::CaseB(msg)),
            protocol::AvailableMessages::StructF(msg) =>
                Ok(StructCResponse::CaseF(msg)),
            protocol::AvailableMessages::StructD(msg) =>
                Ok(StructCResponse::CaseD(msg)),
            protocol::AvailableMessages::StructE(msg) =>
                Ok(StructCResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for StructC has been gotten wrong response",
            ))),
        }
    }
    pub async fn structd(
        &mut self,
        mut request: protocol::StructD,
    ) -> Result<StructDResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let package = request
            .pack(sequence, uuid)
            .map_err(ConsumerError::Protocol)?;
        let message = select! {
            message = self
            .api
            .request(
                sequence,
                &package,
            ) => message,
            _ = sleep(Duration::from_millis(self.timeout)) => Err(ConsumerError::Timeout)
        }?;
        match message {        
            protocol::AvailableMessages::StructA(msg) =>
                Ok(StructDResponse::Response(msg)),
            protocol::AvailableMessages::StructC(msg) =>
                Ok(StructDResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for StructD has been gotten wrong response",
            ))),
        }
    }
    pub async fn structf(
        &mut self,
        mut request: protocol::StructF,
    ) -> Result<StructFResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let package = request
            .pack(sequence, uuid)
            .map_err(ConsumerError::Protocol)?;
        let message = select! {
            message = self
            .api
            .request(
                sequence,
                &package,
            ) => message,
            _ = sleep(Duration::from_millis(self.timeout)) => Err(ConsumerError::Timeout)
        }?;
        match message {        
            protocol::AvailableMessages::StructF(msg) =>
                Ok(StructFResponse::Response(msg)),
            protocol::AvailableMessages::StructE(msg) =>
                Ok(StructFResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for StructF has been gotten wrong response",
            ))),
        }
    }
    pub async fn structempty(
        &mut self,
        mut request: protocol::StructEmpty,
    ) -> Result<StructEmptyResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let package = request
            .pack(sequence, uuid)
            .map_err(ConsumerError::Protocol)?;
        let message = select! {
            message = self
            .api
            .request(
                sequence,
                &package,
            ) => message,
            _ = sleep(Duration::from_millis(self.timeout)) => Err(ConsumerError::Timeout)
        }?;
        match message {        
            protocol::AvailableMessages::StructEmptyB(msg) =>
                Ok(StructEmptyResponse::Response(msg)),
            protocol::AvailableMessages::StructEmptyA(msg) =>
                Ok(StructEmptyResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for StructEmpty has been gotten wrong response",
            ))),
        }
    }
    pub async fn groupa_structa(
        &mut self,
        mut request: protocol::GroupA::StructA,
    ) -> Result<GroupAStructAResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let package = request
            .pack(sequence, uuid)
            .map_err(ConsumerError::Protocol)?;
        let message = select! {
            message = self
            .api
            .request(
                sequence,
                &package,
            ) => message,
            _ = sleep(Duration::from_millis(self.timeout)) => Err(ConsumerError::Timeout)
        }?;
        match message {        
            protocol::AvailableMessages::StructA(msg) =>
                Ok(GroupAStructAResponse::RootA(msg)),
            protocol::AvailableMessages::StructB(msg) =>
                Ok(GroupAStructAResponse::RootB(msg)),
            protocol::AvailableMessages::GroupA(protocol::GroupA::AvailableMessages::StructB(msg)) =>
                Ok(GroupAStructAResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for GroupA::StructA has been gotten wrong response",
            ))),
        }
    }
    pub async fn groupa_structb(
        &mut self,
        mut request: protocol::GroupA::StructB,
    ) -> Result<GroupAStructBResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let package = request
            .pack(sequence, uuid)
            .map_err(ConsumerError::Protocol)?;
        let message = select! {
            message = self
            .api
            .request(
                sequence,
                &package,
            ) => message,
            _ = sleep(Duration::from_millis(self.timeout)) => Err(ConsumerError::Timeout)
        }?;
        match message {        
            protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::StructA(msg)) =>
                Ok(GroupAStructBResponse::GroupBStructA(msg)),
            protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::GroupC(protocol::GroupB::GroupC::AvailableMessages::StructA(msg))) =>
                Ok(GroupAStructBResponse::GroupBGroupCStructA(msg)),
            protocol::AvailableMessages::GroupA(protocol::GroupA::AvailableMessages::StructB(msg)) =>
                Ok(GroupAStructBResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for GroupA::StructB has been gotten wrong response",
            ))),
        }
    }
    pub async fn groupb_groupc_structa(
        &mut self,
        mut request: protocol::GroupB::GroupC::StructA,
    ) -> Result<GroupBGroupCStructAResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let package = request
            .pack(sequence, uuid)
            .map_err(ConsumerError::Protocol)?;
        let message = select! {
            message = self
            .api
            .request(
                sequence,
                &package,
            ) => message,
            _ = sleep(Duration::from_millis(self.timeout)) => Err(ConsumerError::Timeout)
        }?;
        match message {        
            protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::GroupC(protocol::GroupB::GroupC::AvailableMessages::StructB(msg))) =>
                Ok(GroupBGroupCStructAResponse::Response(msg)),
            protocol::AvailableMessages::GroupA(protocol::GroupA::AvailableMessages::StructB(msg)) =>
                Ok(GroupBGroupCStructAResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for GroupB::GroupC::StructA has been gotten wrong response",
            ))),
        }
    }
    pub async fn groupb_structa(
        &mut self,
        mut request: protocol::GroupB::StructA,
    ) -> Result<GroupBStructAResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let package = request
            .pack(sequence, uuid)
            .map_err(ConsumerError::Protocol)?;
        let message = select! {
            message = self
            .api
            .request(
                sequence,
                &package,
            ) => message,
            _ = sleep(Duration::from_millis(self.timeout)) => Err(ConsumerError::Timeout)
        }?;
        match message {        
            protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::StructA(msg)) =>
                Ok(GroupBStructAResponse::GroupBStructA(msg)),
            protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::GroupC(protocol::GroupB::GroupC::AvailableMessages::StructA(msg))) =>
                Ok(GroupBStructAResponse::GroupBGroupCStructA(msg)),
            protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::GroupC(protocol::GroupB::GroupC::AvailableMessages::StructB(msg))) =>
                Ok(GroupBStructAResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for GroupB::StructA has been gotten wrong response",
            ))),
        }
    }
    pub async fn groupb_groupc_structb(
        &mut self,
        mut request: protocol::GroupB::GroupC::StructB,
    ) -> Result<GroupBGroupCStructBResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let package = request
            .pack(sequence, uuid)
            .map_err(ConsumerError::Protocol)?;
        let message = select! {
            message = self
            .api
            .request(
                sequence,
                &package,
            ) => message,
            _ = sleep(Duration::from_millis(self.timeout)) => Err(ConsumerError::Timeout)
        }?;
        match message {        
            protocol::AvailableMessages::StructB(msg) =>
                Ok(GroupBGroupCStructBResponse::CaseB(msg)),
            protocol::AvailableMessages::StructC(msg) =>
                Ok(GroupBGroupCStructBResponse::CaseC(msg)),
            protocol::AvailableMessages::StructD(msg) =>
                Ok(GroupBGroupCStructBResponse::CaseD(msg)),
            protocol::AvailableMessages::GroupB(protocol::GroupB::AvailableMessages::GroupC(protocol::GroupB::GroupC::AvailableMessages::StructA(msg))) =>
                Ok(GroupBGroupCStructBResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for GroupB::GroupC::StructB has been gotten wrong response",
            ))),
        }
    }
    pub async fn shutdown(&self) -> Result<(), ConsumerError<E>> {
        self.api.shutdown().await
    }

    pub fn get_shutdown_token(&self) -> CancellationToken {
        self.api.get_shutdown_token()
    }
}