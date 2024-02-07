// Overall, the next_prime_another_attempt function is the best one

use std::fs::read_to_string;
use std::time::SystemTime;
use std::path::Path;

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
pub fn pre_compute_primes(path: &Path)
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
    
    store_primes_in_file(vector, path);
}

use std::{fs::File, io::Write};

fn store_primes_in_file(primes: Vec<u32>, path: &Path)
{
    let serialized = serialize_primes(&primes);
    let mut file = File::create(path).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    println!("Primes have been stored in file");
}

fn serialize_primes(primes: &Vec<u32>) -> String
{
    return serde_json::to_string(&primes).unwrap();
}

fn gather_primes_from_file(path: &Path) -> Vec<u32>
{
    let file = read_to_string(path).unwrap();
    return deserialize_primes(file);
}

fn deserialize_primes(file: String) -> Vec<u32>
{
    let deserialize: Vec<u32> = serde_json::from_str(file.as_str()).unwrap();
    return deserialize;
}


fn next_prime_precompiled(input: u32, path: &Path) -> u32
{
    let vector = gather_primes_from_file(path);
    let mut m_input = input;
    let time = SystemTime::now();

    // should get the first number in the vector that is greater or equal to m_input
    loop{

        for num in &vector
        {
            if num == &m_input
            {
                println!("The main operation took: {} ns to complete", time.elapsed().unwrap().as_nanos());
                return *num;
            }  
        }
        m_input += 1;
    }
}

#[test]
fn test_2()
{
    use std::time::SystemTime; //For curiosity
    let path = Path::new("primes(10000).json");
    let mut time = SystemTime::now();
    assert_eq!(next_prime_precompiled(12, path), 13);
    println!("The next_prime_precompiled function took: {} ns to complete/n", time.elapsed().unwrap().as_nanos()); //475000ns

    time = SystemTime::now();
    assert_eq!(next_prime_precompiled(24, path), 29);
    println!("The next_prime_precompiled function took: {} ns to complete/n", time.elapsed().unwrap().as_nanos()); //433300ns

    time = SystemTime::now();
    assert_eq!(next_prime_precompiled(11, path), 11);
    println!("The next_prime_precompiled function took: {} ns to complete/n", time.elapsed().unwrap().as_nanos()); //375600ns

    //The next version is still faster than the precompiled one
    //Surely because i'm doing file operations and deserialization
    //Without those, i got: 
    //9300ns, 44400ns, 200ns 
    //which is on par with the next_prime_another_attempt function except for the second test which is the more demanding one
}

fn next_prime_another_attempt(input: u32) -> Option<u32>
{
    for num in input..
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

use std::sync::{Arc, Mutex};

use eta::{Eta,TimeAcc};
use rayon::prelude::*;

//Set at u32::MAX result in a 2.1GB file - i added a limit argument cause i probably don't need to calculate all the primes up to 4 billion - It took 121 minutes to calculate all the primes up to 4 billion
//The file can"t be opened anyway and the deserialization panics
pub fn pre_compute_primes_multitheaded(limit: u32, path: &Path) {
    let time = Arc::new(Mutex::new(SystemTime::now()));
    let eta = Arc::new(Mutex::new(Eta::new(u32::MAX as usize, TimeAcc::SEC)));
    let elapsed = Arc::new(Mutex::new(SystemTime::now()));

    let vector: Vec<u32> = (1..limit)
        .into_par_iter()
        .filter_map(|pnum| 
        {
            let mut eta = eta.lock()
                .unwrap();

            eta.step();

            let mut time = time.lock()
                .unwrap();
            
            if time.elapsed()
                .unwrap()
                .as_secs() == 5 
            {
                println!("{}, ({}s elapsed)", *eta, elapsed.lock().unwrap().elapsed().unwrap().as_secs());
                *time = SystemTime::now();
            }

            drop(eta);
            drop(time);

            if is_prime(pnum) 
            {
                Some(pnum)
            } 
            else 
            {
                None
            }
        })
        .collect();

    store_primes_in_file(vector, path);
}