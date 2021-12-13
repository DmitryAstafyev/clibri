
# K. No config

StructA !StructE {
   (CaseB    > StructB) > StructD;
                        > StructF;
   (CaseC    > StructC);
   (CaseD    > StructD) > StructJ;
}

@StructJ {
   > StructB;
   > StructC;
}