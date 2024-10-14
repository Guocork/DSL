use nom::{
    IResult,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{alphanumeric1, multispace0, char, digit1, one_of},
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, preceded, separated_pair},
};
use std::collections::HashMap;

#[derive(Debug)]
struct Config {
    sections: HashMap<String, HashMap<String, String>>,
}

fn parse_config(input: &str) -> IResult<&str, Config> {
    let (input, sections) = many0(parse_section)(input)?;
    let mut config = HashMap::new();
    for (section, pairs) in sections {
        config.insert(section, pairs);
    }
    Ok((input, Config { sections: config }))
}

fn parse_section(input: &str) -> IResult<&str, (String, HashMap<String, String>)> {
    let (input, section) = delimited(char('['), take_until("]"), char(']'))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, pairs) = many0(parse_key_value)(input)?;
    let mut map = HashMap::new();
    for (k, v) in pairs {
        map.insert(k, v);
    }
    Ok((input, (section.trim().to_string(), map)))
}

fn parse_key_value(input: &str) -> IResult<&str, (String, String)> {
    let (input, (key, value)) = separated_pair(
        delimited(multispace0, alphanumeric1, multispace0),
        tag("="),
        delimited(multispace0, parse_value, multispace0)
    )(input)?;
    Ok((input, (key.to_string(), value)))
}

fn parse_value(input: &str) -> IResult<&str, String> {
    // 支持字符串和数字
    parse_string.or_else(|_| parse_number)(input)
}

fn parse_string(input: &str) -> IResult<&str, String> {
    let (input, content) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    Ok((input, content.to_string()))
}

fn parse_number(input: &str) -> IResult<&str, String> {
    let (input, num) = digit1(input)?;
    Ok((input, num.to_string()))
}

fn main() {
    let config_str = r#"
    [server]
    host = "localhost"
    port = 8080

    [database]
    user = "admin"
    password = "secret"
    "#;

    match parse_config(config_str) {
        Ok((_, config)) => {
            println!("{:#?}", config);
        },
        Err(e) => {
            eprintln!("解析失败: {:?}", e);
        }
    }
}
