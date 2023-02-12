use crate::lexer::lexer;

pub fn parse_file(filename: &str){
    let mut tokens = match lexer::get_tokens(filename){
        None => panic!("no tokens"),
        Some(tokens) => tokens
    };
    

    loop {
        if let Some(token) = tokens.get_next(){
            println!("{}", token);
        }
        else {
            break;
        }
    }
}
