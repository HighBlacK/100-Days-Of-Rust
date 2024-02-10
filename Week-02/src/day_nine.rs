//my version O(n^2)
fn trapping_rain_water(mut height_map: Vec<u32>) -> u32
{
    let mut max_lines = *height_map.iter().max().unwrap();
    let mut water_units = 0;

    while max_lines != 0
    {
        let mut line = Vec::new();

        for block in height_map.clone()
        { 
            if block == 0
            {
                line.push(0);
            }
            else
            {
                line.push(1);
            }
        }

        let mut count = line.iter().position(|&x| x == 1).unwrap();
        let mut end = line.len() - (line.iter().rev().position(|&x| x == 1).unwrap()) - 1;
        while count < end
        {
            if line[count] == 0 
            {
                water_units += 1;
            }

            end = line.len() - (line.iter().rev().position(|&x| x == 1).unwrap());
            count += 1;
        }
                         
        height_map = height_map.iter().map(|x| if *x != 0 { *x - 1} else { *x }).collect();
        
        max_lines -= 1;
    }

    return water_units
}

#[test]
fn test()
{
    assert_eq!(trapping_rain_water(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    assert_eq!(trapping_rain_water(vec![4,2,0,3,2,5]), 9);
}

//copilot's version O(n)
fn trapping_rain_water_copilot(height: Vec<u32>) -> u32 {
    let (mut left, mut right) = (0, height.len() as i32 - 1);
    let (mut max_left, mut max_right) = (0, 0);
    let mut ans = 0;
    while left < right {
        if height[left as usize] < height[right as usize] {
            if height[left as usize] > max_left {
                max_left = height[left as usize];
            } else {
                ans += max_left - height[left as usize];
            }
            left += 1;
        } else {
            if height[right as usize] > max_right {
                max_right = height[right as usize];
            } else {
                ans += max_right - height[right as usize];
            }
            right -= 1;
        }
    }
    ans
}
//This version of the function uses two pointers, left and right, to traverse the height vector from both ends towards the center. It keeps track of the maximum height seen so far from both ends, max_left and max_right. 
//If the height at the current left pointer is less than the height at the right pointer, it checks if the height is greater than max_left. 
//If it is, it updates max_left. Otherwise, it adds the difference between max_left and the current height to ans, which keeps track of the total water trapped. 
//It then moves the left pointer one step to the right. It does a similar operation for the right pointer if the height at the right pointer is less than or equal to the height at the left pointer. 
//The function continues this process until the left and right pointers meet, at which point it returns ans.