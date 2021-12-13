
# B. Duplicate of beacons

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
    > StructB;
    > StructB;
    > StructC;
}
