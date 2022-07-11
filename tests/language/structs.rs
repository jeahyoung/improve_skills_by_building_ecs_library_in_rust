use std::ops::Deref;

#[test]

// struct
// tuple struct
// anonymous structs

fn structs(){
    //let mut player_health = Health { data: 100};
    let mut player_health = Health(100);
    let enemy_health = Health::new(100);

    player_health.lose_health(10);
    player_health.reset();
    player_health.print_health();

    let _ice_cream = FavoriteFood::IceCream {
        topping: "Nothing".to_string(),
        scoops: 1,
        flavor: "Space Junkie".to_string()
    };
}

// struct Health {
//     data: u32,
// }
struct Health(u32);


impl Health {
    pub fn new(data: u32) -> Self {
        //Self { data }
        Self(data)
    }

    pub fn lose_health(&mut self, amount: u32) {
        //self.data -= amount;
        self.0 -= amount;
    }

    pub fn print_health(&self){

        //println!("health: {}", self.data);
        println!("health: {:?}", **self);
    }

    fn reset(&mut self){
        self.0 = 100;
    }
}

impl Deref for Health {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum FavoriteFood {
    Hamburger
    , HotDog
    , IceCream {
        topping: String,
        scoops: u32,
        flavor: String,
    }
    ,
}