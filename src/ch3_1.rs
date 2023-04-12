// 변수와 가변성
pub fn main3_1() {
    // 기본적으로 변수는 변경이 불가능!
    // let x = 10;
    // x = 5; //  cannot assign twice to immutable variable `x`
    // 위처럼 값을 재할당하면 오류가 발생

    // 재할당 하는 방법은 mut 키워드 사용!
    // consider making this binding mutable: `mut x`
    let mut x = 10;
    println!("{x}");
    x = 5;
    println!("{x}");

    // 상수와 변수의 차이
    // 상수에는 mut 키워드 사용 불가, 상수는 불변성 그 자체
    // 상수를 사용하고자 한다면 let 대신 const 키워드 사용
    const MAX_POINTS: u32 = 100_000;
    println!("{MAX_POINTS}");

    // Shadowing
    // 이전에 선언한 변수와 같은 이름의 새로운 변수를 선언 가능!
    let x = 5;
    let x = x + 1; // 5 + 1
    let x = x * 2; // 6 * 2
    println!("{x}"); // 12


    // Shadowing 과 mut 의 차이
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");
    // spaces 라는 같은 이름의 변수에 다른 타입의 값을 넣으려고 할 때
    // 이렇게 Shadowing 을 사용하면 처리가능!

    let mut spaces = "    ";
    println!("{spaces}");
    //spaces = spaces.len(); // expected `&str`, found `usize`
    // 하지만 위처럼 mut 를 사용해서 재할당을 하려고 해도
    // 타입 불일치 오류가 발생!
    spaces = "같은 값으로는 변경 가능!";
    println!("{spaces}");
}