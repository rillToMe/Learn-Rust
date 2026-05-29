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

//Variable Scope
 #[test]
 fn variable_scope() {
    let adit = 1;

    { // inner scope
        println!("Ada berapa Adit:{}", adit);
        let budi = 1;
        println!("Ada berapa Budi: {}", budi);
    }

    // println!("{} {}", adit , budi); //error jika budi di print di luar scope nya
 }

 #[test]
 fn stack_heap() {
    function_a();
    function_b();
 }

 fn function_a() {
    let a = 10;
    let b = String::from("Adit");
    println!("{} {}", a, b);
 }

 fn function_b() {
    let a = 20;
    let b = String::from("Budi");
    println!("{} {}", a, b);
 }

 //String slice
 #[test]
 fn string_slice() {
    let name =  "  Rahmat Aditya  ";
    let trim = name.trim();

    println!("{}", name);
    println!("{}", trim);

    let mut username = "Adit";
    println!("{}", username);

    username = "Budi";
    println!("{}", username);

    let mut value = 10;
    println!("{}", value);
    value = 11;
    println!("{}", value);
 }

 //String
 #[test]
 fn string_type() {
    let mut name = String::from("Rahmat Budi");
    println!("{}", name);

    name.push_str(" Santoso");
    println!("{}", name);

    let andito = name.replace("Rahmat", "Andito");
    println!("{}", andito);
 }

 // OwnerShip Rules
 #[test]
 fn ownership_rules() {
    // a belum bisa di pakai karena belum di deklarasikan disini
     let a = 10; // a sudah bisa di pakai karena sudah di deklarasikan

     {// b belum bisa di pakai karena belum di deklarasikan di sini
        let b = 20; // b sudah bisa di gunakan karena b sudah di deklarasikan di sini

        println!("{} {}", a, b);
     }// b sudah tidak bisa di pakai di sini karena  scope sudah di tutup


     println!("{}", a); // a bisa di pakai karena masih  di function scope yang sama
 }

// Data Copy (Khusus untuk type stack) tidak untuk heap
#[test]
fn data_copy() {
    let a = 50;
    let b =  a; // value let b adalah hasil copy dari let a

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Adit");

    let name2 = name1; //ownership name1 pindah ke name2

    println!("{}", name2);
    // println!("{}", name1);   

    /*name1 sudah tidak bisa di pakai 
    karena sudah di pindahkan ownernya ke  name2*/
}

//khusus untuk type data heap, meng clone variable 
#[test]
fn clone() {
    let name1 = String::from("Adit");

    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

// IF EXPRESSION
#[test]
fn if_expression() {
    let value = 9;
    
    //manual
    let result1: &str;
    if value >= 8 {
        result1 = "Very Good";
    } else if value >= 6 {
        result1 = "Not Bad";
    } else if value >=3 {
        result1 = "Bad";
    } else {
        result1 = "Very Bad";
    }

    println!("{}", result1);

    // otomatis
    let result2 = if value >= 8 {
        "Very Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    println!("{}", result2)
}

#[test]
fn loop_expression() {
    let mut data = 0;

    loop {
        data += 1;

        if data > 10 {
            break;
        } else if data % 2 == 0{
            continue;
        }

        println!("Data: {}", data);
    }
}

#[test]
fn loop_return_value() {
    let mut data = 0;

    let result = loop {
        data += 1;
        if data > 10 {
            break data * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
     'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
     }
}

//While Loop 
#[test]
fn while_loop() {
    let mut data = 0;

    while data <= 10 {
        if data % 2 == 0 {
            println!("Data: {}", data);
        }
        data += 1;
    }
}

#[test]
fn array_iteration() {
    let array = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("Value: {}", array[index]);
        index += 1;
    }
}

// For Loop
#[test]
fn for_loop() {
    let array = ["A", "B", "C", "D", "E"];

    for value in array {
        println!("Value: {}", value);
    }
}

#[test]
fn range() {
    // let array = ["A", "B", "C", "D", "E"];

    let range = 0..5;

    println!("Start: {}", range.start);
    println!("End: {}", range.end);

    for i in range {
        println!("Range: {}", i)
    }
}

#[test]
fn range_exclusive() {
    let range = 0..=4;
    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    let array = ["A", "B", "C", "D", "E"];

    for i in range {
        println!("Range: {}", array[i])
    }
}

//Function