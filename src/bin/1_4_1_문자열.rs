fn main() {
    // len은 바이트 수를 리턴하니까 조심 (글자수가 아님)
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of string containing 'a': {}", "a".len());
    println!("Size of string containing '아': {}", "아".len());
    println!("Size of string containing '😮‍💨': {}", "😮‍💨".len());

    // 문자열의 글자수를 알아내는 방법
    let slice1 = "Hello!";
    println!("Slice1 is {} bytes and also {} characters.", slice1.len(), slice1.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}