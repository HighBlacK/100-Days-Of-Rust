fn age_to_days(age: u32) -> u32
{
    return age * 365;
}

#[test]
fn test() 
{
    assert_eq!(age_to_days(65), 23725);
    assert_eq!(age_to_days(0), 0);
    assert_eq!(age_to_days(20), 7300);
}