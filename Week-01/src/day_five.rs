// Not ideal - It's dumb. It works, but it's not really scallable
//TODO: Come back and write something better
fn pair_of_socks(socks: String) -> i32
{
    if socks.is_empty()
    {
        return 0;
    }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    //add more here

    for sock in socks.chars()
    {
        match sock
        {
            'A' => a += 1,
            'B' => b += 1,
            'C' => c += 1,
            //add more here
            _ => continue,
        }
    }

    if a % 2 != 0
    {
        a -= 1;
    }

    if b % 2 != 0
    {
        b -= 1;
    }

    if c % 2 != 0
    {
        c -= 1;
    }

    return (a + b + c) / 2;
}

#[test]
fn test() 
{
    assert_eq!(pair_of_socks(String::from("AA")), 1);
    assert_eq!(pair_of_socks(String::from("ABABC")), 2);
    assert_eq!(pair_of_socks(String::from("CABBACCC")), 4);
    assert_eq!(pair_of_socks(String::new()), 0);
}