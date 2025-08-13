//! Global read-only storage
//!
//! ```
//! use macroquad::experimental::collections::storage;
//!
//! struct WorldBoundaries(i32);
//!
//! fn draw_player() {
//!   let boundaries: i32 = storage::get::<WorldBoundaries>().0;
//!   assert_eq!(boundaries, 23);
//! }
//!
//! storage::store(WorldBoundaries(23));
//! draw_player();
//! ```

use std::any::{Any, TypeId};

use std::collections::HashMap;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

std::thread_local! {
    static STORAGE: RefCell<Option<HashMap<TypeId, Box<dyn Any>>>> = RefCell::new(None);
}

fn get_storage() -> &'static mut HashMap<TypeId, Box<dyn Any>> {
    STORAGE.with(|storage_cell| {
        let mut storage_opt = storage_cell.borrow_mut();
        if storage_opt.is_none() {
            *storage_opt = Some(HashMap::new());
        }

        // Safe for same reasons as main CONTEXT - single-threaded design
        let storage = storage_opt.as_mut().unwrap();
        unsafe {
            std::mem::transmute::<
                &mut HashMap<TypeId, Box<dyn Any>>,
                &'static mut HashMap<TypeId, Box<dyn Any>>,
            >(storage)
        }
    })
}

/// Store data in global storage.
/// Will silently overwrite an old value if any.
pub fn store<T: Any>(data: T) {
    get_storage().insert(TypeId::of::<T>(), Box::new(Rc::new(RefCell::new(data))));
}

/// Get reference to data from global storage.
/// Will panic if there is no data available with this type.
pub fn get<T: Any>() -> impl Deref<Target = T> {
    try_get::<T>().unwrap()
}

/// Get reference to data from global storage.
/// Will return None if there is no data available with this type.
pub fn try_get<T: Any>() -> Option<impl Deref<Target = T>> {
    get_storage()
        .get(&TypeId::of::<T>())
        .as_ref()
        .and_then(|data| {
            data.downcast_ref::<Rc<RefCell<T>>>()
                .map(|data| data.borrow())
        })
}

/// Get mutable reference to data from global storage.
/// Will return None if there is no data available with this type.
pub fn try_get_mut<T: Any>() -> Option<impl DerefMut<Target = T>> {
    get_storage()
        .get(&TypeId::of::<T>())
        .as_ref()
        .and_then(|data| {
            data.downcast_ref::<Rc<RefCell<T>>>()
                .map(|data| data.borrow_mut())
        })
}

/// Get mutable reference to data from global storage.
/// Will panic if there is no data available with this type.
pub fn get_mut<T: Any>() -> impl DerefMut<Target = T> {
    try_get_mut::<T>().unwrap()
}
