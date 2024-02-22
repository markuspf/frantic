use std::sync::atomic::Ordering;
use std::{ptr, sync::atomic::AtomicPtr};

#[derive(Default)]
struct MPSCQueueNode {
    value: u64,
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
        let mut q = MPSCQueue::default();
        q.head.store(&mut q.stub, Ordering::Release);
        q.tail.store(&mut q.stub, Ordering::Release);
        q
    }

    fn push(&mut self, node: *mut MPSCQueueNode) {
        unsafe {
            (*node).next.store(ptr::null_mut(), Ordering::Relaxed);
        };

        let previous = self.head.swap(node, Ordering::Relaxed);

        unsafe {
            (*previous).next.store(node, Ordering::Release);
        };
    }

    fn empty(&self) -> bool {
        let current_tail = self.tail.load(Ordering::Acquire);
        let current_head = self.head.load(Ordering::Acquire);

        current_head == current_tail
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

#[test]
fn construct_empty() {
    let queue = MPSCQueue::new();
    assert!(queue.empty());
}

#[test]
fn construct_push_not_empty() {
    let mut queue = MPSCQueue::new();
    let mut n = MPSCQueueNode::default();

    queue.push(&mut n);

    assert!(!queue.empty());
}

fn main() {
    println!("Hello, world!");
}
