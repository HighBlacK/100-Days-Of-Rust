pub fn unique_binary_search_trees(unique_bst: i32) -> i32
{
    let mut number = Vec::with_capacity((unique_bst + 1) as usize);
    number.insert(0, 1);
    number.insert(1, 1);

    for count in 2.. (unique_bst + 1)
    {
        let mut total = 0;
        
        for root in 1.. (count + 1)
        {
            let left = root - 1;
            let right = count - root;
            total += number.get(left as usize).unwrap() * number.get(right as usize).unwrap();
        }
        number.insert(count as usize, total);
    }

    return *number.get(unique_bst as usize).unwrap();

}

#[test]
fn test()
{
    assert_eq!(unique_binary_search_trees(3), 5);
    println!("{}", unique_binary_search_trees(19)); // i128: max 68 i64: max 36 i32: max 19
}