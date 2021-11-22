
&config {
   SelfKey: Identification.SelfKey;
   AssignedKey: Identification.AssignedKey;
   Producer: rust;
   Consumer: rust;
}

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

# Broadcast for custom event
@ServerEvents.UserKickOff {
   > Events.Message;
   > Events.UserDisconnected;
}

@ServerEvents.UserAlert {
   > Events.Message;
   > Events.UserConnected?;
}

# No response required messages from client. It's just events on producer side
@beacons {
    > Beacons.LikeUser;
    > Beacons.LikeMessage;
}
