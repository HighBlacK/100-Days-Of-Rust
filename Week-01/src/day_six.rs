// TODO: Find a way more efficient way of doing this
fn next_prime(input: u32) -> u32
{
    let mut m_input = input;

    loop
    {    
        let mut is_prime = true;
        
        for num in 1.. u32::MAX // this is horrible for performance - i suck at math. That loop will run 4 billion times when m_input is a prime number
        {
            if num != 1 && num != m_input && m_input % num == 0
            {
                is_prime = false;
                break
            }
        }

        if is_prime {break}
        m_input += 1;
    }
    
    return m_input;
}


#[test]
fn test()
{
    use std::time::SystemTime; //For curiosity

    let mut time = SystemTime::now();
    assert_eq!(next_prime(12), 13);
    println!("The next_prime function took: {} ms to complete", time.elapsed().unwrap().as_millis()); //19485ms

    time = SystemTime::now();
    assert_eq!(next_prime(24), 29);
    println!("The next_prime function took: {} ms to complete", time.elapsed().unwrap().as_millis()); //19401ms

    time = SystemTime::now();
    assert_eq!(next_prime(11), 11);
    println!("The next_prime function took: {} ms to complete/n", time.elapsed().unwrap().as_millis()); //19496ms
}

//Could probably gain time by making this multi-threaded
//Just calculated: it would take 26 million hours to calculate all the primes up to 4 billion
pub fn pre_compute_primes()
{
    let mut vector = Vec::new();
 
    let mut is_prime = true;
    
    for pnum in 1.. u32::MAX
    {
        let percent = (pnum as f32 / u32::MAX as f32) * 100.0;
        println!("{}, {}", pnum, percent);

        for num in 1.. u32::MAX
        {
            if num != 1 && num != pnum && pnum % num == 0
            {
                is_prime = false;
                break
            }  

        }

        if is_prime {vector.push(pnum)}
    }
    
}

use std::{fs::File, io::{Read, Write}};
use std::str::from_utf8;

fn store_primes_in_file(primes: Vec<u32>)
{
    let serialized = serialize_primes(&primes);
    let mut file = File::create("day_six_primes.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}

fn serialize_primes(primes: &Vec<u32>) -> String
{
    return serde_json::to_string(&primes).unwrap();
}

fn gather_primes_from_file() -> Vec<u32>
{
    let buf: &mut [u8] = &mut [0; 1000];
    let file = File::open("day_six_primes.json")
        .unwrap()
        .read(buf);
    let string: &str = from_utf8(buf).unwrap();

    return deserialize_primes(string.to_owned());
}

fn deserialize_primes(file: String) -> Vec<u32>
{
    let deserialize: Vec<u32> = serde_json::from_str(file.as_str()).unwrap();
    return deserialize;
}


fn next_prime_precompiled(input: u32) -> u32
{
    let vector = gather_primes_from_file();
    let mut m_input = input;
    
    // should get the first number in the vector that is greater or equal to m_input
    loop{

        for num in &vector
        {
            if num == &m_input
            {
                return *num;
            }  
        }

        m_input += 1;
    }
}

//#[test]
fn test_2()
{
    use std::time::SystemTime; //For curiosity

    let mut time = SystemTime::now();
    assert_eq!(next_prime_precompiled(12), 13);
    println!("{}", time.elapsed().unwrap().as_millis()); 

    time = SystemTime::now();
    assert_eq!(next_prime_precompiled(24), 29);
    println!("{}", time.elapsed().unwrap().as_millis()); 

    time = SystemTime::now();
    assert_eq!(next_prime_precompiled(11), 11);
    println!("{}", time.elapsed().unwrap().as_millis()); 

    //would take years to calculate
}


fn next_prime_another_attempt(input: u32) -> Option<u32>
{
    if is_prime(input)
    {
        return Some(input);
    }

    for num in (input..).map(|x| x + 1)
    {
        if is_prime(num)
        {
            return Some(num);
        }
    }

    return None;
}

fn is_prime(input: u32) -> bool
{
    if input <= 1 {return false}

    let prime = |input| -> u32
    {
        if input % 2 == 0 {return 2};
        for num in (1..).map(|x| x * 2 + 1).take_while(|x| x * x <= input)
        {
            if input % num == 0 {return num}
        }
        
        return input;
    };

    return prime(input) == input;

}

#[test]
fn test_3()
{
    use std::time::SystemTime; //For curiosity

    let mut time = SystemTime::now();
    assert_eq!(next_prime_another_attempt(12), Some(13));
    println!("The next_prime_another_attempt function took: {} ns to complete", time.elapsed().unwrap().as_nanos()); //9900ns

    time = SystemTime::now();
    assert_eq!(next_prime_another_attempt(24), Some(29));
    println!("The next_prime_another_attempt function took: {} ns to complete", time.elapsed().unwrap().as_nanos()); //500ns 

    time = SystemTime::now();
    assert_eq!(next_prime_another_attempt(11), Some(11));
    println!("The next_prime_another_attempt function took: {} ns to complete/n", time.elapsed().unwrap().as_nanos()); //300ns

    // this is the best one so far, way faster
}