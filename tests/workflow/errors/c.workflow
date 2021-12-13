
# C. Duplicate of request

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

StructA !StructC {
   (StructJ);
}

@beacons {
    > StructB;
    > StructC;
}
