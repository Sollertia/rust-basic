// 제어문과 반복문
pub fn main3_4() {
    // if, else, else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let 구문에서 제어문 사용하기
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // 주의할점! if 문에서 모든 구역의 반환타입이 동일해야한다.
    // let condition = true;
    //
    // let number = if condition {
    //     5
    // } else {
    //     "six" // expected integer, found `&str`
    // };
    //
    // println!("The value of number is: {}", number);


    // 반복문 : loop, while, for

    // loop : loop keyword 는 Rust 에게 그만두라고 명시하여 알려주기 전까지 코드 블럭을 반복 수행
    // loop {
    //     println!("again!");
    // }

    // while : 조건이 참인 동안 반복문을 수행 , break 을 호출하여 반복을 정지
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // range
    for number in (1..5).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}