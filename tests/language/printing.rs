#[test]
fn printing(){
    let cat = Some("Xilbe");

    let _something = dbg!(cat);
    //let st = dbg!(&cat);
    //println!("The cat's name is {:?}.", cat);
}