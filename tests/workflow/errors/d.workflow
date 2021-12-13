
# D. Reference to not exist struct (assigned)

&config {
   SelfKey: StructB;
   AssignedKey: NotExistStruct;
   Producer: rust;
   Consumer: rust;
}

StructA !StructE {
   (CaseB    > StructB)    > StructD;
                           > StructF;
   (CaseC    > StructC);
   (CaseD    > StructD)    > StructJ;
}
