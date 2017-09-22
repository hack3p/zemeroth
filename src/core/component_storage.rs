// Vec of https://github.com/reem/rust-typemap ? Hmm...

// TODO: check (de)serialization to(from) RON

use std::collections::HashMap;
use std::hash::Hash;

// TODO: remove the mockup Id. Or remove the current Zemeroth`s ObjId
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct ObjId(u32);

#[derive(Debug, Clone)]
pub struct ComponentContainer<Id: Hash + Eq, V: Clone> {
    data: HashMap<Id, V>,
}

impl<Id: Hash + Eq + Copy, V: Clone> ComponentContainer<Id, V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn get_opt(&self, id: Id) -> Option<&V> {
        self.data.get(&id)
    }

    pub fn get(&self, id: Id) -> &V {
        self.get_opt(id).unwrap()
    }

    pub fn get_opt_mut(&mut self, id: Id) -> Option<&mut V> {
        self.data.get_mut(&id)
    }

    pub fn get_mut(&mut self, id: Id) -> &mut V {
        self.get_opt_mut(id).unwrap()
    }

    pub fn insert(&mut self, id: Id, data: V) {
        assert!(self.get_opt(id).is_none());
        self.data.insert(id, data);
    }

    pub fn remove(&mut self, id: Id) {
        assert!(self.get_opt(id).is_some());
        self.data.remove(&id);
    }
}

// TODO: allocate a new id! Check how it's done in `State` struct.

// TODO: iter over all ids
//
// Hmm, for this I should store a list of all alive entity Id's...
// Add an Id to this list on alloc_id (alloc_entity)
// and remove Id from the list on storage.remove(id).
//
// And what if and an entity looses all its components? Should I GC it? When?

// TODO: `Storage`? `ComponentData`? `ComponentState`?
macro_rules! make_storage {
    ($structname:ident<$id_type:ty>: { $($component:ident: $t:ty,)* } ) => {
        #[derive(Clone, Debug)]
        pub struct $structname {
            $(
                $component: ::core::component_storage::ComponentContainer<$id_type, $t>,
            )*
        }

        impl $structname {
            pub fn new() -> Self {
                Self {
                    $(
                        $component: ::core::component_storage::ComponentContainer::new(),
                    )*
                }
            }

            pub fn is_exist(&self, id: $id_type) -> bool {
                $(
                    if self.$component.get_opt(id).is_some() {
                        return true;
                    }
                )*
                false
            }

            // TODO: return a single lined String
            pub fn debug_print_entity(&self, id: $id_type) {
                $(
                    if let Some(component) = self.$component.get_opt(id) {
                        println!("{:?}", component);
                    }
                )*
            }
        }
    }
}
