use super::{api::Api, error::ConsumerError, protocol, protocol::PackingStruct};
use fiber::client;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub struct Consumer<E: client::Error> {
    api: Api<E>,
    shutdown: CancellationToken,
}
pub enum UserLoginRequestResponse {
    Accept(protocol::UserLogin::Accepted),
    Deny(protocol::UserLogin::Denied),
    Err(protocol::UserLogin::Err),
}
pub enum UsersRequestResponse {
    Response(protocol::Users::Response),
    Err(protocol::Users::Err),
}
pub enum MessageRequestResponse {
    Accept(protocol::Message::Accepted),
    Deny(protocol::Message::Denied),
    Err(protocol::Message::Err),
}
pub enum MessagesRequestResponse {
    Response(protocol::Messages::Response),
    Err(protocol::Messages::Err),
}
impl<E: client::Error> Consumer<E> {
    pub fn new(api: Api<E>) -> Self {
        let shutdown = api.get_shutdown_token();
        Consumer { api, shutdown }
    }    
    pub async fn beacons_likeuser(
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
    pub async fn beacons_likemessage(
        &mut self,
        mut beacon: protocol::Beacons::LikeMessage,
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
    
    pub async fn userlogin_request(
        &mut self,
        mut request: protocol::UserLogin::Request,
    ) -> Result<UserLoginRequestResponse, ConsumerError<E>> {
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
            protocol::AvailableMessages::UserLogin(protocol::UserLogin::AvailableMessages::Accepted(msg)) =>
                Ok(UserLoginRequestResponse::Accept(msg)),
            protocol::AvailableMessages::UserLogin(protocol::UserLogin::AvailableMessages::Denied(msg)) =>
                Ok(UserLoginRequestResponse::Deny(msg)),
            protocol::AvailableMessages::UserLogin(protocol::UserLogin::AvailableMessages::Err(msg)) =>
                Ok(UserLoginRequestResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for UserLogin::Request has been gotten wrong response",
            ))),
        }
    }
    pub async fn users_request(
        &mut self,
        mut request: protocol::Users::Request,
    ) -> Result<UsersRequestResponse, ConsumerError<E>> {
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
            protocol::AvailableMessages::Users(protocol::Users::AvailableMessages::Response(msg)) =>
                Ok(UsersRequestResponse::Response(msg)),
            protocol::AvailableMessages::Users(protocol::Users::AvailableMessages::Err(msg)) =>
                Ok(UsersRequestResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for Users::Request has been gotten wrong response",
            ))),
        }
    }
    pub async fn message_request(
        &mut self,
        mut request: protocol::Message::Request,
    ) -> Result<MessageRequestResponse, ConsumerError<E>> {
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
            protocol::AvailableMessages::Message(protocol::Message::AvailableMessages::Accepted(msg)) =>
                Ok(MessageRequestResponse::Accept(msg)),
            protocol::AvailableMessages::Message(protocol::Message::AvailableMessages::Denied(msg)) =>
                Ok(MessageRequestResponse::Deny(msg)),
            protocol::AvailableMessages::Message(protocol::Message::AvailableMessages::Err(msg)) =>
                Ok(MessageRequestResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for Message::Request has been gotten wrong response",
            ))),
        }
    }
    pub async fn messages_request(
        &mut self,
        mut request: protocol::Messages::Request,
    ) -> Result<MessagesRequestResponse, ConsumerError<E>> {
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
            protocol::AvailableMessages::Messages(protocol::Messages::AvailableMessages::Response(msg)) =>
                Ok(MessagesRequestResponse::Response(msg)),
            protocol::AvailableMessages::Messages(protocol::Messages::AvailableMessages::Err(msg)) =>
                Ok(MessagesRequestResponse::Err(msg)),
            _ => Err(ConsumerError::UnexpectedResponse(String::from(
                "for Messages::Request has been gotten wrong response",
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