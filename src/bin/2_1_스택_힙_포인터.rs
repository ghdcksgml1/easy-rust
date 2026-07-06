fn main() {
    let my_number = 15;
    let single_reference = &my_number;
    let double_reference = &single_reference;
    println!("{}", double_reference);
}