// 데이터 타입
pub fn main3_2() {
    // 러스트는 타입이 고정된 언어
    // String 의 타입을 pars() 함수를 사용해서 숫자로 변환가능
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{guess}\n");

    // 스칼라(scalar) 타입 : Integer, Float, Boolean, Character
    // 스칼라 타입은 하나의 값으로 표현되는 타입을 의미

    // 정수형(Integer) : Signed, Unsigned 는 부호(+,-)의 유무를 의미함
    // 부호가 필요하면 i, 오직 양수만을 갖는 데이터라면 u 타입을 사용!

    //8-bit	    i8	    u8
    //16-bit	i16	    u16
    //32-bit	i32	    u32
    //64-bit	i64	    u64
    //arch	    isize	usize

    // 여기서 숫자는 비트를 의미 i8 이면 8 bit = 1 byte

    // 범위는 부호일 때 -(2의 n-1승) 부터 2의 n-1승 - 1
    // 미부호일 때 0에서 2의 n승 - 1
    // i8 = -128 부터 127 사이의 값을 저장 및 표현가능!
    // u8 = 0에서 255까지 저장 및 표현가능!

    // arch 의 경우는 코드를 동작시키는 컴퓨터 환경에 따라 결정 : 64-bit arch 이면 64bit

    // 타입 접미사와 구분을 위한 _ 사용가능
    let n = 20u8; // u8 타입 접미사를 사용해서 지정가능!
    println!("{n}");
    let n = 20; // 타입을 지정하지 않으면 default i32
    println!("{n}");
    let n: u16 = 20; // 이렇게 타입을 지저할 수도 있음
    println!("{n}");
    let n = 100_000; // _ 로 숫자 표현 가능
    println!("{n}");

    // 리터럴 접두사
    let n = 0x20; // Hex
    println!("{n}"); // 32
    let n = 0o20; // Octal
    println!("{n}"); // 16
    let n = 0b111_000; // Binary
    println!("{n}"); // 56
    let n = b'A'; // Byte(u8 only)
    println!("{n}"); // 65


    // 부동 소수점(Float) : f32, f64
    // default 타입은 f64
    let _x = 3.; // f64
    let _y: f32 = 3.14; // f32


    // 연산
    // 사칙 연산과 나머지 연산 등 여러가지 연산 가능!
    // 하지만 주의할 점! 타입간의 연산 불가능!
    // e.g. i32 + i64 , i8 + u8 불가능!

    // let x: i32 = 10;
    // let y: i64 = 20;
    // let xy = x + y; //  mismatched types
    // println!("{xy}")
    // y : expected `i32`, found `i64`

    // let x: i32 = 10;
    // let y: u32 = 20;
    // let xy = x + y; //  mismatched types
    // println!("{xy}");
    // y : expected `i32`, found `u32`

    // casting : as 을 사용하면 연산가능
    let x: i32 = 10;
    let y: u32 = 20;
    let xy = x + y as i32;
    println!("{xy}");
    let x: i32 = 20;
    let y: u32 = 30;
    let xy = x as u32 + y;
    println!("{xy}");


    // Boolean(bool) : true, false
    let t = true;
    println!("{t}");
    let f: bool = false;
    println!("{f}");

    // 문자(Character) : char
    // 여러나라 문자, 이모티콘, 이모지, 공백 모두 사용가능
    let ch = '😻';
    println!("{ch}");
    let ch: char = ' ';
    println!("{ch}");
    let ch: char = 'A';
    println!("{ch}");


    // 컴파운드(compound) 타입 : Tuple, Array

    // Tuple
    let tup: (i32, f64, u8) = (500, 3.14, 8);
    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z: {z} \n");

    // index 를 통해서 접근 가능!
    let first_num = tup.0;
    println!("{first_num}");
    let second_num = tup.1;
    println!("{second_num}");
    let third_num = tup.2;
    println!("{third_num}");


    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let num = arr[4];
    println!("{num}");
    // i32 타입 5개인 배열 선언 및 초기화

    let arr = [9; 5]; // let arr = [9, 9, 9, 9, 9]; 와 동일
    let num = arr[4];
    println!("{num}");
    // 9가 5개인 배열 선언 및 초기화


    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let first_month = months[0];
    println!("{first_month}");

    let total_len = months.len();

    let last_month = months[total_len - 1];
    println!("{last_month}");
}