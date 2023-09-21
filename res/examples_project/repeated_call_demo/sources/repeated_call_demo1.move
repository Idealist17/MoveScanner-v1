module repeated_call_demo::repeated_call_demo1
{

    public fun demo1_f0() {
    }
    
    public fun demo1_f1(a: u8, b: u8) {
    }
    
    // case2: call self or normal function
    public fun demo1_f2(a: u8, b: u8) {
        if(a>12){
            if(b>1){
                demo1_f0();
            }else{
                demo1_f1(a,b);
                return
            }
        }else{
        demo1_f1(a,b);
        };
        // a=a+1;
        demo1_f1(a,b);
        demo1_f0();
    }

    // pass
    public fun demo1_f3() {
        let i = 0;
        let j = 1;
        demo1_f1(i,i);
        demo1_f1(j,j);
    }
}