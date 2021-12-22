
# F. Reference to not exist struct (request)

&config {
   SelfKey: StructB;
   AssignedKey: StructC;
   Producer: rust;
   Consumer: rust;
}

StructA !NotExistStruct {
   (CaseB    > StructB)    > StructD;
                           > StructF;
   (CaseC    > StructC);
   (CaseD    > StructD)    > StructJ;
}
