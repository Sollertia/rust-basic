// 소유권
pub fn main4_1() {
    // 소유권의 규칙
    // 1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
    // 2. 한번에 딱 하나의 오너만 존재할 수 있다.
    // 3. 오너가 스코프 밖으로 벗어날 때, 값은 버려진다.(dropped)

    // 변수의 스코프
    {                      // s는 유효하지 않습니다. 아직 선언이 안됐거든요.
        let s = "hello";   // s는 이 지점부터 유효합니다.
        // s += "world"; // 기존 값 뒤에 추가 불가능!
        println!("안녕: {}", s);
    }                      // 이 스코프는 이제 끝이므로, s는 더이상 유효하지 않습니다.

    // String
    // 위 블럭에 선언된 s는 불변입니다.
    // 하지만 일반적으로 String 값은 고정되어있지 않기 때문에 변경이 가능한 heap 에 데이터를 저장!
    let mut s = String::from("hello"); // from 함수를 사용해서 String 생성
    println!("{}", s);
    s.push_str(" world"); // 문자열 이어붙이기
    println!("{}", s);

    // String 타입은 변경 가능하고 커질 수 있는 텍스트를 지원하기 위해 만들어졌고,
    // 우리는 힙에서 컴파일 타임에는 알 수 없는 어느 정도 크기의 메모리 공간을 할당받아 내용물을 저장할 필요가 있다.
    // 1. 런타임에 운영체제로부터 메모리가 요청되어야 한다.
    // => String::from 을 호출하면, 구현부분에서 필요한 만큼의 메모리를 요청한다.
    // 2. String 의 사용이 끝났을 때 운영체제에게 메모리를 반납할 방법이 필요하다.
    // GC가 없을 경우, 할당받은 메모리가 더 필요없는 시점을 알아서 명시적으로 이를 반납하는 코드 필요

    // Rust 가 메모리 반납을 다루는 방법
    // 1. 변수가 소속되어 있는 스코프 밖으로 벗어나면 그 순간 자동으로 메모리 반납
    {
        let s = String::from("memory"); // s는 여기서부터 유효합니다
        println!("{}", s);
    } // 스코프 종료 s는 더이상 유효하지 않음, 메모리 반납
    // } 괄호가 닫힐때 Rust 는 drop 함수를 자동으로 호출하여 메모리 반납

    // 변수와 데이터 상호작용
    // 1. 이동
    let x = 5;
    let y = x;
    println!("{}", y);
    // x의 값 5를 복사하여 y에 대입 , 값이 5인 x, y 생성됨.

    let s1 = String::from("Robbie");
    let s2 = s1; // s1 이 바라보고 있는 heap 의 주소가 복사됨.
    println!("s2: {}", s2);
    // 이때 생각해볼 부분은 s2와 s1이 스코프 밖으로 벗어나게 되면, 둘 다 같은 메모리를 해제하려 할 것입니다.
    // 그렇게되면 메모리 손상이 일어나고 보안에 취약해질 수 있습니다.

    // 이때, Java 에서는 s1, s2 모두 사용이 가능하지만 Rust 에서는 이를 해결하기 위해
    // s1를 더이상 유효하지 않는다고 간주하여 스코프가 벗어나도 해제할 필요가 없게 만듭니다.

    //let s1 = String::from("hello");
    //let s2 = s1;
    //println!("{}, world!", s1); // value borrowed here after move
    // 더이상 사용하지 않는 변수라고 오류가 발생!

    // 이러한 상황을 Java 에서는 포인터와 길이값 및 용량값만을 복사한다는 개념인 얕은 복사라고 표현합니다.
    // 하지만 Rust 에서 첫 번째 변수를 무효화 시키기 때문에 이동이라고 부릅니다.

    // 2. 클론 : 깊은 복사
    let s1 = String::from("Robbie");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Copy 가 가능한 타입
    // u32와 같은 모든 정수형 타입들
    // true와 false값을 갖는 부울린 타입 bool
    // f64와 같은 모든 부동 소수점 타입들
    // Copy가 가능한 타입만으로 구성된 튜플들. (i32, i32)는 Copy가 되지만, (i32, String)은 안됩니다.


    // 소유권과 함수
    let s = String::from("Robbie Owner");  // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s);
    // s의 값이 함수 안으로 이동했습니다...
    // ... 그리고 이제 더이상 유효하지 않습니다.
    // println!("s: {}", s); // this parameter takes ownership of the value

    let x = 5; // x가 스코프 안으로 들어왔습니다.

    makes_copy(x); // x가 함수 안으로 이동했습니다만,
    // i32는 Copy가 되므로, x를 이후에 계속 사용해도 됩니다.
    println!("Again X: {}", x);


    // 반환 값과 스코프
    // 값의 반환 또는 소유권을 이동!
    let s1 = gives_ownership();
    println!("{s1}");
    // gives_ownership은 반환값을 s1에게 이동시킵니다.

    let s2 = String::from("hello");
    // s2가 스코프 안에 들어왔습니다.

    let s3 = takes_and_gives_back(s2);
    println!("{s3}");
    // s2는 takes_and_gives_back 안으로
    // 이동되었고, 이 함수가 반환값을 s3으로도 이동시켰습니다.

    // main4_1 함수 마지막 블록에서 s3는 스코프 밖으로 벗어났으며 drop 이 호출됩니다. s2는 스코프 밖으로
    // 벗어났지만 이동되었으므로 아무 일도 일어나지 않습니다. s1은 스코프 밖으로
    // 벗어나서 drop 이 호출됩니다.
}

fn takes_ownership(some_string: String) { // some_string 이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string 이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는 해제되었습니다.

fn makes_copy(some_integer: i32) { // some_integer 이 스코프 안으로 들어왔습니다.
    println!("{}", some_integer);
} // 여기서 some_integer 가 스코프 밖으로 벗어났습니다. 별다른 일은 발생하지 않습니다.


fn gives_ownership() -> String {
    // gives_ownership 함수가 반환 값을
    // 호출한 쪽으로 이동시킵니다.

    let some_string = String::from("hello"); // some_string 이 스코프 안에 들어왔습니다.

    some_string
    // some_string 이 반환되고, 호출한 쪽의 함수로 이동됩니다.
}

// takes_and_gives_back 함수는 String 을 하나 받아서 다른 하나를 반환합니다.
fn takes_and_gives_back(a_string: String) -> String { // a_string 이 스코프 안으로 들어왔습니다.

    a_string  // a_string 은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}