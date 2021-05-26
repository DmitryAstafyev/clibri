
&config {
   SelfKey: Identification.SelfKey;
   AssignedKey: Identification.AssignedKey;
   Producer: rust;
   Consumer: typescript;
}

UserLogin.Request !UserLogin.Err {
   (Accept    > UserLogin.Accepted) > Events.UserConnected;
                                    > Events.Message;
                                    > Events.AdminConnected?;
   (Deny      > UserLogin.Denied);
}

Users.Request !Users.Err {
   (Response);
}

Message.Request !Message.Err {
   (Accept    > Message.Accepted) > Events.Message;
   (Deny      > Message.Denied);
}

Messages.Request !Messages.Err {
   (Response);
}

@KickOff {
    reason: str
} > Events::Message;
  > Events::UserConnected;

# If messages are defined in group
# UserLogin {
#     Request !Err;
#     (Accept > Accepted) > Events.UserConnected;
#                         > Events.Message;
#                         > Events.AdminConnected?;
#     (Deny   > Denied);
# }

# @startuml
# Consumer -> Producer: UserLogin.Request
# Producer -->x Consumer: UserLogin.Err
# alt Accepted
#     Producer -> Consumer: UserLogin.Accepted
#     Producer ->]: Events.UserConnected
#     Producer ->]: Events.Message
#     Producer -->]: Events.AdminConnected
# end
# alt Denied
#     Producer -> Consumer: Denied
# end
# UserLogin.@enduml
