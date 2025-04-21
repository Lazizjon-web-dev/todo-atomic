use std::thread;
use todo_atomic::core::TodoList;
#[test]
fn hammer_add_remove() {
    let todo = TodoList::default();
    let handles: Vec<_> = (0..100).map(|_| {
        thread::spawn(|| {
            let id = todo.add("task".into());
            assert!(todo.remove(id).is_ok());
        })
    }).collect();
    // No deadlocks/race conditions
}