#![allow(unused_imports, unused_variables, dead_code)]

pub mod ast;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);
