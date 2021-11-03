use super::{api::Api, error::ConsumerError, protocol, protocol::PackingStruct};
use fiber::client;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Consumer<E: client::Error> {
    api: Api<E>,
    shutdown: CancellationToken,
}

pub enum RequestMessageResponse {
    Accepted(protocol::Message::Accepted),
    Denied(protocol::Message::Denied),
    Err(protocol::Message::Err),
}

pub enum RequestMessagesResponse {
    Response(protocol::Messages::Response),
    Err(protocol::Messages::Err),
}

pub enum RequestUsersResponse {
    Response(protocol::Users::Response),
    Err(protocol::Users::Err),
}

pub enum RequestUserLoginResponse {
    Accepted(protocol::UserLogin::Accepted),
    Denied(protocol::UserLogin::Denied),
    Err(protocol::UserLogin::Err),
}

impl<E: client::Error> Consumer<E> {
    pub fn new(api: Api<E>) -> Self {
        let shutdown = api.get_shutdown_token();
        Consumer { api, shutdown }
    }

    pub async fn beacon_like_user(
        &mut self,
        mut beacon: protocol::Beacons::LikeUser,
    ) -> Result<(), ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        self.api
            .send(
                &beacon
                    .pack(sequence, uuid)
                    .map_err(ConsumerError::Protocol)?,
            )
            .await
    }

    pub async fn request_message(
        &mut self,
        mut request: protocol::Message::Request,
    ) -> Result<RequestMessageResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let message = self
            .api
            .request(
                sequence,
                &request
                    .pack(sequence, uuid)
                    .map_err(ConsumerError::Protocol)?,
            )
            .await?;
        match message {
            protocol::AvailableMessages::Message(
                protocol::Message::AvailableMessages::Accepted(msg),
            ) => Ok(RequestMessageResponse::Accepted(msg)),
            protocol::AvailableMessages::Message(protocol::Message::AvailableMessages::Denied(
                msg,
            )) => Ok(RequestMessageResponse::Denied(msg)),
            protocol::AvailableMessages::Message(protocol::Message::AvailableMessages::Err(
                msg,
            )) => Ok(RequestMessageResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for Message::Request has been gotten wrong response",
            ))),
        }
    }

    pub async fn request_messages(
        &mut self,
        mut request: protocol::Messages::Request,
    ) -> Result<RequestMessagesResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let message = self
            .api
            .request(
                sequence,
                &request
                    .pack(sequence, uuid)
                    .map_err(ConsumerError::Protocol)?,
            )
            .await?;
        match message {
            protocol::AvailableMessages::Messages(
                protocol::Messages::AvailableMessages::Response(msg),
            ) => Ok(RequestMessagesResponse::Response(msg)),
            protocol::AvailableMessages::Messages(protocol::Messages::AvailableMessages::Err(
                msg,
            )) => Ok(RequestMessagesResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for Messages::Request has been gotten wrong response",
            ))),
        }
    }

    pub async fn request_users(
        &mut self,
        mut request: protocol::Users::Request,
    ) -> Result<RequestUsersResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        let uuid = self.api.uuid_as_string().await?;
        let message = self
            .api
            .request(
                sequence,
                &request
                    .pack(sequence, uuid)
                    .map_err(ConsumerError::Protocol)?,
            )
            .await?;
        match message {
            protocol::AvailableMessages::Users(protocol::Users::AvailableMessages::Response(
                msg,
            )) => Ok(RequestUsersResponse::Response(msg)),
            protocol::AvailableMessages::Users(protocol::Users::AvailableMessages::Err(msg)) => {
                Ok(RequestUsersResponse::Err(msg))
            }
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for Messages::Request has been gotten wrong response",
            ))),
        }
    }

    pub async fn request_userlogin(
        &mut self,
        mut request: protocol::UserLogin::Request,
    ) -> Result<RequestUserLoginResponse, ConsumerError<E>> {
        let sequence = self.api.sequence().await?;
        println!("sequence: {:?}", sequence);
        let uuid = self.api.uuid_as_string().await?;
        println!("uuid: {:?}", uuid);
        let message = self
            .api
            .request(
                sequence,
                &request
                    .pack(sequence, uuid)
                    .map_err(ConsumerError::Protocol)?,
            )
            .await?;
        match message {
            protocol::AvailableMessages::UserLogin(
                protocol::UserLogin::AvailableMessages::Accepted(msg),
            ) => Ok(RequestUserLoginResponse::Accepted(msg)),
            protocol::AvailableMessages::UserLogin(
                protocol::UserLogin::AvailableMessages::Denied(msg),
            ) => Ok(RequestUserLoginResponse::Denied(msg)),
            protocol::AvailableMessages::UserLogin(
                protocol::UserLogin::AvailableMessages::Err(msg),
            ) => Ok(RequestUserLoginResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for UserLogin::Request has been gotten wrong response",
            ))),
        }
    }

    pub fn shutdown(&self) {
        self.shutdown.cancel();
    }

    pub fn get_shutdown_token(&self) -> CancellationToken {
        self.shutdown.clone()
    }
}
