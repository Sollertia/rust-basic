// 참조자와 빌림
pub fn main4_2() {
    let s = String::from("Robbie");

    let len = calculate_length(&s);

    println!("The length of '{}' is {}.", s, len);

    // 참조자를 통해 빌린 값을 고치려고 할 때
    change(&s);
    // 변수가 기본적으로 불변인 것처럼, 참조자도 마찬가지입니다.
    // 참조하는 어떤 것을 변경하는 것은 허용되지 않습니다.

    // 가변 참조자
    let mut s = String::from("hello");

    change2(&mut s);
    println!("가변 참조자 : {}", s);

    // 주의!
    // 특정한 스코프 내에 특정한 데이터 조각에 대한 가변 참조자를 딱 하나만 만들 수 있다.
    //let mut at = String::from("hello");

    //let r1 = &mut at;
    //let r2 = &mut at;

    //println!("{}, {}", r1, r2);
    // cannot borrow `at` as mutable more than once at a time

    // 이렇게 제한함으로써 얻을 수 있는 이점은 컴파일 타임에 데이터 레이스(data race)를 방지
    // 데이터 레이스(data race)가 발생하는 조건
    // 1. 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
    // 2. 그 중 적어도 하나의 포인터가 데이터를 쓴다.
    // 3. 데이터에 접근하는데 동기화를 하는 어떠한 메커니즘도 없다.

    // 새로운 스코프를 만들어서 처리!
    let mut at = String::from("hello");
    {
        let r1 = &mut at;
        println!("{}", r1)
    } // 여기서 r1은 스코프 밖으로 벗어났으므로, 우리는 아무 문제 없이 새로운 참조자를 만들 수 있습니다.
    let r2 = &mut at;

    println!("{}", r2);


    // 주의!
    // 가변 참조자와 불변 참조자를 혼용할 경우!
    //let mut s = String::from("hello");

    //let r1 = &s; // 문제 없음
    //immutable borrow occurs here
    //let r2 = &s; // 문제 없음
    //let r3 = &mut s; // 큰 문제
    // mutable borrow occurs here
    //println!("{0}, {1}, {2}", r1, r2, r3 );


    // Dangling References
    // 댕글링 포인터란 어떤 메모리를 가리키는 포인터를 보존하는 동안,
    // 그 메모리를 해제함으로써 다른 개체에게 사용하도록 줘버렸을 지도 모를 메모리를 참조하고 있는 포인터를 말합니다.
    // 러스트에서는 컴파일러가 모든 참조자들이 댕글링 참조자가 되지 않도록 보장해 줍니다.

    // let reference_to_nothing = dangle();
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from.
    // 이 함수의 반환 타입은 빌린 값을 포함하고 있는데, 빌려온 실제 값은 없습니다.
    // 결론은 러스트는 이러한 댕글링 포인터가 생기지 않도록 보장합니다.

    // 참조자의 규칙
    // 1. 하나의 가변 참조자 혹은 임의 개수의 불변 참조자들
    // 2. 참조자는 항상 유효해만 한다.

}

fn calculate_length(s: &String) -> usize { // s는 String 의 참조자입니다
    s.len()
} // 여기서 s는 스코프 밖으로 벗어났습니다. 하지만 가리키고 있는 값에 대한 소유권이 없기 때문에, 아무일도 발생하지 않습니다.


fn change(some_string: &String) {
    println!("{}", some_string);
    // some_string.push_str(", world");
    // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}
fn change2(some_string: &mut String) {
    some_string.push_str(" world");
}

//fn dangle() -> &String { // dangle은 String의 참조자를 반환합니다 , expected named lifetime parameter

  //  let s = String::from("hello"); // s는 새로운 String입니다

 //   &s // 우리는 String s의 참조자를 반환합니다.
//} // 여기서 s는 스코프를 벗어나고 버려집니다.