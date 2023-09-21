module demo::demo_module2
{
    const UNUSED_CONSTANT_EXAM: u64 = 666;

    public fun f_unnecessary_type_conversion(x: u64){
        let b = (x as u64);
    }

    fun f_unused_private_functions(){

    }

    public fun f_recursive_function_call(){
        f_recursive_function_call();
    }
    

    public fun f_repeated_called(){

    }
    public fun f_repeated_function_call(){
        f_repeated_called();
        f_repeated_called();
    }
    


}