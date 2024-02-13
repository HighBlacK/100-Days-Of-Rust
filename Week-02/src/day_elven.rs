pub fn restore_ip_addresses(string: String) -> Vec<String>
{
    let mut all_ip_addresses: Vec<String> = Vec::new();
    let mut path: [i32; 4] = [i32::default(); 4];
    snapshot_ip(&mut all_ip_addresses, string, 0, &mut path, 0);
    all_ip_addresses
}

fn snapshot_ip(
    all_ip_adresses: &mut Vec<String>, 
    string: String,
    builder_index: usize,
    path: &mut [i32; 4],
    segment: usize
)
{
    let string_length = string.len();
    
    if (segment == 4) && (builder_index == string_length)
    {
        all_ip_adresses.push(format!("{}.{}.{}.{}", path[0], path[1], path[2], path[3]));
        return;
    }
    
    if (segment == 4) || (builder_index == string_length)
    {
        return;
    }

    for len in 1..=3
    {
        if builder_index + len > string_length
        {
            break;
        }

        let snapshot = string.get(builder_index.. (builder_index + len)).unwrap();
        let value: i32 = snapshot.parse().unwrap();
        
        if (value > 255) || (len >= 2 && string.chars().nth(builder_index) == Some('0'))
        {
            break;
        }

        path[segment] = value;
        snapshot_ip(all_ip_adresses, string.clone(), builder_index + len, path, segment + 1);

        path[segment] = -1;

    }
}


#[test]
fn test()
{
    assert_eq!(restore_ip_addresses(String::from("25525511135")), vec!["255.255.11.135","255.255.111.35"]);
    assert_eq!(restore_ip_addresses(String::from("0000")), vec!["0.0.0.0"]);
    assert_eq!(restore_ip_addresses(String::from("1111")), vec!["1.1.1.1"]);
    assert_eq!(restore_ip_addresses(String::from("010010")), vec!["0.10.0.10","0.100.1.0"]);
    assert_eq!(restore_ip_addresses(String::from("101023")), vec!["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]);
}