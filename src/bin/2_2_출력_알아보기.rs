fn main() {
    print!("\t Start with a tab\nand move to a new line");

    println!();
    println!("Inside quotes
you can write over
many lines
and it will print just fine.");

    println!("Here are two escape characters: \\n and \\t");
    println!(r#"He said, "You can find the file at c:\files\my_documents\file.txt."
    Then I found the file."#);

    let r#let = 6;
    let mut r#mut = 10;

    println!("{:?}", b"This will look like numbers");
    println!("{:X}", '행' as u32);

    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);

    let number = 555;
    println!("Binary: {:b}, hexadecimal: {:x}, octal: {:o}", number, number, number);

    println!(
        "{city1} is in {country} and {city2} is also in {country},\
        but {city3} is not in {country}.",
        city1 = "Seoul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea"
    );
}