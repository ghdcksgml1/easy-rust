fn main() {
    // let 은 불변성 변수
    let my_number = 8;
    // my_number = 10; // 에러!

    // mut을 붙이면, 변경 가능
    let mut my_number = 8;
    my_number = 10;

    // 타입은 변경할 수 없음.
    let mut my_variable = 8;
    // my_variable = "Hello, World!"; // 에러! my_variable은 i32니까
}