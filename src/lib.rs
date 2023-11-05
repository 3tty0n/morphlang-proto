#[macro_export]
macro_rules! create_router {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Defined router function {:?}()",
                     stringify!($func_name));
        }
    }
}
