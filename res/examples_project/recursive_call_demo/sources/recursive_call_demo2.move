module recursive_call_demo::recursive_call_demo2 {
    public fun recursive_call_fun5(a: u8, b: u8) {
        recursive_call_fun5(a,a+b);
    }
    
}