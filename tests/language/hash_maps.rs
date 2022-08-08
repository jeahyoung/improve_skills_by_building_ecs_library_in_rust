use std::collections::HashMap;

#[test]
fn hash_maps(){
    // HashMap 생성
    let mut hash_map = HashMap::new();
    hash_map.insert(Letters::A, vec!['a']);

    //let letters = hash_map.entry(Letters::B).or_insert(vec!['b']);
    let letters = hash_map.entry(Letters::B).or_insert_with(|| vec!['b']);
    letters.push('b');
    let a = hash_map.get_mut(&Letters::A).unwrap();
    dbg!(&hash_map);

    for (letter, characters) in &hash_map {
        println!("inside values");
        println!("{:?} : {:?}", letter, characters);
    }

    // hash_map.iter().for_each(|(letter, characters)|{
    //     dbg!(characters);
    // });

    // hash_map.keys().for_each(|letter|{
    //     dbg!(letter);
    // });

    // hash_map.values().for_each(|characters|{
    //     dbg!(characters);
    // });

    if hash_map.contains_key(&Letters::A) {
        dbg!("We have A");
    }

    // hash_map.retain(|key, value| false);
    // dbg!(hash_map.len());

    hash_map.remove(&Letters::B);
    dbg!(hash_map.len());
    hash_map.clear();
    dbg!(hash_map.len());
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Letters {
    A,
    B,
    C
}
// { key: value }