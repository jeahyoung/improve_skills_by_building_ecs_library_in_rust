use std::any::{Any, TypeId};
use std::collections::HashMap;

#[derive(Default,Debug)]
pub struct Resource {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl Resource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, data: impl Any) {
        let type_id = data.type_id();
        self.data.insert(type_id, Box::new(data));
    }

    pub fn get_ref<T: Any>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get(&type_id) {
            data.downcast_ref::<T>()
        } else {
            None
        }
    }

    pub fn get_mut<T: Any>(&mut self) -> Option<&mut T> {

        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data.get_mut(&type_id) {
            data.downcast_mut()
        } else {
            None
        }
    }

    pub fn remove<T: Any>(&mut self){
        let type_id = TypeId::of::<T>();
        self.data.remove(&type_id);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add_resource() {
        // //let mut resource = Resource::default();
        // let mut resource = initialize_resource();
        // let world_width = WorldWidth(100.0);
        // let world_with_type_id = world_width.type_id();
        //
        // resource.add(world_width);
        //
        // //let stored_resource = resource.data.get(&world_with_type_id).unwrap();
        // let stored_resource = resource.data.get(&TypeId::of()::<WorldWidth>()).unwrap();
        // let extracted_world_width = stored_resource.downcast_ref::<WorldWidth>().unwrap();
        //
        // assert_eq!(extracted_world_width.0, 100.0);
        let resources = initialize_resource();

        let stored_resource = resources.data.get(&TypeId::of::<WorldWidth>()).unwrap();
        let extracted_world_width = stored_resource.downcast_ref::<WorldWidth>().unwrap();
        assert_eq!(extracted_world_width.0, 100.0);
    }

    #[test]
    fn get_resource(){
        // let mut resource = Resource::default();
        // let world_width = WorldWidth(100.0);
        //
        // resource.add(world_width);
        //
        // if let Some(extracted_world_width) = resource.get_ref::<WorldWidth>(){
        //     assert_eq!(extracted_world_width.0, 100.0);
        // }
        let resource = initialize_resource();

        if let Some(extracted_world_width) = resource.get_ref::<WorldWidth>(){
            assert_eq!(extracted_world_width.0, 100.0);
        }
    }

    #[test]
    fn get_resource_mut(){
        let mut resource = initialize_resource();
        {
            let world_width: &mut WorldWidth = resource.get_mut::<WorldWidth>().unwrap();
            world_width.0 += 1.0;
        }
        let world_width = resource.get_ref::<WorldWidth>().unwrap();
        assert_eq!(world_width.0, 101.0);
    }

    #[test]
    fn remove_resource() {
        let mut resources = initialize_resource();
        resources.remove::<WorldWidth>();
        let world_width_type_id = TypeId::of::<WorldWidth>();
        assert!(!resources.data.contains_key(&world_width_type_id));
    }

    fn initialize_resource() -> Resource {
        let mut resources = Resource::default();
        let world_width = WorldWidth(100.0);

        resources.add(world_width);
        resources
    }

    struct WorldWidth(pub f32);
}