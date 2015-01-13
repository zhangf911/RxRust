use std::sync::mpsc::{SyncSender, Sender};
use mio::EventLoopSender;

/// Sendable
/// Wrapper trait for all types of queue senders
pub trait Sendable : Sized + Clone {
    type Item;
    fn send(&self, a: Self::Item) -> Result<(), Self::Item>;
}

impl<A : Send> Sendable for Sender<A> {
    type Item = A;
    fn send(&self, a: <Self as Sendable>::Item) -> Result<(), <Self as Sendable>::Item> {
        match self.send(a) {
            Ok(_) => Ok(()),
            Err(e)  => Err(e.0)
        }
    }
}

impl<A : Send> Sendable for SyncSender<A> {
    type Item = A;
    fn send(&self, a: <Self as Sendable>::Item) -> Result<(), <Self as Sendable>::Item> {
        match self.send(a) {
            Ok(_) => Ok(()),
            Err(e)  => Err(e.0)
        }
    }
}

impl<A : Send + Clone> Sendable for EventLoopSender<A> {
    type Item = A;
    fn send(&self, a: <Self as Sendable>::Item) -> Result<(), <Self as Sendable>::Item> {
        self.send(a)
    }
}
