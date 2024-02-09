use std::collections::BTreeMap;
use lazy_static::lazy_static;

lazy_static! 
{
    static ref MAPPINGS: BTreeMap<i32, Vec<char>> = 
    {
        let mut map = BTreeMap::new();
        map.insert(2, vec!['a','b','c']);
        map.insert(3, vec!['d','e','f']);
        map.insert(4, vec!['g','h','i']);
        map.insert(5, vec!['j','k','l']);
        map.insert(6, vec!['m','n','o']);
        map.insert(7, vec!['p','q','r','s']);
        map.insert(8, vec!['t','u','v']);
        map.insert(9, vec!['w','x','y', 'z']);
        return map
    };

    static ref EMPTY: Vec<char> = Vec::new();
}

pub fn letter_combinations(string: String) -> Vec<String>
{
    if string.is_empty(){ return Vec::new(); }

    //step 1 parse string into digits
    let numbers = string_to_digit(string);

    //step 2 get mapings for each digits
    let chars = get_chars(numbers);

    //step 3 combine the mappings
    let combined = combine_mappings(chars);

    return combined
}

fn get_chars(numbers: Vec<i32>) -> Vec<&'static Vec<char>>
{
    let mut chars = Vec::new();
    for num in numbers
    {
        chars.push(MAPPINGS.get(&num).unwrap_or(&EMPTY));
    }

    return chars
}

fn string_to_digit(string: String) -> Vec<i32>
{
    let digits: Vec<char> = string.chars().collect();

    let mut numbers: Vec<i32> = Vec::new();
    for digit in digits
    {
        if digit.is_digit(10)
        {
            let int = digit.to_digit(10).unwrap();
            if int >= 2
            {
                numbers.push(int as i32);
            }
        }
    }

    return numbers
}

fn combine_mappings(chars: Vec<&Vec<char>>) -> Vec<String> 
{
    if chars.len() == 1
    {
        let mut result = Vec::new();
        for c in chars[0]
        {
            result.push(c.to_string());
        }

        return result
    }

    let mut combined = Vec::new();
    let vec = chars[0];

    for c in vec 
    {
        for i in chars[1] 
        {
            combined.push(format!("{}{}", c, i));
        }
    }

    return combined
}


#[test]
fn test()
{
    assert_eq!(letter_combinations(String::from("23")), vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
    assert_eq!(letter_combinations(String::new()), Vec::<String>::new());
    assert_eq!(letter_combinations(String::from("2")),["a","b","c"]);
}