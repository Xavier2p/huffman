mod heap;

fn main() {
    println!("Hello, world!");
    push();
}

fn push() {
    let mut heap: heap::Heap = heap::Heap::new();
    println!("{:?}", heap.nodes);
    let heap = heap.push(heap::Node::new(1, 'a'));
    println!("{:?}", heap.nodes);
}
