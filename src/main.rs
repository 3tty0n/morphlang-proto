mod lib;

use lib::parse_expr;
use nom::IResult;

fn main() {

    match parse_expr("1 + 2") {
        Ok((s, e)) => println!("{:?}, rest of str: {}", e, s),
        Err(e) => println!("err {}", e),
    }
}
