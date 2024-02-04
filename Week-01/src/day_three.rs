fn bbq_skewers(text: String) -> Vec<u32>
{
    let mut array:Vec<u32> = vec![0,0];

    let skewers: Vec<&str> = text.split('\n').collect();

    for s in skewers
    {
        if s.contains('x')
        {
            array[1] += 1;
        }
        else 
        {
            array[0] += 1;
        }
    }

    return array
}

#[test]
fn test() 
{
    let text1 = String::from(
    r#"{"--oooo-ooo--",
    "--xx--x--xx--",
    "--o---o--oo--",
    "--xx--x--ox--",
    "--xx--x--ox--"}"#);

    let text2 = String::from(
        r#"{"--oooo-ooo--",
        "--xxxxxxxx--",
        "--o---",
        "-o-----o---x--",
        "--o---o-----"}"#);

    assert_eq!(bbq_skewers(text1), vec![2,3]);
    assert_eq!(bbq_skewers(text2), vec![3,2]);
}