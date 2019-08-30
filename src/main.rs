// extern crate bnf;
// use bnf::Grammar;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "yul.pest"]
struct YulParser;

fn main() {
    let yul = "{
        function f() {

        }
        let _1 := mload(0)
        let f_a := mload(1)
        let f_r
        {
            f_a := mload(f_a)
            f_r := add(f_a, calldatasize())
        }
        let z := mload(2)

        if mload(2) {
            f_a := mload(f_a)
        }

        let a := 1
        switch calldataload(0)
        case 0 {
            mstore(0, 1)
            a := 8
        }
        default {
            a := 3
            a := 4
        }
        a := 5
    }";
    let sol = "contract C { 
        u256 a;
    }";
    let result = YulParser::parse(Rule::Block, yul).unwrap_or_else(|e| panic!("{}", e));
    println!("{:?}", res);
}
