module demo::demo_module1
{


    public fun f_has_return() : u8{
        1
    }
    
    public fun f_inifinite_loop() {
        while(true){

        }
    }

    public fun f_overlfow(){
        let i = 1 << 200;
    }

    public fun f_precision_loss(arg:u8){
        let a = 13;
        let b = 11;
        let i = a/arg*b;
    }

    public fun f_uncheck_return(){
        f_has_return();
    }

    public fun f_unnecessary_bool_judgment(arg:bool){
        if (arg==true) {
        }
    }

}