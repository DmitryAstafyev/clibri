
# G. Reference to not exist struct (request)

&config {
   SelfKey: StructB;
   AssignedKey: StructC;
   Producer: rust;
   Consumer: rust;
}

StructA !StructE {
   (CaseB    > NotExistStruct)   > StructD;
                                 > StructF;
   (CaseC    > StructC);
   (CaseD    > StructD)          > StructJ;
}
