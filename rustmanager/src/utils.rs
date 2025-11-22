


pub fn test_2(){
    let string: &str = "DJKASDLADJSKLADJLAKSSDJLAKDSJSALKSDJALKDSSJALKDJALKDSJL";
    let ptr = string.as_ptr();
    let len = string.len();
    println!("Address: {:p}", ptr);
    println!("Size of my value: {}", len);
}


pub fn obt_test_2_info(){
    let ptr: fn() = test_2;
    let size = size_of_val(&ptr);
    println!("Address of test_2:{:p}", ptr);
    println!("Size of the ptr: {}", size);



}
