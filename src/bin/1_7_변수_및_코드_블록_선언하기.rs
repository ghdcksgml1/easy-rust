fn main() {
    let my_number = 8;
    println!("Hello, number {}", my_number);
    println!("Hello, number {my_number}");

    // 변수는 코드 블록인 {} 안에서 시작하고 끝남.
    {
        let my_number = 123131;
    }
    println!("Hello, number {my_number}"); // 8이 출력됨.

    // 코드 블록을 사용해 값을 반환할 수도 있음.
    let my_number = { // <- 익명함수 느낌인가?
        let second_number = 23;
        second_number + 3
    };
    println!("Hello, number {}", my_number);

    let my_number = { // <- 익명함수 느낌인가?
        let second_number = 23;
        second_number + 3;
    };
    println!("Hello, number {:?}", my_number); // 결과: Hello, number ()
}