extern crate morph;
extern crate morph_macro;

use quote::quote;
use morph_macro::morph;

#[test]
fn test_morph_macro() {
    morph! {
        function f(x: i32): i32 = let y = x + 1 in y + 2;;
        function g(z: i32): i32 = z + f(1);;
    };
    assert_eq!(4, f(1));
    assert_eq!(5, g(1))
}
