use ::*;

#[derive(Debug)]
pub struct VecMap<K: IntoUsize + Copy, V> {
    data: Vec<Option<V>>,
    phantom_data: PhantomData<K>,
}

pub mod internal {
    use ::*;

    pub trait IntoUsize {
        fn into_usize(&self) -> usize;
    }

    impl IntoUsize for u32 {
        fn into_usize(&self) -> usize {
            (*self) as usize
        }
    }

    pub struct ValuesIterator<'a, T: 'a> {
        pub data: &'a Vec<Option<T>>,
        pub index: usize,
    }

    impl<'a, T> Iterator for ValuesIterator<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<&'a T> {
            while self.index < self.data.len() {
                let result = self.data[self.index].as_ref();
                self.index += 1;
                if let Some(data) = result {
                    return Some(data);
                }
            }
            None
        }
    }

    pub struct ValuesIteratorMut<'a, T: 'a> {
        pub data: &'a mut Vec<Option<T>>,
        pub index: usize,
    }

    impl<'a, T> Iterator for ValuesIteratorMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<&'a mut T> {
            while self.index < self.data.len() {
                use std::ops::IndexMut;
                let data: *mut Vec<Option<T>> = self.data as *mut _;
                let data = unsafe { &mut (*data) };
                let result = data.index_mut(self.index);
                let result = result.as_mut();
                self.index += 1;
                if result.is_some() {
                    return result;
                }
            }
            None
        }
    }
}

use self::internal::*;

impl<K: IntoUsize + Copy, V> VecMap<K, V> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            phantom_data: PhantomData,
        }
    }
    pub fn values(&self) -> ValuesIterator<V> {
        ValuesIterator {
            data: &self.data,
            index: 0,
        }
    }
    pub fn values_mut(&mut self) -> ValuesIteratorMut<V> {
        ValuesIteratorMut {
            data: &mut self.data,
            index: 0,
        }
    }
    pub fn insert(&mut self, id: K, value: V) {
        let id = id.into_usize();
        while self.data.len() <= id {
            self.data.push(None);
        }
        self.data[id] = Some(value);
    }
    pub fn get<'a>(&'a self, id: &K) -> Option<&'a V> {
        let id = id.into_usize();
        if id >= self.data.len() {
            None
        } else {
            self.data[id].as_ref()
        }
    }
    pub fn get_mut<'a>(&'a mut self, id: &K) -> Option<&'a mut V> {
        let id = id.into_usize();
        if id >= self.data.len() {
            None
        } else {
            self.data[id].as_mut()
        }
    }
    pub fn contains_key(&self, id: &K) -> bool {
        let id = id.into_usize();
        if id > self.data.len() {
            false
        } else {
            self.data[id].is_some()
        }
    }
    pub fn remove(&mut self, id: &K) -> Option<V> {
        let id = id.into_usize();
        if id >= self.data.len() {
            None
        } else {
            let mut swp = None;
            std::mem::swap(&mut swp, &mut self.data[id]);
            swp
        }
    }
}