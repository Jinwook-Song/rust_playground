use std::io;

fn main() {
    input_and_print();
}

/**
    1. 크기를 알 수 없는 String이 input에 저장된다. (즉, 힙에 저장된다. 크기의 정보를 갖는 메타데이터도 함께)
    2. Stringdml 소유권이 input2로 넘어간다.
*/
fn input_and_print() {
    let input = String::new();

    let mut input2 = input;

    println!("Enter some text:");

    match io::stdin().read_line(&mut input2) {
        Ok(_) => {
            println!("You entered: {}", input2.trim());
        }
        Err(error) => {
            println!("Error reading input: {}", error);
        }
    }
}

fn input_and_print2() {
    let mut input = String::new();
    ref_text(&mut input);
    // ❌ 함수로 소유가 넘어간 이후에 input을 사용하려함
    let mut input2 = input;
}

fn take_text(_: String) {}
fn ref_text(s: &mut String) {
    s.push_str("append");
}
