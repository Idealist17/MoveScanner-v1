module repeated_call_demo::repeated_call_demo5
{

    public fun demo5_f0():u8 {
        1
    }
    public fun demo5_f1(a:u8){
    }

    // wrong
    public fun demo5_f2() {
        let a = demo5_f0();
        demo5_f1(a);
        demo5_f1(a);
    }

    // todo
    public fun demo5_f3() {
        demo5_f1(demo5_f0());
        demo5_f1(demo5_f0());
    }


}