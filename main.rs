fn main() {
    println!("Hello World");
    println!("Arsenal");
    println!("ist");
    println!("Nummer");
    println!("1");


    //variablen basics

    let mut age: i32 = 17;  // Diese Variable kann verändert werden

    let mut name: String = "Laurentiu";
    name = "Jonas";                         //String


    let mut is_name_jonas: bool = false;    //bool variable


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
    let quotient = a / b;   


    let alt = 20;
    let hat_id = true;

    let erlaubt = alt >= 18 && hat_id;




    //bedingungen 
     let eingeloggt = false;

    if !eingeloggt {
        println!("Bitte einloggen");
    }



