fn main() {
    println!("Hello, world!");

    // println! -> 콘솔에 출력하는 매크로
    // 매크로는 코드를 쓰는 함수와 같은데 뒤에 !가 붙는다.

    println!("Hello, world number {}!", 8);
    println!("Hello, world number {} and {}!", 8, 9);

    println!("Hello, world number {}!", number());

    multiply(8, 9);
    let some_number = 10;
    let some_other_number = 2;
    multiply(some_number, some_other_number);

    let multiply_result = multiply(some_number, some_other_number);
}

fn number() -> i32 {
    8 // 세미콜론을 넣으면 아무것도 반환하지 않음 (unit 타입)
}

fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times  {} is {}", number_one, number_two, result);
}

fn multiply2(number_one: i32, number_two: i32) -> i32 {
    let result = number_one * number_two;
    println!("{} times  {} is {}", number_one, number_two, result);
    result
}
