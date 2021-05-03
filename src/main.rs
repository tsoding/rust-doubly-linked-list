use std::fmt::Debug;
use std::marker::PhantomData;

struct Memory<T> {
    buffer: Vec<T>,
    free: Vec<usize>,
}

impl<T: Debug> Memory<T> {
    fn new() -> Self {
        Self {
            buffer: Vec::<T>::new(),
            free: Vec::new(),
        }
    }

    fn debug_print(&self) {
        print!("buffer: ");
        for x in self.buffer.iter() {
            print!("* ");
        }
        println!("");

        print!("free: ");
        for x in self.free.iter() {
            print!("{:?} ", x);
        }
        println!("");
    }

    fn alloc(&mut self, value: T) -> usize {
        if let Some(index) = self.free.pop() {
            self.buffer[index] = value;
            index
        } else {
            self.buffer.push(value);
            self.buffer.len() - 1
        }
    }

    fn dealloc(&mut self, ptr: usize) {
        self.free.push(ptr);
    }

    fn deref(&self, ptr: usize) -> &T {
        &self.buffer[ptr]
    }

    fn deref_mut(&mut self, ptr: usize) -> &mut T {
        &mut self.buffer[ptr]
    }
}

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: Option<usize>,
    prev: Option<usize>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value: value,
            next: None,
            prev: None,
        }
    }
}

struct Deque<T> {
    begin: Option<usize>,
    end: Option<usize>,
    phantom: PhantomData<T>,
}

impl<T: Clone + Debug> Deque<T> {
    fn new() -> Self {
        Self {
            begin: None,
            end: None,
            phantom: Default::default(),
        }
    }

    fn debug_print(&self, memory: &Memory<Node<T>>) {
        let mut iter = self.begin;
        print!("deque: |");
        while let Some(node_ref) = iter {
            print!("{:?} ", memory.deref(node_ref).value);
            iter = memory.deref(node_ref).next;
        }
        print!("|");
        println!("");
    }

    fn push_front(&mut self, memory: &mut Memory<Node<T>>, value: T) {
        let new_node = memory.alloc(Node::new(value));
        if let Some(begin_node) = self.begin {
            memory.deref_mut(new_node).next = Some(begin_node);
            memory.deref_mut(begin_node).prev = Some(new_node);
            self.begin = Some(new_node);
        } else {
            assert!(self.end.is_none());
            self.begin = Some(new_node);
            self.end = Some(new_node);
        }
    }

    fn push_back(&mut self, memory: &mut Memory<Node<T>>, value: T) {
        let new_node = memory.alloc(Node::new(value));
        if let Some(end_node) = self.end {
            memory.deref_mut(new_node).prev = Some(end_node);
            memory.deref_mut(end_node).next = Some(new_node);
            self.end = Some(new_node);
        } else {
            assert!(self.begin.is_none());
            self.begin = Some(new_node);
            self.end = Some(new_node);
        }
    }

    fn pop_back(&mut self, memory: &mut Memory<Node<T>>) {
        if let Some(end_ref) = self.end {
            self.end = memory.deref(end_ref).prev;
            memory.dealloc(end_ref);

            if self.end.is_none() {
                self.begin = None;
            }
        }
    }

    fn pop_front(&mut self, memory: &mut Memory<Node<T>>) {
        if let Some(begin_ref) = self.begin {
            self.begin = memory.deref(begin_ref).next;
            memory.dealloc(begin_ref);

            if self.begin.is_none() {
                self.end = None;
            }
        }
    }
}

fn main() {
    let mut memory = Memory::<Node<i32>>::new();
    let mut deque = Deque::<i32>::new();

    for i in 0..10 {
        deque.push_back(&mut memory, i);
    }
    println!("");
    memory.debug_print();
    deque.debug_print(&memory);

    for i in 10..20 {
        deque.push_front(&mut memory, i);
    }
    println!("");
    memory.debug_print();
    deque.debug_print(&memory);

    for _ in 0..20 {
        deque.pop_front(&mut memory);
    }
    println!("");
    memory.debug_print();
    deque.debug_print(&memory);

    for i in 0..10 {
        deque.push_back(&mut memory, i);
    }
    println!("");
    memory.debug_print();
    deque.debug_print(&memory);

    for i in 0..10 {
        deque.push_front(&mut memory, i);
    }
    println!("");
    memory.debug_print();
    deque.debug_print(&memory);

    for i in 0..10 {
        deque.push_front(&mut memory, i);
    }
    println!("");
    memory.debug_print();
    deque.debug_print(&memory);
}
