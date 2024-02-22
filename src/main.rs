use std::ptr::null_mut;
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

    fn pop(&mut self) -> *mut MPSCQueueNode {
        let mut current = self.tail.load(Ordering::Acquire);
        let mut next = unsafe { (*current).next.load(Ordering::Acquire) };
        let stub = &mut self.stub as *mut _;

        if current == stub {
            // The queue does not contain a
            // reachable element
            if next == null_mut() {
                return null_mut();
            }

            self.tail.store(next, Ordering::Release);
            current = next;
            next = unsafe { (*next).next.load(Ordering::Acquire) };
        }

        if next != null_mut() {
            self.tail.store(next, Ordering::Release);
            return current;
        }

        let current_head = self.head.load(Ordering::Acquire);

        if self.tail.load(Ordering::Acquire) != current_head {
            return null_mut();
        }

        self.push(stub);

        next = unsafe { (*current).next.load(Ordering::Acquire) };

        if next != null_mut() {
            self.tail.store(next, Ordering::Release);
            unsafe { (*current).next.store(null_mut(), Ordering::Release) };
        }

        return null_mut();
    }
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

#[test]
fn push_n_pop() {
    let mut queue = MPSCQueue::new();
    let mut n1 = MPSCQueueNode::default();
    let mut n2 = MPSCQueueNode::default();
    let mut n3 = MPSCQueueNode::default();

    queue.push(&mut n1);
    queue.push(&mut n2);
    queue.push(&mut n3);

    assert!(!queue.empty());

    let m1 = queue.pop();
    let m2 = queue.pop();
    let m3 = queue.pop();

    assert!(queue.empty());
}

fn main() {
    let mut queue = MPSCQueue::new();
    let mut n1 = MPSCQueueNode::default();
    let mut n2 = MPSCQueueNode::default();
    let mut n3 = MPSCQueueNode::default();

    queue.push(&mut n1);
    queue.push(&mut n2);
    queue.push(&mut n3);

    assert!(!queue.empty());

    let m1 = queue.pop();
    let m2 = queue.pop();
    let m3 = queue.pop();

    assert!(queue.empty());

    println!("Hello, world!");
}
