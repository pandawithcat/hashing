use sha2::{Digest, Sha256};
use std::cmp::min;
use std::collections::HashMap;
use std::fs;
type Pair = (String, usize);
type PairList = Vec<Pair>;
type HashTable = HashMap<u8, PairList>;

fn main() {
    println!("Starting Hw3");
    run_problem1();
    run_problem2();
    run_problem2_c_d();
}

fn run_problem1() {
    println!("----- Running problem1--------");

    println!("----- Running problem1.a--------");
    let mut hasher = Sha256::new();
    hasher.update("hello");
    let result = hasher.finalize();
    println!("{:?}, {:?}, {:?}", result[0], result[1], result[2]);

    println!("----- Running problem1.b and problem1.c --------");
    let filename: String = String::from("stream.txt");
    println!("Filename: {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //println!("{}", contents);
    let words: Vec<&str> = contents.split(' ').collect();
    println!("{}", words.len());

    //initialize the hashmap
    let mut hash_table = HashTable::default();

    // start the loop
    for word in words {
        let mut hasher = Sha256::new();
        hasher.update(word);
        let result = hasher.finalize();
        let index = result[0];
        if hash_table.contains_key(&index) {
            let mut pairs: Vec<Pair> = hash_table.get(&index).unwrap().clone();
            //loop pairs to see if the string already exists;
            // if it does add 1 to the value
            let mut exist = false;
            for i in 0..pairs.len() {
                if pairs[i].0 == word {
                    pairs[i].1 = pairs[i].1 + 1;
                    exist = true;
                }
            }
            if !exist {
                pairs.push((word.to_string(), 1));
            }
            hash_table.insert(index, pairs);
        } else {
            let mut new_pair = Pair::default();
            new_pair.0 = word.to_string();
            new_pair.1 = 1;
            let mut newPairList = PairList::default();
            newPairList.push(new_pair);
            hash_table.insert(index, newPairList);
        }
    }

    //print the hash_table stats
    // answer the questions
    // the, are, sydney, london
    println!(
        "the appears: {:?} times",
        count(String::from("the"), hash_table.clone())
    );

    println!(
        "are appears: {:?} times",
        count(String::from("are"), hash_table.clone())
    );

    println!(
        "sydney appears: {:?} times",
        count(String::from("sydney"), hash_table.clone())
    );

    println!(
        "london appears: {:?} times",
        count(String::from("london"), hash_table.clone())
    );

    println!("----- Running problem1.d --------");
    //loop through the hash table and calculate values;
    let mut sum = 0;

    for value in hash_table.values() {
        for pair in value.clone() {
            sum += 4 * pair.0.chars().count() + 1;
        }
    }
    println!("This is the total memory used: {:?}", sum);
    println!("----- Problem1 has ended --------");
}

fn count(word: String, hash_table: HashTable) -> usize {
    let mut answer = 0;
    let mut hasher = Sha256::new();
    hasher.update(word.clone());
    let result = hasher.finalize();
    let index = result[0];
    for key in hash_table.keys() {
        if *key == index {
            let pairs = hash_table.get(&key).unwrap().clone();
            for i in 0..pairs.len() {
                if pairs[i].0 == word {
                    answer = pairs[i].1;
                }
            }
        }
    }
    return answer;
}

fn run_problem2() {
    println!("----- Running problem2--------");

    let mut cms_hash_table: [[u32; 256]; 5] = [[0; 256]; 5];

    println!(
        "row, col: {}, {}",
        cms_hash_table.len(),
        cms_hash_table[0].len()
    );

    let filename: String = String::from("stream.txt");
    println!("Filename: {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //println!("{}", contents);
    let words: Vec<&str> = contents.split(' ').collect();
    println!("{}", words.len());

    for word in words {
        let mut hasher = Sha256::new();
        hasher.update(word);
        let result = hasher.finalize();
        //println!("word: {}, hash: {:?}", word, result);
        cms_hash_table[0][result[0] as usize] += 1;
        cms_hash_table[1][result[1] as usize] += 1;
        //println!("current hash: {:?}", cms_hash_table[2][result[2] as usize]);
        cms_hash_table[2][result[2] as usize] += 1;
        cms_hash_table[3][result[3] as usize] += 1;
        cms_hash_table[4][result[4] as usize] += 1;
    }

    println!("----- Running problem2.a--------");

    println!(
        "this is the test value for paris: {}",
        test_cms(String::from("paris"), cms_hash_table)
    );

    println!(
        "this is the test value for her: {}",
        test_cms(String::from("her"), cms_hash_table)
    );

    println!(
        "this is the test value for well: {}",
        test_cms(String::from("well"), cms_hash_table)
    );

    println!("----- Running problem2.b--------");

    println!(
        "this is the value for the: {}",
        test_cms(String::from("the"), cms_hash_table)
    );

    println!(
        "this is the value for are: {}",
        test_cms(String::from("are"), cms_hash_table)
    );

    println!(
        "this is the value for sydney: {}",
        test_cms(String::from("sydney"), cms_hash_table)
    );

    println!(
        "this is the value for london: {}",
        test_cms(String::from("london"), cms_hash_table)
    );
}

fn run_problem2_c_d() {
    println!("----- Running problem2.c and problem2.d--------");

    let mut cms_hash_table: [[u32; 256]; 5] = [[0; 256]; 5];

    println!(
        "row, col: {}, {}",
        cms_hash_table.len(),
        cms_hash_table[0].len()
    );

    let filename: String = String::from("stream.txt");
    println!("Filename: {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //println!("{}", contents);
    let words: Vec<&str> = contents.split(' ').collect();
    println!("{}", words.len());

    for word in words {
        let mut hasher = Sha256::new();
        hasher.update(word);
        let result = hasher.finalize();
        //find the minimum
        let mut min_value = cms_hash_table[0][result[0] as usize];
        for i in 0..5 {
            min_value = min(min_value, cms_hash_table[i][result[i] as usize]);
        }

        for i in 0..5 {
            if min_value == cms_hash_table[i][result[i] as usize] {
                cms_hash_table[i][result[i] as usize] += 1;
            }
        }
    }

    println!(
        "this is the value for the: {}",
        test_cms(String::from("the"), cms_hash_table)
    );

    println!(
        "this is the value for are: {}",
        test_cms(String::from("are"), cms_hash_table)
    );

    println!(
        "this is the value for sydney: {}",
        test_cms(String::from("sydney"), cms_hash_table)
    );

    println!(
        "this is the value for london: {}",
        test_cms(String::from("london"), cms_hash_table)
    );

    println!("----- Running problem2.f--------");

    println!(
        "this is the total memory(bytes) being used: {:?}",
        5 * 256 * 4
    );

    println!("----- Problem2 has ended --------");
}
fn test_cms(word: String, cms_hash_table: [[u32; 256]; 5]) -> u32 {
    let mut hasher = Sha256::new();
    hasher.update(word);
    let result = hasher.finalize();
    let first = min(
        cms_hash_table[0][result[0] as usize],
        cms_hash_table[1][result[1] as usize],
    );
    let second = min(first, cms_hash_table[2][result[2] as usize]);
    let third = min(second, cms_hash_table[3][result[3] as usize]);
    return min(third, cms_hash_table[4][result[4] as usize]);
}
