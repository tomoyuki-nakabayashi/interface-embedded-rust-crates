use heapless::spsc::Queue;

fn main() {
    let mut rb: Queue<u8, 4> = Queue::new();

    assert!(rb.enqueue(0).is_ok());
    assert!(rb.enqueue(1).is_ok());
    assert!(rb.enqueue(2).is_ok());
    assert!(rb.enqueue(3).is_err()); // full

    assert_eq!(rb.dequeue(), Some(0));
    assert_eq!(rb.dequeue(), Some(1));
    assert_eq!(rb.dequeue(), Some(2));
    assert_eq!(rb.dequeue(), None);
}