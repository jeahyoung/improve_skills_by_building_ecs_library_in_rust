mod resource;

use std::any::Any;
use crate::resource::Resource;

#[derive(Default,Debug)]
pub struct World {
    resources: Resource,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, resource_data: impl Any){
        self.resources.add(resource_data);
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /// Query for a resource and get a mutable reference to it.
    /// The type of the resource must be added in so that we can find it.
    /// ```
    /// use improve_skills_by_building_ecs_library_in_rust::World;
    /// let mut world = World::new();
    /// world.add_resource(10_u32);
    /// {
    /// let resource = world.get_resource_mut::<u32>().unwrap();
    /// *resource += 1;
    /// }
    /// let resource = world.get_resource::<u32>().unwrap();
    /// assert_eq!(*resource,11);
    /// ```
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /**
    This will remove the resource from the world, and it doesn't care if the resource exists at this point in time.
    */
    pub fn delete_resource<T: Any>(&mut self) {
        self.resources.remove::<T>();
    }
}

#[cfg(test)]
mod tests {
}