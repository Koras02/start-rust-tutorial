// ================================Chapter1 항목입니다.============================================== // 
// // fn main() {} 이 부분은 프로그램의 진입점이다.
// Chapter 1. HelloWorld
// fn main() {
//     // 여기서 println!() 은 매크로다,(매크로로는 !를 사용해 호출)

//     // println! 은 콜솔에 출력하느 함수 (줄바꿈) (print!는 줄바꿈 X)
//     // println!("와 샌즈!");  // ; 러스트에서 필요할 수 도 아닐 수도 있다. 기본적으로 사용하자
//     //    let x = 10;
//     //    println!("{}", x)

// }

// ================================Chapter1 항목입니다.============================================== // 


// ================================Chapter2 항목입니다.============================================== // 
// // Chapter 2. 변수 
//  // 변수 x 에 10을 할당하고, x 값을 출력하는 코드 
// //  let x = 10;
// //  println!("{}", x)

// //  println!("===============================================")
//  // 다음 예제는 x의 값을 20으로 바꾸는 평볌한 코드이다.
//  fn main() {
//      // 정상적으로 출력될 것 같지만, 오류가 나온다.
//      // 그 이유는 변수 x는 불변이기 떄문에 다른 값으로 바꿀 수 없다.
//      // 그럴 떄는 mut 키워드를 사용해준다.
//     // let x = 10; // X
//     let mut x = 10; // O
//     println!("{}", x);
//     println!("===============================================");
//     x = 20;
//     println!("{}", x);
//     println!("===============================================");
 

//     // 또한, 러스트에서는 변수를 덮을 수 있다.
//     let x = 10;
//     println!("{}",x);
//     println!("===============================================");
//     let x = 20;
//     println!("{}", x);

// }

/* 추가) 값의 타입
   
   타입을 정하고 싶다면, let 변수 이름: 타입 = 값; 으로 정하면 된다.

*/ 

// ================================Chapter2 항목입니다.============================================== // 

// ================================Chapter3 항목입니다.============================================== // 

/* Chapter3. 함수  
  
   러스트에서 함수는 fn으로 선언이 가능하다.

   fn 함수이름(args) -> 반환값_타입 {}

   이런식으로 함수를 선언한다.
*/

// 이 프로그램은 hello() 함수를 호출했으니, "hello() 함수 호출됨"이 출력될 것이다.
// 매개변수가 빠질수없는데, 매개변수는 함수에 값을 전달하는 역할을 한다.
// fn main() {
//     hello();
// }

// fn hello() {
//     println!("hello() 함수 호출됨.");
// }

// println!("======================================================");

// 함수 반환 
// 다른 언어에서 흔이 return 키워드를 사용하느데 러스트에서는 조금 특별한 방법을 사용한다.
// fn main() {
//     println!("{}", hello());
// }
// // 러스트에서는 return 대신에 값에 세미콜론 ; 이 없다면 그 값이 반환값이 되는 것.
// // 또한 반환값의 타입을 정할떄는, 함수 임를 뒤에 -> [타입]을 추가해 반환값을 정한다.
// fn hello() -> i32 {
//     println!("hello() 함수는 10을 반환합니다.");
//     20
// }

// public 함수 
// 지금 당장에 필요하지 않지만 나중에 프로젝트 규모가 커질때, 함수를 다른 파일에 작성해야하는 상황이 생길텐데 
// 뒤에서 배울 것 이지만, 러스트에서 함수는 기본적으로 private이기 떄문에, public으로 정해줘야 한다.
// 간단히 fn 키워드 앞에 pub를 붙여주면 된다.

// pub fn a() {} // public 함수

// fn b () {} // private 함수

// ================================Chapter3 항목입니다.============================================== // 

// ================================Chapter4 항목입니다.============================================== //

/* Chapter4. 흐름제어
*/


// fn main() {
//     // if 키워드 
//     // if는 이미 많이 사용해보았을 것이다.
//     let x = 30;
//     if x == 10 {
//     println!("x가 10입니다!");
//     } else if x == 20  {
//         println!("x가 20입니다!");
//     } else {
//         println!("10 과 20만 판단 가능");
//     }

//     println!("=========================================");

//     // for 키워드 
//     // 다음으로는 for키워드이다. for문 또한 이미 많이 사용해보았을 것이다.
//     // 이예제는 0부터 10까지 증가하는 for문이다.
//     // 러스트에서 0..10부분은 파이썬의 range(0,10) 과 비슷
//     // 배열의 값을 차례대로 출력하는 for문도 존재한다.
//     // break, continue 키워드를 사용해 루프에서 빠저나오거나, 거너뛸 수 있다.
//     for i in 0..10 {
//         println!("인덱스: {}", i);
//     }

//     println!("=========================================");

//     // while문 
//     // 주로 단순 반복문으로 많이 사용된다.
//     // 주로 true를 넣어서 무한 루프에 사용되지만, while로 무한루프를 사용하지 않는다.
//     // 그 이유로는 loop 키워드 때문인데, break, continue 키워드를 사용해 루프에 빠저나오거나 건너뛸 수 있다.
//     let mut x = 0;
//     while x < 10 {
//         x += 1;
//         println!("{}", x);
//     }

//     println!("=========================================");

//     // loop 키워드 
//     // loop는 말그대로 무한 루프이다.
//     loop {
//         println!("멈춰!!!!!!~@!@!@"); // 절대 안멈춤 ㅋㅋㅋ
//         // break 또는 continue 를 쓰면 빠져나올 수 있따.
//         break;
//     }

//     // 루프 레이블 (loop label)
//     // 러스트에서 제공하는 신기한 기능(쓸모없는 기능//)이다.
//     println!("=========================================");

//         'a: loop {
//             println!("a 루프 시작");
    
//             'b: loop {
//                 println!("b 루프 시작");
//                 break 'a;
//             }
    
//             println!("멈춰!");
//         }
    
//         println!("종료되었다 쿠쿠루뽕삥뽕#@#!@#~~");
    

// }

// ================================Chapter4 항목입니다.============================================== // 


// ================================Chapter5 항목입니다. ============================================ //
  /*
    Chapter5. 소유권 

    소유권은 러스트의 독특한 기능이다. GC에 의존하지 않고도, 메모리 안정성을 보장하려는 리스트의 해법

    모든 프로그램은 런타임중에 메모리를 관리해야 한다. 어떤 언어는 가비지 콜렉터를 이용해 관리하기도 하고 
    프로그래머가 명시적으로 관리해야하는 언어도 있다.

    러스트에서는 다른 방법을 사용한다. 컴파일러가 컴파일 시점에 검사하는 다양한 규칙으로 이루어진 소유권 시스템에 의해 관리된다.

    그래서 소유권과 관련된 기능은 런타임 성능에 아무런 영향을 미치지 않는다. 

    소유권의 규칙 
    1. 러스트가 다루는 각각의 값은 소유자라고 부르는 변수를 가지고 있다.
    2. 특정 시점에서 값의 소유자는 단 하나뿐 
    3. 소유자가 범위를 벗어나면 그 값은 제거된다. 
   
   
    let x = "ㅁㄴㅇㄹ"; 이 변수의 타입은 str이다. str 또한 많이 사용되는 데이터 타입이지만, 그리 적합한 타입은 아니다.
    그 이유로는 , str은 불변이기 때문이다.

    String 타입은 힙 메모리에 저장된다.
    String 타입은 from 함수를 사용해 문자열을 생성한다.
    */ 
//     fn main() {
//         // let mut x = String::from("ㅁㄴㅇㄹ");
//         // s.push_str(", ㅁㄴㅇㄹ 멈춰!");

//         // x: ㅁㄴㅇㄹ, ㅁㄴㅇㄹ 멈춰!
//         // str과 String이 다른 이유 

//         /* 
//           메모리 할당 
//           str는 컴파일 시점에서 내용을 알고 있으므로, 텍스트를 최종 실행할 수 있는 형태로 직접 하드코딩해줄 수 있다.
          
//           그러므로 str은 빠르다. 하지만 str은 불변이라 

//           컴파일 시점에 내용을 미리 알 수 없거나 런타임 중에 내용이 변경되는 문자열들은 바이너리로 저장할 수 없다 => 비효율적
//           하지만 String 타입은 런타임중에 변경이 가능하므로, 컴파일 시점에 내용을 알 수 없을 경우 유용하다.
//           러스트에서는 drop 함수가 존재하는데, drop 함수는 string을 해제하는 함수이다.
//           러스트에서 } 를 만나면 drop이 호출된다.

//         */

//         // 1.변수와 데이터가 상호작용하는 방식: Move 
//         // 단순한 a의 값(10)을 b로 넘기는 매우 평범한 코드이다. 또한 정상적으로 작동한다.
//         // let a = 10;
//         // let b = a;

//         // println!("a: {}, b:{}", a, b);
        
//         // 다음 예제를 보면
//         // 이또한 정상작동될거 같지만 오류가 난다
//         // 이는 다른 언어에서 얕은 복사, 깊은 복사를 들어본적이 있다면 알것이다. 

//         // 실제 데이터를 복사하지 않고, 포인터 / 용량만 복사하는 동작이 얕은 복사와 비슷하다고 생각할것이다.
        
        
//         // let a = String::from("ㅁㄴㅇㄹ");
//         // let b = a;
        

//         // println!("a: {} b: {}", a,b);

//         /*
//           하지만, a를 미국으로 보내버리므로, 러스트에서는 이동(Move)라고 한다.
//           즉, a변수를 b로 이동했다. 라고 표현한다.
//           이럴때는 .clone() 메서드를 사용하면된다.
//         */
              
//         let a = String::from("ㅁㄴㅇㄹ");
//         let b = a.clone();
        
//         println!("a: {} b: {}", a,b);
// }
   
// ================================Chapter5 항목입니다. ============================================ //

// ================================Chapter6 항목입니다 ============================================ //
fn main() {
  /*
    Chapter6. 참조

    참조는 C#의 ref 기능과 비슷한 기능이다.
    러스트에서는 참조에 키워드는 ref 대신, &를 사용한다.
  */

  // 이 예제는 & 참조의 예제이다.
  // get_size 함수에 main 함수에 있는 x 변수를 넘겨서, 넘겨진 값의 길이를 알아오는 코드이다. 


  // let x = String::from("ㅁㄴㅇㄻㄴㅇㄹ");
  // println!("{}", get_size(&x));

  // 기변 참조 
  // 러스트는 기본적으로 불변인데, 그래서, 기변 참조는 mut 키워드를 사용해야 한다.

    let mut s = String::from("ㅁㄴㅇㄹ");

    test(&mut s);

    println!("{}", s);

  fn test(s: &mut String) {
     s.push_str(", world!");
  }

}

// fn get_size(s: &String) -> usize {
//   s.len()
// }