use ::*;

#[derive(Debug)]
pub struct VecMap<K: AsUsize + Copy, V> {
    data: Vec<Option<V>>,
    phantom_data: PhantomData<K>,
}

pub mod internal {
    pub trait AsUsize {
        fn as_usize(&self) -> usize;
    }

    impl AsUsize for u32 {
        fn as_usize(&self) -> usize {
            (*self) as usize
        }
    }
}

use self::internal::*;

impl<K: AsUsize + Copy, V> VecMap<K, V> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            phantom_data: PhantomData,
        }
    }
    // TODO: impl Trait for static dispatch
    pub fn values<'a>(&'a self) -> Box<Iterator<Item=&'a V> + 'a> {
        Box::new(self.data.iter().filter_map(|x| x.as_ref()))
    }
    pub fn values_mut<'a>(&'a mut self) -> Box<Iterator<Item=&'a mut V> + 'a> {
        Box::new(self.data.iter_mut().filter_map(|x| x.as_mut()))
    }
    pub fn insert(&mut self, id: K, value: V) {
        let id = id.as_usize();
        while self.data.len() <= id {
            self.data.push(None);
        }
        self.data[id] = Some(value);
    }
    pub fn get<'a>(&'a self, id: &K) -> Option<&'a V> {
        let id = id.as_usize();
        if id >= self.data.len() {
            None
        } else {
            self.data[id].as_ref()
        }
    }
    pub fn get_mut<'a>(&'a mut self, id: &K) -> Option<&'a mut V> {
        let id = id.as_usize();
        if id >= self.data.len() {
            None
        } else {
            self.data[id].as_mut()
        }
    }
    pub fn contains_key(&self, id: &K) -> bool {
        let id = id.as_usize();
        if id >= self.data.len() {
            false
        } else {
            self.data[id].is_some()
        }
    }
    pub fn remove(&mut self, id: &K) -> Option<V> {
        let id = id.as_usize();
        if id >= self.data.len() {
            None
        } else {
            let mut swp = None;
            std::mem::swap(&mut swp, &mut self.data[id]);
            swp
        }
    }
}