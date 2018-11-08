fn get_str<'a>() -> &'a str {
    "hello"
}
fn f(x: &Vec<u32>){
    for i in x.iter(){
        println!("{}", i)
    }
}

fn g(x: &Vec<u32>){
    for i in x.iter(){
        println!("{}", i)
    }
}

fn main() {
    let x;
    {  
        let y:&'static i32 = &16;
        x = y;
    }                         
    let x = vec![10, 20, 30];
    let c = true;
    if c {
        f(&x); // ... ok to move from x here
    } else {
        g(&x); // ... and ok to also move from x here
    }
    // bad: x is uninitialized here if either path uses it                          
    println!("{:?}", x);  

    let n = (2,3);
    let m = n;
    print!("{:?}", n);      
}                             