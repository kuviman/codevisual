use ::*;

#[derive(Debug)]
pub struct ProfiledRegion {
    pub name: &'static str,
    pub time_consumed: f64,
    pub children: Vec<ProfiledRegion>,
}

impl ProfiledRegion {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            time_consumed: 0.0,
            children: Vec::new(),
        }
    }
    fn get_child_rec<'a>(&'a mut self, index: &[usize]) -> &'a mut ProfiledRegion {
        if index.len() == 0 {
            self
        } else {
            self.children[index[0]].get_child_rec(&index[1..])
        }
    }
    fn pretty_print(&self, indent: usize, super_total: f64) {
        if indent == 0 {
            println!("Profiler data:");
        } else {
            for _ in 0..indent {
                print!(" ");
            }
            println!("{:.2}% - {}", 100.0 * self.time_consumed / super_total, self.name);
        }
        let mut children: Vec<_> = self.children.iter().collect();
        children.sort_by(|a, b| b.time_consumed.partial_cmp(&a.time_consumed).unwrap());
        for child in children {
            child.pretty_print(indent + 1, self.time_consumed);
        }
    }
}

pub struct ProfiledScope<'a> {
    timer: Timer,
    profiler: &'a Profiler,
    position: Vec<usize>,
}

impl<'a> ProfiledScope<'a> {
    fn new(profiler: &'a Profiler, name: &'static str) -> Self {
        let mut root = profiler.root.borrow_mut();
        let mut current_position = profiler.current_position.borrow_mut();
        let position = root.get_child_rec(&current_position);
        current_position.push(
            if let Some((index, _)) = position.children.iter()
                .enumerate().find(|&(_, ref child)| child.name == name) {
                index
            } else {
                position.children.push(ProfiledRegion::new(name));
                position.children.len() - 1
            });
        Self {
            timer: Timer::new(),
            profiler,
            position: current_position.deref().clone(),
        }
    }
}

impl<'a> Drop for ProfiledScope<'a> {
    fn drop(&mut self) {
        let mut root = self.profiler.root.borrow_mut();
        let position = root.get_child_rec(&self.position);
        position.time_consumed += self.timer.elapsed();
        self.profiler.current_position.borrow_mut().pop().unwrap();
    }
}

pub struct Profiler {
    timer: RefCell<Timer>,
    root: RefCell<ProfiledRegion>,
    current_position: RefCell<Vec<usize>>,
}

impl Profiler {
    pub ( crate ) fn new() -> Self {
        Self {
            timer: RefCell::new(Timer::new()),
            root: RefCell::new(ProfiledRegion::new("_root_")),
            current_position: RefCell::new(Vec::new()),
        }
    }
    pub fn new_scope(&self, name: &'static str) -> ProfiledScope {
        ProfiledScope::new(self, name)
    }
    pub ( crate ) fn tick(&self) {
        assert_eq!(self.current_position.borrow().len(), 0);
        let mut root = self.root.borrow_mut();
        root.time_consumed += self.timer.borrow_mut().tick();
        if root.time_consumed > 1.0 {
            root.pretty_print(0, root.time_consumed);
            *root = ProfiledRegion::new("_root_");
        }
    }
}