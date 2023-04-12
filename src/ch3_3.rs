// 함수
pub fn main3_3() {
    // Rust 는 함수의 위치를 신경쓰지 않는다.
    // fn 키워드를 사용해서 함수를 정의한다.

    // 함수 호출 및 함수의 매개변수
    parameter_func(8, 999);

    // {} 표현식
    let block = {
        let x = 10;
        let y = 22;
        // 반환 갑시 없으면 오류
        x + y // ; 없으면 해당 연산 결과를 반환하겠다는 것으로 인식하여 값이 반환됨.
    };
    println!("{}", block);

    // 반환 값을 갖는 함수
    // 반환타입은 -> 를 사용해서 표시한다.
    let return_num = return_func();
    println!("{}", return_num);

    none_return_func();
}

fn return_func() -> i32 {
    // return 10 // return 사용가능!
    // 10; //expected `i32`, found `()` , 종결을 ; 로 선언하면 반환을 하지 않음을 의미
    // 위처럼 ; 로 종결을 지었기 때문에 Java 에서의 void 처럼 되는데 Rust 에서 void 는 (), 즉 empty tuple 이다.
    10 // 이렇게 하면 10이 반환
}

fn none_return_func() /*-> ()*/ {
    // -> () 이렇게 반환타입 없음을 직접 선언해도 되고 생략해도 된다.
    // 기본적으로 반환타입을 작성하지 않으면 void

    let x = 10; // 지역변수
    let y = 20; // 지역변수
    // x + y // 반환타입이 void 즉, () 임의로 오류
    let sum = x + y;
    println!("x: {2} + y : {1}, sum: {0}", sum, y, x);
}

fn parameter_func(x: i8, y: i16) {
    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
}