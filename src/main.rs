use std::{collections::{BTreeSet}};

fn main() 
{
    let arg = match std::env::args().nth(1)
    {
        Some(s) => s,
        _ => {
            println!("Err: Couldn't parse input argument");
            std::process::exit(2);
        }
    };

    let fields:Vec<String> = arg.split(' ').map(|s| String::from(s)).collect();

    println!("{}", print_output("minute",       get_range(&fields[0],0,59)));
    println!("{}", print_output("hour",         get_range(&fields[1],0,23)));
    println!("{}", print_output("day of month", get_range(&fields[2],1,30)));
    println!("{}", print_output("month",        get_range(&fields[3],1,12)));
    println!("{}", print_output("day of week",  get_range(&fields[4],1,7)));
    println!("{}", print_output_string("command",&fields[5]));
}

/// Turns the BTreeSet range to a string with a specified row name at the front.
/// The start of the row will always be a set 14 characters longs regardless of row name
/// 
/// # Arguments
/// * row_name - the name to be in the start of the row
/// * inp - the btreeset range that is to be converted to a string
fn print_output(row_name:&str,inp: BTreeSet<u8>) -> String
{
    let mut s = row_name.to_string();
    for _ in row_name.len()..14
    {
        s.push(' ');
    }
    for i in inp
    {
        s.push_str(&i.to_string());
        s.push(' ');
    }
    s
}
fn print_output_string(row_name:&str,input:&str) -> String
{
    let mut s = row_name.to_string();
    for _ in row_name.len()..14
    {
        s.push(' ');
    }
    s.push_str(&input);
    s
}

/// Gets the range specified by the cron field, as a BTreeSet to keep it ordered and non-repetitive
/// 
/// # Arguments
/// * field - the cron field to be parsed and returned
/// * range_from - lower range for the cron field (eg. 0 for minute)
/// * range_to - upper range for the cron field (eg. 12 for month)
fn get_range(field: &String, range_from:u8, range_to:u8) -> BTreeSet<u8>
{
    let inputs:Vec<&str> = field
        .split(',')
        .collect();

    let mut out = BTreeSet::new();

    for inp in inputs     // we split by ',' to get the ranges 
    {
        let (try_input_range_from,try_input_range_to,freq) = parse_inp(inp);
        
        let from = match try_input_range_from
        {
            Some(i) => i,
            None => range_from
        };
        let to = match try_input_range_to
        {
            Some(i) => i,
            None => range_to
        };
        if from < range_from || to > range_to
        {
            panic!("Err: Range {}-{} is outside the bounds {}-{}",from,to,range_from,range_to);
        }
        for i in from..=to
        {
            if (i - from) % freq == 0
            {
                out.insert(i);
            }
        }
    }
    out
}

/// Max ranges '*' will be None.
/// If no frequency, it will be 1.
/// 
/// # Arguments
/// * inp - the input from the comma separated list to parse
/// can contain { 0123456789-*/ }
fn parse_inp(inp:&str) -> (Option<u8>,Option<u8>,u8)
{
    // every n minutes on a range
    // we use string instead of &str
    // bcause we need .contains
    let every_n:Vec<String> = inp
        .split('/')
        .map(|s| String::from(s))   
        .collect();

    let mut modu = 1;                                                                    
    if every_n.len() > 1
    {
        modu = try_get_num(every_n[1].as_str());
    }
    let (lower, higher);
    if every_n[0].contains('-')         // to-from range
    {
        let tofrom:Vec<&str> = every_n[0].split('-').collect();
        lower  = Some(try_get_num(tofrom[0]));
        higher = Some(try_get_num(tofrom[1]));
        
    }
    else if every_n[0].contains('*')    // every number in the legal range
    {
        lower = None;
        higher = None;
    }
    else                                // single num range
    {
        let i = try_get_num(inp);
        lower = Some(i);
        higher = Some(i);
    }
    (lower,higher,modu)
}

/// Tries to parse an 8 bit integer from a string
/// 
/// # Arguments
/// * inp - the input that is to be parsed
fn try_get_num(inp:&str) -> u8
{
    let try_num = inp.parse::<u8>();
    if try_num.is_err()
    {
        panic!("Err: Couldn't parse {} to number", &inp);
    }
    try_num.unwrap()
}

#[cfg(test)]
mod tests 
{
    use super::*;
    
    #[test]
    #[should_panic]
    fn number_parse()
    {
        let s_1 = "1";
        let s_char_fail = "a";
        let s_null_fail = "";
        assert_eq!(try_get_num(s_1),1);
        try_get_num(s_char_fail);
        try_get_num(s_null_fail);
    }

    #[test]
    fn input_parse()
    {
        let inp_all = "*";
        let inp_all_range_1_to_5 = "1-5";
        let inp_all_freq2 = "*/2";
        let inp_all_num_3 = "3";
        assert_eq!(parse_inp(inp_all),(None,None,1));
        assert_eq!(parse_inp(inp_all_range_1_to_5),(Some(1),Some(5),1));
        assert_eq!(parse_inp(inp_all_freq2),(None,None,2));
        assert_eq!(parse_inp(inp_all_num_3),(Some(3),Some(3),1));
    }

    #[test]
    #[should_panic]
    fn input_parse_fail_notnum()
    {
        parse_inp("aa");
    }
    #[test]
    #[should_panic]
    fn input_parse_fail_notnum_freq()
    {
        parse_inp("4-5/a");
    }
    #[test]
    #[should_panic]
    fn input_parse_fail_notrange_freq()
    {
        parse_inp("3/3");
    }
    #[test]
    #[should_panic]
    fn input_parse_fail_notnum_range()
    {
        parse_inp("1-a");
    }
    #[test]
    fn range_check()
    {
        let res: BTreeSet<_> = [1, 2, 3].iter().cloned().collect();
        assert_eq!(get_range(&"1,2,3".to_string(),1,5),res);
        assert_eq!(get_range(&"1-3".to_string(),1,5),res);
        assert_eq!(get_range(&"*".to_string(),1,3),res);

        let res: BTreeSet<_> = [0,2,4,6,8].iter().cloned().collect();
        assert_eq!(get_range(&"*/2".to_string(),0,9),res);

        let res: BTreeSet<_> = [1,3,5,7,9].iter().cloned().collect();
        assert_eq!(get_range(&"*/2".to_string(),1,10),res);
        
        let res: BTreeSet<_> = [1,2,3,8,9,10].iter().cloned().collect();
        assert_eq!(get_range(&"1-3,8-10".to_string(),1,10),res);
    }
    #[test]
    #[should_panic]
    fn range_check_outofbounds_lower()
    {
        get_range(&"1-10".to_string(),1,9);
    }
    #[test]
    #[should_panic]
    fn range_check_outofbounds_upper()
    {
        get_range(&"1-10".to_string(),2,10);
    }
}