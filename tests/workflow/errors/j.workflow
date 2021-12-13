
# I. Reference to not exist struct (event)

&config {
   SelfKey: StructB;
   AssignedKey: StructC;
   Producer: rust;
   Consumer: rust;
}

StructA !StructE {
   (CaseB    > StructB) > StructD;
                        > StructF;
   (CaseC    > StructC);
   (CaseD    > StructD) > StructJ;
}

@NotExistStruct {
   > StructB;
   > StructC;
}