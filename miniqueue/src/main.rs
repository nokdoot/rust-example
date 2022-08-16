#[macro_use]
extern crate queues;

use queues::*;

fn main() {
    // Create a simple Queue
    let mut q: Queue<isize> = queue![];

    // Add some elements to it
    q.add(1);
    q.add(-2);
    q.add(3);

    // Check the Queue's size
    q.size();  // 3

    // Remove an element
    q.remove();  // Ok(1)

    // Check the Queue's size
    q.size();  // 2

    // Peek at the next element scheduled for removal
    q.peek();  // Ok(-2)

    // Confirm that the Queue size hasn't changed
    q.size();  // 2

    // Remove the remaining elements
    q.remove();  // Ok(-2)
    q.remove();  // Ok(3)

    // Peek into an empty Queue
    q.peek();  // Raises an error

    // Attempt to remove an element from an empty Queue
    q.remove();  // Raises an error
}
