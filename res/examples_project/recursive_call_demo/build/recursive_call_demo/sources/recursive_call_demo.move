module recursive_call_demo::recursive_call_demo{

    use recursive_call_demo::recursive_call_demo2;

    public fun recursive_call_fun1(a: u8, b: u8) {
        recursive_call_fun1(a,a+b);
    }
    
    public fun recursive_call_fun2(a: u8, b: u8) {
        if (a < 0) {
            recursive_call_fun2(a,a+b);
        } else {
            recursive_call_fun1(a,b);
        }
    }

    public fun recursive_call_fun3(a: u8, b: u8) {
        recursive_call_fun4(a,a+b);
    }

    public fun recursive_call_fun4(a: u8, b: u8) {
        recursive_call_fun3(a,a+b);
    }

    public fun recursive_call_fun5(a: u8, b: u8) {
        recursive_call_demo2::recursive_call_fun5(a,a+b);
    }

}