use ::*;

#[derive(Debug)]
pub struct ProfiledRegion {
    pub name: &'static str,
    pub time_consumed: f64,
    pub invocation_count: usize,
    pub children: Vec<ProfiledRegion>,
}

impl ProfiledRegion {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            time_consumed: 0.0,
            invocation_count: 0,
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
            println!("{:.2}% ({} ms, {} calls) - {}",
                     100.0 * self.time_consumed / super_total,
                     (self.time_consumed * 1000.0) as usize,
                     self.invocation_count,
                     self.name);
        }
        let mut children: Vec<_> = self.children.iter().collect();
        children.sort_by(|a, b| b.time_consumed.partial_cmp(&a.time_consumed).unwrap());
        for child in children {
            child.pretty_print(indent + 1, self.time_consumed);
        }
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

    pub fn start_scope(&self, name: &'static str) -> Timer {
        let mut root = self.root.borrow_mut();
        let mut current_position = self.current_position.borrow_mut();
        let position = root.get_child_rec(&current_position);
        current_position.push(
            if let Some((index, _)) = position.children.iter()
                .enumerate().find(|&(_, ref child)| child.name == name) {
                index
            } else {
                position.children.push(ProfiledRegion::new(name));
                position.children.len() - 1
            });
        Timer::new()
    }

    pub fn end_scope(&self, timer: Timer) {
        let mut current_position = self.current_position.borrow_mut();
        let mut root = self.root.borrow_mut();
        let position = root.get_child_rec(&current_position);
        position.time_consumed += timer.elapsed();
        position.invocation_count += 1;
        current_position.pop().unwrap();
    }

    pub fn scoped<R, F: FnOnce() -> R>(&self, name: &'static str, f: F) -> R {
        let timer = self.start_scope(name);
        let result = f();
        self.end_scope(timer);
        result
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