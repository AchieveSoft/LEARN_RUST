fn variable() {
    let a = 1;
    let b: u32 = 70000;
    let c = true;
    let name: String = String::from("nilphumiphat");
    println!("variable declare {}, {}, {}, {}", name, a, b, c)
}

fn array() {
    let a1: [u8; 3] = [1, 2, 3];
    println!("array {:?}", a1)
}

// function with return value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// function return tuple
fn get_fullname() -> (String, String) {
    return (String::from("Phumiphat"), String::from("Jaroenyonwhatthana"))
}

// Return empty tuple (เรียกอีกอย่างว่า unit)
fn unit_fn() -> () {
    println!("Print from unit function");
    ()
}

fn loop_declare() {
    let mut a = 1;

    loop {
        a += 1;
        println!("Loop count: {}", a);

        if a == 10000 {
            break
        }
    }

    println!("End loop count: {}", a)
}


fn while_declare() {
    let mut a: u8 = 1;

    while a <= 3 {
        println!("While count: {}", a);
        a += 1
    }
}

fn if_else(score: u8) {
    if score <= 49 {
        println!("Grade: F")
    } else if score >= 50 && score <= 59 {
        println!("Grade: D")
    } else if score >= 60 && score <= 69 {
        println!("Grade: C")
    } else if score >= 70 && score <= 79 {
        println!("Grade: B")
    } else if score >= 80 && score <= 89 {
        println!("Grade: A")
    } else if score >= 90 {
        println!("Grade: S")
    } else {
        println!("Can not calculate grade from score: {}", score)
    }
}

fn iterator_simple() {
    for a in 1..3 {
        println!("iter: {}", a)
    }
}

fn match_declare(value: i32) {
    match value {
        0 | 1 => {
            println!("hahaha")
        }

        matched_value @ 0..=100 => {
            println!("between 0 to 100 hahaha. value: {}", matched_value)
        }

        _ => println!("can not match case")
    }
}

fn assign_from_loop() {
    let mut a = 1;

    let b: i32 = loop {
        a += 1;

        if a == 3 {
            break a
        }
    };

    println!("Value from loop: {}", b)
}

fn assign_from_match() {
    let sl = 70000;
    let name = match sl {
        70000..130000 => {
            "nilphumiphat"
        }

        _ => "can't determinate name"
    };

    println!("Value from match: {}", name)
}

#[derive(Debug)]
#[allow(unused)]
struct Product {
    name: String
}

fn struct_use() {
    let product1 = Product { name: String::from("ACE") };
    let product2 = Product { name: String::from("GO") };
    let product3 = Product { name: String::from("ACE Cloud") };

    println!("Product 1: {:?}", product1);
    println!("Product 2: {:?}", product2);
    println!("Product 3: {:?}", product3);
}

enum Cases {
    A,
    B,
    C
}

fn enum_pattern_matching(case: Cases) {
    match case {
        Cases::A => println!("Case: A"),
        Cases::B => println!("Case: B"),
        Cases::C => println!("Case: C"),
    }
}

fn enum_use() {
    let case1 = Cases::A;
    let case2 = Cases::B;
    let case3 = Cases::C;

    enum_pattern_matching(case1);
    enum_pattern_matching(case2);
    enum_pattern_matching(case3);
}


// union enum

#[derive(Debug)]
enum ProductTypes {
    Ace(u64),
    Go,
    AceCloud(String, String),
}

#[derive(Debug)]
struct ProductDetail {
    customer_code: String,
    product_type: ProductTypes
}


// For use gethering value from union enum field. 
fn print_product_detail(detail: ProductDetail) {
    println!("Customer code: {}", detail.customer_code);
    
    match detail.product_type {
        ProductTypes::Ace(subscription_month) => println!("Ace: subscription month: {}", subscription_month),
        ProductTypes::Go => println!("Go: New customer joining"),
        ProductTypes::AceCloud(service, region) => println!("Ace Cloud: service: {} on region: {}", service, region) 
    }
}

fn union_enum_use() {
    let product1 = ProductDetail {
        customer_code: String::from("cus001"),
        product_type: ProductTypes::Ace(9)
    };
    let product2 = ProductDetail {
        customer_code: String::from("cus001"),
        product_type: ProductTypes::Go
    };
    let product3 = ProductDetail {
        customer_code: String::from("cus001"),
        product_type: ProductTypes::AceCloud(String::from("vps"), String::from("th"))
    };

    print_product_detail(product1);
    print_product_detail(product2);
    print_product_detail(product3);
}

fn main() {
    variable();
    
    array();

    println!("{}", add(1, 2));

    let fullname = get_fullname();
    let (name, surname) = &fullname; // Use borrowing to keep name, surname
    println!("Full name: {} {}", name, surname);
    println!("Full name (from tuple index): {} {}", fullname.0, fullname.1);

    unit_fn();

    if_else(100);

    loop_declare();

    while_declare();

    iterator_simple();

    match_declare(100);

    assign_from_loop();

    assign_from_match();

    struct_use();

    enum_use();

    union_enum_use();
}
