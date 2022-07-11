use std::{cell::RefCell, collections::VecDeque, iter::Peekable, rc::Rc};

enum Next<T> {
    Done(T),
    Nodes(Box<dyn Iterator<Item=T>>)
}

struct Bundle<T> {
    root: T,
    next_: Next<T>,
}

type IterBundle = Peekable<Box<dyn Iterator<Item=Bundle<i32>>>>;

fn neighbors(source: Box<dyn Iterator<Item=i32>>) -> Box<dyn Iterator<Item=(i32, Box<dyn Iterator<Item=i32>>)>> {
    Box::new(source.filter_map(|x| {
        let kids: Vec<_> = [10 * x, 10 * x + 1]
            .iter()
            .copied()
            // NOTE comment out this line to test infinite recursion
            .filter(|&x| x < 1000)
            .collect();
        if kids.is_empty() {
            None
        } else {
            let iter: Box<dyn Iterator<Item=i32>> = Box::new(kids.into_iter());
            Some((x, iter))
        }
    }))
}

struct BundleMonad {
    inner: Box<dyn Iterator<Item=(i32, Box<dyn Iterator<Item=i32>>)>>,
    queue: Rc<RefCell<VecDeque<Bundle<i32>>>>,
}

impl BundleMonad {
    fn bind(from: IterBundle) -> Self {
        let queue = Rc::new(RefCell::new(VecDeque::new()));
        let queue_clone = Rc::clone(&queue);
        let flattened = Box::new(from.flat_map(move |bundle| {
            match bundle.next_ {
                Next::Done(x) => {
                    queue.borrow_mut().push_back(Bundle {
                        root: bundle.root,
                        next_: Next::Done(x),
                    });
                    let iter: Box<dyn Iterator<Item=i32>> = Box::new(vec![].into_iter());
                    iter
                },
                Next::Nodes(nodes) => {
                    let queue_clone = Rc::clone(&queue);
                    let iter: Box<dyn Iterator<Item=i32>> = Box::new(nodes.map(move |node| {
                        queue_clone.borrow_mut().push_back(Bundle {
                            root: bundle.root,
                            next_: Next::Done(node),
                        });
                        node
                    }));
                    iter
                },
            }
        }));
        let processed = neighbors(flattened);
        Self {
            inner: processed,
            queue: queue_clone,
        }
    }
}

impl Iterator for BundleMonad {
    type Item=Bundle<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        // See if queue has items
        if let Some(b) = self.queue.borrow_mut().pop_front() {
            return Some(b)
        }

        // Queue is empty, so generate some elements. We can't return
        // them though, since this also adds to the queue. Those elements
        // need to be returned first. If not, we will infinite-loop on
        // infinite-depth graphs.
        if let Some((root, kids)) = self.inner.next() {

            self.queue.borrow_mut().push_back(Bundle {
                root,
                next_: Next::Nodes(kids),
            });
        }

        // Try reading from the queue again, since pulling
        // from self.inner might have added elements.
        if let Some(b) = self.queue.borrow_mut().pop_front() {
            return Some(b)
        }
        None
    }
}

struct Rec {
    from: Option<IterBundle>,
}

impl Rec {
    fn new(from: IterBundle) -> Self {
        Self {
            from: Some(from),
        }
    }
}

impl Iterator for Rec {
    type Item=i32;

    fn next(&mut self) -> Option<Self::Item> {
        while {
            match self.from.as_mut().expect("a").peek() {
                Some(Bundle {
                    root: _,
                    next_: Next::Nodes(_),
                }) => true,
                _ => false,
            }
        } {
            let iter: Box<dyn Iterator<Item=Bundle<i32>>> = Box::new(
                BundleMonad::bind(self.from.take().expect("b")));
            self.from = Some(iter.peekable());
        }

        self.from.as_mut().expect("c").next().map(|b| {
            match b.next_ {
                Next::Done(x) => x,
                Next::Nodes(_) => panic!("AAA"),
            }
        })
    }
}

fn main() {
    let start: Box<dyn Iterator<Item=Bundle<i32>>> = Box::new([Bundle {
        root: 0,
        next_: Next::Nodes(Box::new([1, 2].into_iter())),
    }].into_iter());
    let start = start.peekable();

    let rec = Rec::new(start);
    let output: Vec<_> = rec.take(4).collect();
    dbg!(output);
}
