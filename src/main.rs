use std::env;

fn main() {
    let mut args = env::args().skip(1);
    if args.next().as_deref() == Some("median") {
        let values: Vec<i32> = args.map(|a| a.parse().expect("invalid number")).collect();
        let result = peppec_test::median(&values).expect("empty list");
        println!("{result}");
    }
}
