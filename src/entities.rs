use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Entities {
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
}

impl Entities {
    pub fn register_component<T: Any + 'static>(&mut self) {
        self.components.insert(TypeId::of::<T>(), vec![]);
    }

    pub fn create_entity(&mut self) {
        self.components
            .iter_mut()
            .for_each(|(key, components)| components.push(None));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::any::TypeId;

    #[test]
    fn register_an_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let type_id = TypeId::of::<Health>();
        let health_components = entities.components.get(&type_id).unwrap();
        assert_eq!(health_components.len(), 0);
        dbg!(entities);
    }

    #[test]
    fn create_entity() {
        let mut entites = Entities::default();
        entites.register_component::<Health>();
        entites.register_component::<Speed>();
        entites.create_entity();
        let health = entites.components.get(&TypeId::of::<Health>()).unwrap();
        let speed = entites.components.get(&TypeId::of::<Speed>()).unwrap();
        assert!(health.len() == speed.len() && health.len() == 1);
        assert!(health[0].is_none() && speed[0].is_none());
        dbg!(entites);
    }

    struct Health(pub u32);
    struct Speed(pub u32);
}
