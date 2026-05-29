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

#[test]
fn boolean() {
    let a = true;
    let b = false;

    println!("{} {}", a, b)
}

#[test]
fn comparison_operator() {
    let a = 10;
    let b = 5;

    // lebih dari
    let c = a > b;
    println!("{}", c);

    // kurang dari
    let d = a < b;
    println!("{}", d);

    // lebih dari sama dengan
    let e = a >= b;
    println!("{}", e);

    // kurang dari sama dengan
    let f = a <= b;
    println!("{}", f);

    // sama dengan
    let g = a == b;
    println!("{}", g);

    // lebih ringkas
    let result = 10 > 9;
    println!("{}", result);
}

#[test]
fn operator_dan_atau_kebalikan() {
    /* Operator Dan
    true && true = true
    true && false = false
    false && true = true 
    falsse && false = false*/
    let compare_dan = true && false;
    println!("{}", compare_dan);

    /*Operator Atau 
    true || true = true
    true || false = true
    false || true = true
    false || false = false*/
    let compare_atau = true || false;
    println!("{}", compare_atau);

    /*Operator Kebalikan
    !true = false
    !false = true */
    let compare_kebalikan = !true;
    println!("{}", compare_kebalikan);

}

#[test]
fn boolean_operator() {
    // jika salah satu di 
    let absen = 70;
    let nilai_akhir = 80;

    let lulus = absen >= 75;
    let lulus_nilai_akhir  = nilai_akhir >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;

    print!("{}", lulus_final);
}

#[test]
fn char_type() {
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2)
}

#[test]
fn tuple() {
    let mut data: (i32, f64, bool) = (10, 10.5,  true);
    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    // println!("{}, {}, {}", a, b, c);

    /*destructuring tuple, 
    jika tidak ingin menggunakan salah satu data nya, 
    maka ganti data itu menggunakan _ , 
    contoh c di ganti jadi _ */

    let (a, b, _) = data;
    println!("{} {} ", a, b, );

    data.0 = 20;
    data.1 = 1.5;
    data.2 = false;

    println!("{:?}", data);

}

fn unit() {
    println!("Hello");
}

#[test]
fn unit_test() {
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let array    = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", array);

    let array_bool = [true, false, true, true];
    println!("{:?}", array_bool);

    let a = array[0];
    let b  = array[3];
    let c = array[5];

    println!("{} {} {}", a, b, c);

    // length array
    let lenght = array.len();
    println!("{}", lenght);
}


#[test]
fn mutable_array() {
    let mut mut_array = [10, 7, 23, 12];
    println!("{:?}", mut_array);

    let a = mut_array[1];
    let b = mut_array[3];
    println!("{} {}", a,b);

    mut_array[2] = 20;
    mut_array[0] = 16;

    println!("{:?}", mut_array);
    
}

#[test]
fn two_demensional_array() {
    // array 2 dimensi atau aray dalam array
    let matrix = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix[0][2]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[1][0]);
    println!("{:?}", matrix[1][1]);
    println!("{:?}", matrix[1][2])
}

// Constant
const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MAXIMUM, MINIMUM);
}