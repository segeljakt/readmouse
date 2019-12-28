use readmouse::Mouse;

fn main() {
    loop {
        println!("{:?}", Mouse::location());
    }
}
