
# H. Reference to not exist struct (request)

&config {
   SelfKey: StructB;
   AssignedKey: StructC;
   Producer: rust;
   Consumer: rust;
}

StructA !StructE {
   (CaseB    > StructB)    > StructD;
                           > NotExistStruct;
   (CaseC    > StructC);
   (CaseD    > StructD)    > StructJ;
}
