use std::any::{Any, TypeId};
use std::collections::HashMap;

#[test]

fn type_id(){
    let mut components:  HashMap<TypeId, Vec<Box<dyn Any + 'static>>> = HashMap::new();
    let health = 100_u32;
    let health_type_id = TypeId::of::<u32>();
    components.insert(health_type_id, vec![Box::new(health)]);
    let speed = Speed(150);
    let speed_type_id = speed.type_id();
    components.insert(speed_type_id, vec![Box::new(speed)]);

    for (component_type_id, component_value) in components{
        println!("{:?} {:?}", component_type_id, component_value);
        let type_id = component_value[0].type_id();
        //dbg!(type_id);
        println!("{:?}", type_id);
    }
}

struct Speed(u32);
