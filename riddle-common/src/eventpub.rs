use std::sync::{Arc, Mutex, RwLock, Weak};

struct EventQueue<T> {
    queue: Mutex<Vec<T>>,
    filter: Box<dyn Fn(&T) -> bool + Sync + Send>,
}

pub struct EventPub<T> {
    subs: RwLock<Vec<Weak<EventQueue<T>>>>,
}

pub struct EventSub<T> {
    events: Arc<EventQueue<T>>,
}

impl<T: Clone> EventPub<T> {
    pub fn new() -> Self {
        Self {
            subs: RwLock::new(vec![]),
        }
    }

    pub fn attach(&self, sub: &EventSub<T>) {
        let mut subs = self.subs.write().unwrap();
        subs.push(Arc::downgrade(&sub.events));
    }

    pub fn dispatch(&self, event: &T) {
        let mut dirty = false;

        for sub in self.subs.read().unwrap().iter() {
            match Weak::upgrade(sub) {
                Some(strong_sub) => strong_sub.deliver(event.clone()),
                None => dirty = true,
            }
        }

        if dirty {
            self.clean()
        }
    }

    fn clean(&self) {
        let mut subs = self.subs.write().unwrap();
        subs.retain(|w| Weak::upgrade(w).is_some())
    }
}

impl<T> EventSub<T> {
    pub fn new() -> Self {
        Self {
            events: EventQueue::new().into(),
        }
    }

    pub fn new_with_filter<F>(filter: F) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        Self {
            events: EventQueue::new_with_filter(filter).into(),
        }
    }

    pub fn collect(&self) -> Vec<T> {
        self.events.collect()
    }
}

impl<T> EventQueue<T> {
    fn new() -> Self {
        Self::new_with_filter(|_| true)
    }

    fn new_with_filter<F>(filter: F) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        Self {
            queue: Mutex::new(vec![]),
            filter: Box::new(filter),
        }
    }

    fn deliver(&self, event: T) {
        if (*self.filter)(&event) {
            let mut queue = self.queue.lock().unwrap();
            queue.push(event);
        }
    }

    fn collect(&self) -> Vec<T> {
        let mut queue = self.queue.lock().unwrap();
        let mut res = vec![];
        res.append(&mut queue);
        res
    }
}
