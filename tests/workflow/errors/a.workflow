
# A. Beacons and requests should not be in conflict

&config {
   SelfKey: StructA;
   AssignedKey: StructC;
   Producer: rust;
   Consumer: rust;
}

StructA !StructE {
   (CaseB    > StructB)    > StructD;
                           > StructF;
   (CaseC    > StructC);
   (CaseD    > StructD)    > StructJ;
}

@beacons {
    > StructA;
    > StructB;
}
