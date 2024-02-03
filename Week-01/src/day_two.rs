fn finding_nemo(mut string: String) -> String
{
    let position = string.find("Nemo");
    if position.is_none() {return String::from("I can't find Nemo :(")}
    let start = position.unwrap() + 4;
    
    string.replace_range(start.. , "");

    let mut count: usize = 0;
    for c in string.chars()
    {
        match c 
        {
            ' ' | '\t' | '\n' | '\r'  => count += 1,
            _ => continue,
        }
    }

    return format!("I found Nemo at {}!", count + 1)

}

#[test]
fn test()
{
    assert_eq!(finding_nemo(String::from("I am finding Nemo !")), String::from("I found Nemo at 4!"));
    assert_eq!(finding_nemo(String::from("Nemo is me")), String::from("I found Nemo at 1!"));
    assert_eq!(finding_nemo(String::from("I Nemo am")), String::from("I found Nemo at 2!"));
    assert_eq!(finding_nemo(String::from("Nowhwere to be seen")), String::from("I can't find Nemo :("));
}

fn finding_nemo_2(string: String) -> String
{
    let find = 
    string  .split(' ')
            .position(|word| word == "Nemo");
    
    if find.is_none() {return String::from("I can't find Nemo :(")}

    return format!("I found Nemo at {}!", find.unwrap() + 1)
    
}

#[test]
fn test_2()
{
    assert_eq!(finding_nemo_2(String::from("I am finding Nemo !")), String::from("I found Nemo at 4!"));
    assert_eq!(finding_nemo_2(String::from("Nemo is me")), String::from("I found Nemo at 1!"));
    assert_eq!(finding_nemo_2(String::from("I Nemo am")), String::from("I found Nemo at 2!"));
    assert_eq!(finding_nemo_2(String::from("Nowhwere to be seen")), String::from("I can't find Nemo :("));
}