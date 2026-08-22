use std::fs;
use std::io;

pub mod opts {
    pub static CRLF_ENFORCED: bool = false; // True => Do not accept LF at end instead of CRLF
}

pub type Table = Vec<Vec<String>>;

// Open a csv file and return the table
pub fn csv_open(filepath: &str) -> io::Result<Table> {
    // init and sanitize
    let contents = fs::read_to_string(filepath)?;
    if !contents.is_ascii() {
        return Err(io::ErrorKind::InvalidData.into());
    }

    // tokenize and parse
    let tokens = tokenize(&contents)?;
    return if let Some(table) = parse_file(&tokens) { 
        Ok(table)
    } else { 
        Err(io::ErrorKind::InvalidData.into())
    }
}

/* Tokenizing and Parsing functions below.
 * reference - https://datatracker.ietf.org/doc/html/rfc4180 */

// ommiting redundant tokens
#[derive(Debug)]
enum Token {
    // patterns
    // File(Table):         record *(CRLF record) [CRLF]
    // Record(Vec<String>): field *(COMMA field)
    // Field(String):       (escaped / non-escaped)
    // NonEscaped(String):  *TEXTDATA
    // Escaped(String):     DQUOTE *(TEXTDATA / COMMA / CR / LF / 2DQUOTE) DQUOTE
    // CRLF:                CR LF

    // text characters
    TEXTDATA(char), // %x20-21 / %x23-2B / %x2D-7E
    COMMA,          // ,
    DQUOTE,         // "
    CR,             // \r
    LF,             // \n
}

// Tokenize a csv buffer into vector
fn tokenize(buffer: &String) -> io::Result<Vec<Token>> {
    let mut ret = vec![];
    for ch in buffer.chars() {
        let tok = match ch {
            '\u{0020}'..='\u{0021}' | '\u{0023}'..='\u{002B}' | '\u{002D}'..='\u{007E}' => {
                Token::TEXTDATA(ch)
            }
            ',' => Token::COMMA,
            '\"' => Token::DQUOTE,
            '\u{000D}' => Token::CR,
            '\u{000A}' => Token::LF,
            _ => return Err(io::ErrorKind::InvalidData.into()),
        };
        ret.push(tok);
    }
    return Ok(ret);
}

// File -> record *(CRLF record) [CRLF]
fn parse_file(token_stream: &Vec<Token>) -> Option<Table> {
    // init
    let mut index = 0;
    let mut ret = vec![];

    // record
    ret.push(parse_record(token_stream, &mut index)?);

    // *(CRLF record)
    while parse_crlf(token_stream, &mut index) != None {
        ret.push(parse_record(token_stream, &mut index)?);
    }

    // CRLF
    if index != token_stream.len() {
        parse_crlf(token_stream, &mut index)?;
    }
    return Some(ret);
}

// Record -> field *(COMMA field)
fn parse_record(token_stream: &Vec<Token>, index: &mut usize) -> Option<Vec<String>> {
    // init
    let mut i = *index; 
    let mut ret = vec![];

    // field 
    ret.push(parse_field(token_stream, &mut i)?);
    
    // *(COMMA field)
    while let Some(Token::COMMA) = token_stream.get(i) {
        i += 1;
        ret.push(parse_field(token_stream, &mut i)?);
    }

    // success
    *index = i;
    return Some(ret);
}

// Field -> (escaped / non-escaped)
fn parse_field(token_stream: &Vec<Token>, index: &mut usize) -> Option<String> {
    // init
    let mut i = *index;
    
    // (escaped / non-escaped)
    let ret = if let Some(escaped) = parse_escaped(token_stream, &mut i) {
        escaped
    } else if let Some(non_escaped) = parse_non_escaped(token_stream, &mut i) {
        non_escaped
    } else {
        return None;
    };

    // success
    *index = i;
    return Some(ret);
}

// Escaped -> DQUOTE *(TEXTDATA / COMMA / CR / LF / 2DQUOTE) DQUOTE
fn parse_escaped(token_stream: &Vec<Token>, index: &mut usize) -> Option<String> {
    // init
    let mut i = *index;
    let mut ret = String::new();

    // DQUOTE
    if !matches!( token_stream.get(i), Some(Token::DQUOTE)) { return None; }
    i += 1;

    // *(TEXTDATA / COMMA / CR / LF / 2DQUOTE) 
    loop {
        match token_stream.get(i)? {
            Token::TEXTDATA(ch) => ret.push(*ch), // TEXTDATA
            Token::COMMA => ret.push(','),          // COMMA
            Token::CR => ret.push('\r'),            // CR
            Token::LF => ret.push('\n'),            // LF
            Token::DQUOTE => {                      // 2DQUOTE (ensure not 1)
                if let Some(Token::DQUOTE) = token_stream.get(i+1) {
                    i += 1;
                    ret.push('\"')
                } else {
                    i -= 1;
                    break;
                }
            }
        }
        i += 1;
    }

    // DQUOTE
    if !matches!(token_stream.get(i), Some(Token::DQUOTE)) { return None; }
    i += 1;

    // Success
    *index = i;
    return Some(ret);
}

// NonEscaped -> *TEXTDATA
fn parse_non_escaped(token_stream: &Vec<Token>, index: &mut usize) -> Option<String> {
    // init
    let mut ret = String::new();

    // *TEXTDATA
    while let Some(Token::TEXTDATA(ch)) = token_stream.get(*index) {
        ret.push(*ch);
        *index += 1;
    }

    // success
    return Some(ret);
}

// CRLF -> CR LF
fn parse_crlf(token_stream: &Vec<Token>, index: &mut usize) -> Option<()> {
    // init
    let mut i = *index;
    
    // CR
    if opts::CRLF_ENFORCED {
        if !matches!(token_stream.get(i), Some(Token::CR)) { return None; }
        i += 1;
    }

    // LF
    if !matches!(token_stream.get(i), Some(Token::LF)) { return None; }
    i += 1;
    
    // success
    *index = i;
    return Some(());
}
