use std::collections::HashMap;

fn main() {
    println!("Hello World");
    println!("Arsenal");
    println!("ist");
    println!("Nummer");
    println!("1");


    //variablen basics

    let mut age: i32 = 17;  // Diese Variable kann verändert werden

    let mut name: String = String::from("Laurentiu");
    name = String::from("Jonas");                         //String

    let mut is_name_jonas: bool = false;    //bool variable
    is_name_jonas = name == "Jonas";


    let a: i32 = 10;
    let b: f64 = a as f64; // type casting ( typ konvertieren)

    let c: i32 = 5;
    let d = c as f64 + b;


    //operatoren basics (Das meiste ziemlich gleich wie bei C#)

    let a = 10;
    let b = 3;

    let sum = a + b;
    let diff = a - b;
    let product = a * b;
    let quotient = a / b;   // ganzzahlig (10/3 = 3)


    let alt = 20;
    let hat_id = true;

    let erlaubt = alt >= 18 && hat_id;


    // bedingungen 
    let eingeloggt = false;

    if !eingeloggt {
        println!("Einloggen");
    } else if erlaubt {
        println!("Du darfst rein");
    } else {
        println!("Kein Zutritt");
    }

   
    let hat_vip = false;
    if erlaubt || hat_vip {
        println!("Zutritt entweder über VIP oder Alter+ID");
    }

    // match (switch)
    let note: i32 = 5;
    match note {
        6 => println!("Note 6: sehr gut"),
        5 => println!("Note 5: gut"),
        4 => println!("Note 4: reicht so lala"),
        1..=3 => println!("Unter 4: traurig aber nicht bestanden"),
        _ => println!("Ungültig"),
    }

    // match mit bool
    match is_name_jonas {
        true => println!("Name ist Jonas"),
        false => println!("Name ist nicht Jonas"),
    }


    // schleifen 

    // loop (läuft bis break)
    let mut counter = 0;
    loop {
        counter += 1;

        if counter == 2 {
            // continue = springt zum nächsten loop-durchlauf
            continue;
        }

        println!("loop counter: {}", counter);

        if counter >= 4 {
            break;
        }
    }

    // while
    let mut tries = 3;
    while tries > 0 {
        println!("tries left: {}", tries);
        tries -= 1;
    }

    // for
    for i in 1..=5 {
        println!("for i: {}", i);
    }


  // funktionen
    let x = add(10, 5);
    println!("add(10,5) = {}", x);

    let ok = is_adult(alt);
    println!("is_adult({}) = {}", alt, ok);

    greet(&name);


    // collections (Vec, HashMap)

    // Vec (Liste)
    let mut numbers: Vec<i32> = vec![1, 2, 3];
    numbers.push(4);
    numbers.push(5);

    // durchgehen
    for n in &numbers {
        println!("vec item: {}", n);
    }

    // index zugriff 
    match numbers.get(2) {
        Some(value) => println!("numbers[2] = {}", value),
        None => println!("kein element an index 2"),
    }

    // HashMap (key -> value)
    let mut points: HashMap<String, i32> = HashMap::new();
    points.insert(String::from("Jonas"), 10);
    points.insert(String::from("Laurentiu"), 7);

    // auslesen mit get()
    match points.get("Jonas") {
        Some(p) => println!("Jonas points: {}", p),
        None => println!("Jonas nicht gefunden"),
    }

    // update (wenn key nicht existiert -> einfügen)
    let key = String::from("Jonas");
    let entry = points.entry(key).or_insert(0);
    *entry += 1;


}


// funktionen

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn is_adult(age: i32) -> bool {
    age >= 18
}

fn greet(name: &str) {
    println!("Hallo {}", name);
}
