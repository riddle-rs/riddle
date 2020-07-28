use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct EventQueue<T> {
    queue: RefCell<Vec<T>>,
    filter: Box<dyn Fn(&T) -> bool>,
}

pub struct EventPub<T> {
    subs: RefCell<Vec<Weak<EventQueue<T>>>>,
}

pub struct EventSub<T> {
    events: Rc<EventQueue<T>>,
}

impl<T: Clone> EventPub<T> {
    pub fn new() -> Self {
        Self {
            subs: RefCell::new(vec![]),
        }
    }

    pub fn attach(&self, sub: &EventSub<T>) {
        self.subs.borrow_mut().push(Rc::downgrade(&sub.events));
    }

    pub fn dispatch(&self, event: &T) {
        let mut dirty = false;
        for sub in self.subs.borrow().iter() {
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
        self.subs
            .borrow_mut()
            .retain(|w| Weak::upgrade(w).is_some())
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
        F: Fn(&T) -> bool + 'static,
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
        F: Fn(&T) -> bool + 'static,
    {
        Self {
            queue: RefCell::new(vec![]),
            filter: Box::new(filter),
        }
    }

    fn deliver(&self, event: T) {
        if (*self.filter)(&event) {
            self.queue.borrow_mut().push(event);
        }
    }

    fn collect(&self) -> Vec<T> {
        self.queue.replace(vec![])
    }
}
