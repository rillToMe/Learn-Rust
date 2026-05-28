fn main() {
   println!("halo test 123");
   println!("botol ijo");

   print!("test ");
   print!("test2");
}


#[test]
fn hello_test() {
    println!("hello world test");
    println!("halo test 123");
    println!("botol ijo");

    print!("test ");
    print!("test2");
}

// imuntable varible
#[test]
fn imuntable_variable() {
    let name = "adit";
    println!("adit {}", name);
}

// mutable variable
#[test]
fn mutable_variable(){
    let mut name = "adit";
    println!("hello {}", name);

    name = "budi";
    println!("hello {}", name);
}

#[test]
fn shadowing() {
    let name = "adit";
    println!("hello {}", name);

    let name = 10;
    println!("angka {}", name);
}

#[test]
fn number() {
    let a = 20;
    println!("your {} age", a);

    let b = 2.0;
    println!("decimal {}", b);
}


#[test]
fn number_conversion() {
    let a: i8 =  10;
    println!("{}", a);

    let b: i32 = a as i32;
    println!("{}", b);

    let c: i64 = b as i64;
    println!("{}", c);

    // interger overflow
    let d: i64 = 10000000000000;
    
    let e: i8 = d as i8;
    println!("{}", e);
}

#[test] 
fn numeric_operator() {
    let a = 10;
    let b = 10;

    let c = a - b;
    let d = a + b;
    let e = a * b;
    let f = a / b;
    let g = a % b;

    println!("kurang: {c}, tambah: {d}, kali: {e}, bagi: {f}, sisa bagi: {g}");
}

#[test]
fn augmented_assignment() {
    let mut a = 20;
    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);

    a *= 10;
    println!("{}", a);

    a /= 10;
    println!("{}", a);

    a %= 10;
    println!{"{}", a};
}