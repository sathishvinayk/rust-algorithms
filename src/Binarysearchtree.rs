// left reps smaller or eq to its parent
// right reps greather than parent
use std::mem;

#[derive(Clone, Debug)]
pub struct IoTDevice {
    pub numerical_id: u64,
}

impl IoTDevice {
    pub fn new(id: u64) -> IoTDevice {
        IoTDevice {
            numerical_id: id,
        }
    }
}

type Tree = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    pub dev: IoTDevice,
    left: Tree,
    right: Tree,
}

impl Node {
    pub fn new(dev: IoTDevice) -> Tree {
        Some(Box::new(Node {
            dev: dev,
            left: None,
            right: None,
        }))
    }
}

// Tree struct itself is only a pointer to the root node,
#[derive(Debug)]
pub struct DeviceRegistry {
    root: Tree,
    pub length: u64,
}

impl DeviceRegistry {
    pub fn new_empty() -> DeviceRegistry {
        DeviceRegistry {
            root: None,
            length: 0
        }
    }

    pub fn add(&mut self, device: IoTDevice) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }

    fn add_rec(&mut self, node: Tree, device: IoTDevice) -> Tree {
        match node {
            Some(mut n) => {
                println!("==> {:?}", n);
                if n.dev.numerical_id <= device.numerical_id {
                    n.left = self.add_rec(n.left, device);
                } else {
                    n.right = self.add_rec(n.right, device);
                }
                Some(n)
            }
            _ => Node::new(device),
        }
    }

    pub fn find(&self, numerical_id: u64) -> Option<IoTDevice> {
        self.find_r(&self.root, numerical_id)
    }

    fn find_r(&self, node: &Tree, numerical_id: u64) -> Option<IoTDevice> {
        match node {
            Some(n) => {
                if n.dev.numerical_id == numerical_id {
                    Some(n.dev.clone())
                } else if n.dev.numerical_id < numerical_id {
                    self.find_r(&n.left, numerical_id)
                } else {
                    self.find_r(&n.right, numerical_id)
                }
            },
            _ => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&IoTDevice) -> ()) {
        self.walk_in_order(&self.root, &callback);
    }
    
    fn walk_in_order(&self, node: &Tree, callback: &impl Fn(&IoTDevice) -> ()) {
        if let Some(n) = node {
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }
}


fn main() {
    let amber = IoTDevice::new(7);

    let amber2 = IoTDevice::new(6);

    let amber3 = IoTDevice::new(2);

    let mut bsa = DeviceRegistry::new_empty();
    bsa.add(amber);
    bsa.add(amber2);
    bsa.add(amber3);

    let b = bsa.find(2);

    println!("Bsa: -> {:?}", bsa);
    println!("BBB: -> {:?}", b);
}