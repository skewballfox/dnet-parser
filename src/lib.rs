use nom::{error::ParseError, IResult};

pub enum DnetToken {
    Comment,
}

//  1.     <file>  ->  [ <object> ; ]*

//  2.   <object>  ->  <objtype> <idname> { <stmt>* }

//  3.     <stmt>  ->  <field> = <value> ;

//  4.     <stmt>  ->  <object> ;

//  5.    <value>  ->  <idname>  |  <number>  |  <string> | <object> | <list>

//  6.     <list>  ->  ( [<value>] [, <value>]* )

//  7.    <field>  ->  <idname>

//  8.  <objtype>  ->  <idname>

//  9.   <idname>  ->  <alpha> <idchar>*

// 10.   <idchar>  ->  <alpha>  |  <digit>  |  _

// 11.   <string>  ->  " <genchar>* "

// 12.  <genchar>  ->  <char>  |  \ <char>  |  \\  |  \"  |  \ <hex> <hex>

// 13.     <char>  ->  <alpha>  |  <digit>  |  <punc>

// 15.      <hex>  ->  <digit> | A | B | C | D | E | F

// 16.    <alpha>  ->  a | b | c | ... | z | A | B | C | ... | Z

// 17.    <digit>  ->  0 | 1 | 2 | ... | 9

// 18.     <punc> -> !|@|#|$|%|^|&|*|(|)|-|=|+|[|]|{|}|;|:|'|||,|.|<|>|?|/|`|~

// 14.   <number>  ->  [+ | -] [0x] <digit>* [.] <digit>+ [e <digit>+]
fn any_number<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    todo!()
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
