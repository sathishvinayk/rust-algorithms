// This is for purely educational purpose
// RefCell and Rc were used to allocating space on heap and 
// therfore allow for larger lists
use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    value: String,
    next: SingleLink,
}

#[derive(Debug)]
struct TranscationLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub length: u64
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

impl TranscationLog {
    pub fn new_empty() -> TranscationLog {
        TranscationLog {
            head: None,
            tail: None,
            length: 0
        }
    }
    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take(){
            Some(old) => {
                println!("Old ==> {:?}", old);
                old.borrow_mut().next = Some(new.clone())
            },
            None => {
                println!("None ==> ");
                self.head = Some(new.clone())
            }
        }
        self.length += 1;
        self.tail = Some(new);
    }
    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is wrong")
                .into_inner()
                .value
        })
    }
}


fn main() {
    let mut a = TranscationLog::new_empty();
    a.append("Value 1".to_string());
    a.append("Value 2".to_string());
    a.append("Value 3".to_string());

    println!("Head{:?} ", a.head);
    println!("Tail{:?} ", a.tail);
    println!("Length{:?} ", a.length);

    println!("");println!("");println!("");
    let pop_value = a.pop();

    println!("Head{:?} ", a.head);
    println!("Tail{:?} ", a.tail);
    println!("Length{:?} ", a.length);

    println!("");println!("");println!("");

    println!("{:?}", pop_value);
}