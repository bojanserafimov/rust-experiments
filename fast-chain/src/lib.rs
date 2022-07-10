use std::collections::LinkedList;


struct FastChain<T> {
    iters: LinkedList<Box<dyn Iterator<Item=T>>>
}

impl<T> FastChain<T> {
    fn new() -> Self {
        Self {
            iters: LinkedList::new()
        }
    }

    fn prepend(&mut self, it: Box<dyn Iterator<Item=T>>) {
        self.iters.push_front(it);
    }
}

impl<T> Iterator for FastChain<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(first) = self.iters.front_mut() {
            match first.next() {
                Some(x) => return Some(x),
                None => self.iters.pop_front(),
            };
        }
        None
    }
}




#[cfg(test)]
mod tests {
    use std::iter::Peekable;

    use measure_time::print_time;
    use super::*;

    #[test]
    fn bench_chain() {
        let n = 10000;
        {
            print_time!("chain");
            let mut iter: Box<dyn Iterator<Item=i32>> = Box::new(0..0);
            for _ in 0..n {
                iter = Box::new((0..20).chain(iter));
            }
            let all: Vec<_> = iter.collect();
        }
        {
            print_time!("fast chain");
            let mut iter = FastChain::<i32>::new();
            for _ in 0..n {
                iter.prepend(Box::new(0..20));
            }
            let all: Vec<_> = iter.collect();
        }
    }
}
