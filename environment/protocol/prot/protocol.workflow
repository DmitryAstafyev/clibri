
&config {
   SelfKey: Identification.SelfKey;
   AssignedKey: Identification.AssignedKey;
   Producer: rust;
   Consumer: typescript;
}

#UserLogin.Request !UserLogin.Err {
#   (Accept    > UserLogin.Accepted) > Events.UserConnected;
#                                    > Events.Message;
#                                    > Events.AdminConnected?;
#   (Deny      > UserLogin.Denied);
#}

UserLogin.Request !UserLogin.Err {
   (Accept    > UserLogin.Accepted) > Events.UserConnected;
                                    > Events.Message;
   (Deny      > UserLogin.Denied);
}

Users.Request !Users.Err {
   (Users.Response);
}

Message.Request !Message.Err {
   (Accept    > Message.Accepted) > Events.Message;
   (Deny      > Message.Denied);
}

Messages.Request !Messages.Err {
   (Messages.Response);
}

# Broadcast for default event
@disconnected {
   > Events.Message?;
   > Events.UserDisconnected;
}

@ServerEvents.UserKickOff {
   > Events.Message;
   > Events.UserDisconnected;
}

@ServerEvents.UserAlert {
   > Events.Message;
   > Events.UserConnected?;
}

# No response required messages from client. It has to be events on producer side
@beacons {
   > Events.UserDisconnected;
   > Events.Message;
}

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
