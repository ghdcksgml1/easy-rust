/// 섀도잉이란? <br/>
/// <br/>
/// let을 사용해 이전에 선언한 변수와 동일한 이름으로 새 변수를 선언하는 것을 의미 (덮어씌우기(?))
fn main() {
    let my_number = 8;
    println!("{}", my_number);

    // 이름만 같지 완전 다른 변수가 된거임.
    let my_number = 8.2;
    println!("{}", my_number);

    println!("===");

    another_case();

    println!("===");

    ex();
}

fn another_case() {
    let my_number = 8;
    println!("{}", my_number);

    // 서로다른 블록에 있으면, 둘 다 볼 수 있음.
    {
        let my_number = 8.2;
        println!("{}", my_number);
    }

    println!("{}", my_number);
}

/// 섀도잉 사용 예시 <br/>
/// <br/>
/// 변수에 많은 작업을 하면서 그사이에 변수에 신경 쓰고 싶지 않을 때 유용
fn ex() {
    let final_number = {
        let y = 10;
        let x = 9;
        let x = times_two(x);
        let x = x + y;
        x
    };
    println!("The number is now: {}", final_number);
}

fn times_two(number: i32) -> i32 {
    number * 2
}