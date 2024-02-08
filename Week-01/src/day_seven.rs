fn merge_sorted_array_nc(array1: Vec<i32>, array2: Vec<i32>) -> Vec<i32>
{
    // merge the arrays
    let mut merged = array1;

    for num in array2
    {
        merged.push(num);
    }
     
    //Implement bubble sort
    let length = merged.len();

    let mut sorted;

    loop 
    {   
        sorted = false;

        for i in 0..length -1 
        {
            if merged[i] > merged[i + 1]
            {
                merged.swap(i, i + 1);
                sorted = true
            }
        }

        // if no sorting happenened during the loop it breaks
        if !sorted { break }
    }
    
    //return the sorted array
    return merged
}

#[test]
fn test()
{
    let mut array1 = vec![1,2,3];
    let array2 = vec![2,5,6];
    assert_eq!(merge_sorted_array_nc(array1.clone(), array2.clone()), vec![1,2,2,3,5,6]);

    let size1 = array1.len();
    let size2 = array2.len();

    array1 = Vec::with_capacity(size1 + size2);
    
    let mut number = 1;
    for i in 0.. array2.len()
    {
        array1.insert(i, number);
        number += 1
    }
        
    assert_eq!(merge_sorted_array_with_contraints(array1, size1, array2, size2), vec![1,2,2,3,5,6]);
}

fn merge_sorted_array_with_contraints(array1: Vec<i32>, size1: usize, array2: Vec<i32>, size2: usize) -> Vec<i32>
{
    if array1.capacity() != (size1 + size2) || array2.capacity() != size2
    {
        panic!("array1 should have m + n length and array2 should have n length")
    }
    
    /*for num in &array1
    {
        //why????
        if &(-10^9) <= num
        {
            panic!("-10^9 <= array1[i]")
        }
    }*/
   
    let mut merged = array1;

    for num in array2
    {
        /*//why????
        if num <= 10^9
        {
            panic!("array2[i] <= 10^9")
        }*/

        merged.push(num);
    }
     
    let length = merged.len();

    let mut sorted;

    loop 
    {   
        sorted = false;

        for i in 0..length -1 
        {
            if merged[i] > merged[i + 1]
            {
                merged.swap(i, i + 1);
                sorted = true
            }
        }

        if !sorted { break }
    }

    return merged
}
