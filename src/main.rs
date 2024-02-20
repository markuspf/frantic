use std::sync::atomic::Ordering;
use std::{ptr::null, sync::atomic::AtomicPtr};

#[derive(Default)]
struct MPSCQueueNode {
    next: AtomicPtr<MPSCQueueNode>,
}

#[derive(Default)]
struct MPSCQueue {
    head: AtomicPtr<MPSCQueueNode>,
    tail: AtomicPtr<MPSCQueueNode>,
    stub: MPSCQueueNode,
}

impl MPSCQueue {
    fn new() -> MPSCQueue {
        MPSCQueue::default()
    }

    fn push(&mut self, node: *mut MPSCQueueNode) {
        unsafe {
            (*node).next = null;
        }

        let prev = self
            .head
            .compare_exchange(self.head, node, Ordering::SeqCst, Ordering::Relaxed);

        prev.next.store(node, Ordering::Release);
    }
    /*

    fn pop(&mut self) -> () {
        let mut current = self.tail.load(Ordering::Acquire);
        let mut next = current.next.load(Ordering::Acquire);

        if current == self.stub {
            // The queue does not contain a
            // reachable element
            if next == null {
                return;
            }

            self.tail.store(next, Ordering::Release);
            current = next;
            next = next.load(Ordering::Acquire);
        }

        if next != null {
            self.tail.store(next, Ordering::Release);
            return; // tail;
        }

        let current_head = self.head.load(Ordering::Acquire);

        if self.tail != current_head {
            return;
        }

        self.push(&stub);

        next = current.next.load(Ordering::Acquire);

        if next != null {
            self.tail.store(next);
            current.next.store(null, Ordering::Release);
        }

        return ();
    }*/
}

fn main() {
    let x = MPSCQueue::new();

    println!("Hello, world!");
}
