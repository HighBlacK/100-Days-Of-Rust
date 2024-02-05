fn progress_days(runs: Vec<u32>) -> u32
{
    let mut result = 0;
    let mut buffer = 0; //to store previous run

    for run in runs
    {
        if buffer != 0 && buffer < run  
        {
            result += 1
        }

        buffer = run;
    }

    return result
}

#[test]
fn test() 
{
    assert_eq!(progress_days(vec![3,4,1,2]), 2);
    assert_eq!(progress_days(vec![10,11,12,9,10]), 3);
    assert_eq!(progress_days(vec![6,5,4,3,2,9]), 1);
    assert_eq!(progress_days(vec![9,9]), 0);
}