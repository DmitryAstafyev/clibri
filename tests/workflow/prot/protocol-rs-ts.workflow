
&config {
   SelfKey: StructA;
   AssignedKey: StructC;
   Producer: rust;
   Consumer: typescript;
}

StructA !StructE {
   (CaseB    > StructB)    > StructD;
                           > StructF;
   (CaseC    > StructC);
   (CaseD    > StructD)    > StructJ;
}

StructC !StructE {
   (CaseB    > StructB);
   (CaseF    > StructF);
   (CaseD    > StructD);
}

StructD !StructC {
   (StructA);
}

StructF !StructE {
   (StructF);
}

StructEmpty !StructEmptyA {
   (StructEmptyB);
}

GroupA.StructA !GroupA.StructB {
   (RootA    > StructA) > StructD;
   (RootB    > StructB);
}

GroupA.StructB !GroupA.StructB {
   (GroupBStructA          > GroupB.StructA) > GroupB.GroupC.StructB;
   (GroupBGroupCStructA    > GroupB.GroupC.StructA);
}

GroupB.GroupC.StructA !GroupA.StructB {
   (GroupB.GroupC.StructB);
}

GroupB.StructA !GroupB.GroupC.StructB {
   (GroupBStructA          > GroupB.StructA);
   (GroupBGroupCStructA    > GroupB.GroupC.StructA);
}


GroupB.GroupC.StructB !GroupB.GroupC.StructA {
   (CaseB    > StructB)    > StructD;
                           > StructF;
   (CaseC    > StructC);
   (CaseD    > StructD)    > StructJ;
}

@EventA {
   > StructB;
   > StructC;
}

@EventB {
   > StructC;
}

@Events.EventA {
   > StructA;
   > StructB;
}

@Events.EventB {
   > GroupA.StructA;
   > GroupA.StructB;
   > GroupB.StructA;
}

@Events.Sub.EventA {
   > GroupB.GroupC.StructA;
   > GroupB.GroupC.StructB;
}

@TriggerBeaconsEmitter {
   > TriggerBeacons;
}

@FinishConsumerTest {
   > FinishConsumerTestBroadcast;
}

@beacons {
    > BeaconA;
    > Beacons.BeaconA;
    > Beacons.BeaconB;
    > Beacons.Sub.BeaconA;
    > Beacons.ShutdownServer;
}
