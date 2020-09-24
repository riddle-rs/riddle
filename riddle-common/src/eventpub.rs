/*!
An event pubsub utility.

Publishers can have multiple subscribers, and subscribers can filter the events
they accept.

# Example

```
# use riddle_common::eventpub::*;
#[derive(Clone, Eq, PartialEq, Debug)]
enum Message {
    Test
}

fn main() {
    // Make a new pub and sub
    let publisher: EventPub<Message> = EventPub::new();
    let subscriber: EventSub<Message> = EventSub::new();
    publisher.attach(&subscriber);

    // Send a message through the pub
    publisher.dispatch(Message::Test);

    // Take the messages from the sub
    let messages = subscriber.collect();
    assert_eq!(vec![Message::Test], messages);
}
```
*/

use std::sync::{Arc, Mutex, RwLock, Weak};

/////////////////////////////////////////////////////////////////////////////
// struct EventQueue
/////////////////////////////////////////////////////////////////////////////

struct EventQueue<T> {
    queue: Mutex<Vec<T>>,
    filter: Box<dyn Fn(&T) -> bool + Sync + Send>,
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

/////////////////////////////////////////////////////////////////////////////
// struct EventPub
/////////////////////////////////////////////////////////////////////////////

/// Event publisher which can have multiple subscribers.
///
/// # Example
///
/// ```
/// # use riddle_common::eventpub::*;
/// #[derive(Clone)]
/// enum Message {
///     Test(u32)
/// }
///
/// fn main() {
///     let publisher: EventPub<Message> = EventPub::new();
///
///     // Attach any subscribers
///     // ...
///
///     publisher.dispatch(Message::Test(42));
///     publisher.dispatch(Message::Test(13));
/// }
/// ```
pub struct EventPub<T> {
    subs: RwLock<Vec<Weak<EventQueue<T>>>>,
}

impl<T: Clone> EventPub<T> {
    /// Create a new event publisher.
    ///
    /// The new publisher will have no subscribers, so any message dispatched to it in
    /// this state will be silently dropped.
    pub fn new() -> Self {
        Self {
            subs: RwLock::new(vec![]),
        }
    }

    /// Attach a subscriber to the publisher.
    ///
    /// Any events dispatched after this call will be registered with the subscriber.
    /// The subscriber won't receive any events dispatched before it has been attached.
    ///
    /// The subscriber is detached by dropping the attached [`EventSub`].
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_common::eventpub::*;
    /// # #[derive(Clone)] enum Message { Test(u32) }
    /// # fn main() {
    /// let publisher: EventPub<Message> = EventPub::new();
    /// let subscriber: EventSub<Message> = EventSub::new();
    ///
    /// publisher.attach(&subscriber);
    /// assert_eq!(1, publisher.subscription_count());
    ///
    /// drop(subscriber);
    /// assert_eq!(0, publisher.subscription_count());
    /// # }
    /// ```
    pub fn attach(&self, sub: &EventSub<T>) {
        let mut subs = self.subs.write().unwrap();
        subs.push(Arc::downgrade(&sub.events));
    }

    /// Send an event to all currently registered subscribers.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_common::eventpub::*;
    /// # #[derive(Clone, PartialEq, Eq, Debug)] enum Message { Test(u32) }
    /// # fn main() {
    /// let sub_a: EventSub<Message> = EventSub::new();
    /// let sub_b: EventSub<Message> = EventSub::new();
    /// let publisher: EventPub<Message> = EventPub::new();
    /// publisher.attach(&sub_a);
    /// publisher.attach(&sub_b);
    ///
    /// publisher.dispatch(Message::Test(0));
    ///
    /// assert_eq!(vec![Message::Test(0)], sub_a.collect());
    /// assert_eq!(vec![Message::Test(0)], sub_b.collect());
    /// # }
    /// ```
    pub fn dispatch(&self, event: T) {
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

    /// The current count of attached subscribers.
    pub fn subscription_count(&self) -> u32 {
        self.clean();
        self.subs.read().unwrap().len() as u32
    }

    fn clean(&self) {
        let mut subs = self.subs.write().unwrap();
        subs.retain(|w| Weak::upgrade(w).is_some())
    }
}

/////////////////////////////////////////////////////////////////////////////
// struct EventSub
/////////////////////////////////////////////////////////////////////////////

/// An event subscriber which can filter incoming messages.
///
/// # Example
///
/// ```
/// # use riddle_common::eventpub::*;
/// #[derive(Clone)]
/// enum Message {
///     Test(u32)
/// }
///
/// fn main() {
///     let subscriber: EventSub<Message> = EventSub::new();
///
///     // Attach to publisher
///     // ...
///
///     // At a later point consume events
///     let events: Vec<Message> = subscriber.collect();
/// }
/// ```
pub struct EventSub<T> {
    events: Arc<EventQueue<T>>,
}

impl<T> EventSub<T> {
    /// Create a new EventSub which accepts all events
    pub fn new() -> Self {
        Self {
            events: EventQueue::new().into(),
        }
    }

    /// Create a new EventSub which applies a filter on incoming events
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_common::eventpub::*;
    /// # #[derive(Clone, PartialEq, Eq, Debug)] enum Message { Test(u32) }
    /// # fn main() {
    /// // Subscriber only accepts even numbered messages
    /// let subscriber: EventSub<Message> = EventSub::new_with_filter(|Message::Test(v)| {
    ///     v % 2 == 0
    /// });
    ///
    /// let publisher: EventPub<Message> = EventPub::new();
    /// publisher.attach(&subscriber);
    ///
    /// publisher.dispatch(Message::Test(0));
    /// publisher.dispatch(Message::Test(1));
    /// publisher.dispatch(Message::Test(2));
    ///
    /// assert_eq!(vec![Message::Test(0), Message::Test(2)], subscriber.collect());
    /// # }
    /// ```
    pub fn new_with_filter<F>(filter: F) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        Self {
            events: EventQueue::new_with_filter(filter).into(),
        }
    }

    /// Return all received events and clear the subscribers buffer.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_common::eventpub::*;
    /// # #[derive(Clone, PartialEq, Eq, Debug)] enum Message { Test(u32) }
    /// # fn main() {
    /// let subscriber: EventSub<Message> = EventSub::new();
    /// let publisher: EventPub<Message> = EventPub::new();
    /// publisher.attach(&subscriber);
    ///
    /// publisher.dispatch(Message::Test(0));
    ///
    /// // First collect consumes all pending events
    /// assert_eq!(1, subscriber.collect().len());
    /// // Leaving none for the second collect
    /// assert_eq!(0, subscriber.collect().len());
    /// # }
    /// ```
    pub fn collect(&self) -> Vec<T> {
        self.events.collect()
    }
}
