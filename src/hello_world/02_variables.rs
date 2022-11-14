use std::mem;

fn some_funtction(pair:(i32, bool)) -> (bool, i32){
    let (int_param, bool_param) = pair; //method param pair
    return (bool_param, int_param);
}

fn analyze_array(slices: &[i32]){
    println!("Anaylsing array");
    println!("First element {}", slices[0]);
    println!("# of elements {}", slices.len());
    println!("# of Bytes    {}", mem::size_of_val(slices));
}

fn main(){
    println!("One million is written as {}", 1_000_000u32); //unsiged 32 bit number with value 1000000
    let return_value = some_funtction((1337, true));
    // Formatter missing for // println!("{}", return_value);
    print!("Print Indexed tupple {}", return_value.0);

    let pair_of_pair = ((1,true), (2, false), (3, true));
    println!("Print pair of pair tupple {:?} ", pair_of_pair);

    let (a, b, c) = pair_of_pair;
    println!("{:?}{:?}{:?}", a, b, c);

    // Arrays 
    
    const NUM_ELEMENTS: usize = 5;
    let array: [i32; NUM_ELEMENTS] = [1, 2, 3, 4, 5];  // Type assigment : [i32, num_elments]
    analyze_array(&array);
    analyze_array(&array[1..3]);

    for i in 0..array.len() + 1 { // +1 to execute None inside
        match array.get(i) {                                    //safe array access
            Some(xval) => println!("{}: {}", i, xval),          //, instead of ;
            None => println!("Slow down! {} is to far!", i),    //, instead of ;
        }
    }
}
