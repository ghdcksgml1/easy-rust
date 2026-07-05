fn main() {
    let my_float = 5.; // 기본은 f64

    let my_float: f64 = 5.0;
    let my_other_float: f32 = 8.5;

    // let third_float = my_float + my_other_float; // 에러
    /*
error[E0308]: mismatched types
 --> src/bin/1_5_1_실수.rs:7:34
  |
7 |     let third_float = my_float + my_other_float; // 에러
  |                                  ^^^^^^^^^^^^^^ expected `f64`, found `f32`
     */

    let third_float = my_float + my_other_float as f64;
    println!("{}", third_float);
}