// Vecs act sort of like arrays, but allow more flexibility
#[test]
fn making_an_empty_vec() {
    let vector: Vec<()> = vec![];
    assert!(vector.len() == 0);
}

// The vec! macro makes it easier to instantiate a vec
#[test]
fn vec_macro() {
    let vector = vec![1, 2, 3, 4];
    assert!(vector.len() == 4);
}

// Elements can be added to mutable Vecs
#[test]
fn adding_to_vecs() {
    let mut vector = vec![0, 1, 2];
    vector.push(3);
    assert!(vector.len() == 4);
}

// Elements can also be removed from mutable Vecs
#[test]
fn removing_from_end_of_vecs() {
    let mut vector = vec![0, 1, 2, 3];
    vector.pop();
    assert!(vector.len() == 3);
}

// The elements removed can also be stored to another variable
#[test]
fn storing_vec_elements() {
    let mut vector = vec![0, 1, 2, 3];
    let num = vector.pop().unwrap();
    assert!(num == 3);
}

// Vecs can change size to fit their contents
// Vecs do still need to be declared as mutable in order to change
#[test]
fn changing_size_of_vecs() {
    let mut vector = vec![1, 2, 3];
    vector.push(4);
    assert!(vector.len() == 4);
    vector.pop();
    assert!(vector.len() == 3);
}

// Vecs have a certain maximum capacity at any given point
// When this capacity is reached, they will allocate more memory
#[test]
fn capacity() {
    let mut vector = vec![1, 2, 3, 4];
    assert_eq!(vector.capacity(), 4);
    vector.extend(vec![5, 6, 7, 8]);
    assert_eq!(vector.capacity(), 8);
    vector.pop();
    assert_eq!(vector.capacity(), 8);
}

// This extra memory can also be deallocated when its no longer needed
#[test]
fn shrink_vecs() {
    let mut vector = vec![1, 2, 3, 4, 5];
    assert_eq!(vector.capacity(), 5);
    vector.pop();
    assert_eq!(vector.capacity(), 5);
    vector.shrink_to_fit();
    assert_eq!(vector.capacity(), 4);
}

// Vecs can reserve more space in order to prevent allocating several times
#[test]
fn reserve() {
    let mut vector = vec![1];
    vector.reserve(7);
    assert_eq!(vector.capacity(), 8);
}

// You can also cut Vecs down to size
#[test]
fn truncate() {
    let mut vector = vec![1, 2, 3, 4, 5];
    vector.truncate(2);
    assert_eq!(vector, vec![1, 2]);
}

// New elements can be stuffed into mutable Vectors
#[test]
fn insert() {
    let mut vector = vec![1, 2, 3, 4, 5];
    vector.insert(2, 6);
    assert_eq!(vector, vec![1, 2, 6, 3, 4, 5]);
}

// Elements can also be deleted a particular position in a Vector
#[test]
fn remove() {
    let mut vector = vec![1, 2, 3, 4, 5];
    vector.remove(0);
    assert_eq!(vector, vec![2, 3, 4, 5])
}

// We created an empty Vec in our first example,
// now let's check if a Vec is empty
#[test]
fn empty_vecs() {
    let mut vector = vec![""; 0];
    assert!(vector.is_empty()); // should return true
    vector.push("1");
    assert!(!vector.is_empty()); // should return false
}

// Elements of a Vec can be accessed by their index.
// note: Vecs in Rust are zero-indexed
#[test]
fn vec_indices() {
    let vector = vec!["red", "green", "refactor"];
    assert_eq!(Some(&"green"), vector.get(1));
}

// You can also easily grab a Vec's first and last elements using the respective method
#[test]
fn first_and_last() {
    let vector = vec![false, true];
    assert_eq!(vector.first(), Some(&false));
    assert_eq!(vector.last(), Some(&true));
}

// It's also easy to check if a Vec contains a particular value
#[test]
fn contains_element() {
    let vector = vec!["Google", "Twitter", "Mozilla"];
    assert!(vector.contains(&"Twitter"));
    assert!(!vector.contains(&"Mozillaz"));
}

// Similar to contains(), you can also check if a Vec begins with a particular element
#[test]
fn starts_with() {
    let vector = vec![0, 2, 4, 6];
    assert!(vector.starts_with(&[0]));
}

// starts_with() can also accept multiple elements
#[test]
fn starts_with_2() {
    let vector = vec![0, 2, 4, 6];
    assert!(vector.starts_with(&[0, 2]));
}

// The same can be said for ends_with
#[test]
fn ends_with() {
    let vector = vec![2, 4, 6];
    assert!(vector.ends_with(&[6]));
    assert!(vector.ends_with(&[2, 4, 6]));
}

// Reversing a Vec is pretty easy in Rust
#[test]
fn reverse_vecs() {
    let mut vector = vec![1, 2, 3];
    vector.reverse();
    assert_eq!(vector.first(), Some(&3));
}

// You can also just swap two elements in a Vec
#[test]
fn trading_spaces() {
    let mut vector = vec![false, true];
    vector.swap(0, 1);
    assert_eq!(vector.first(), Some(&true));
}

// Vecs can be broken up into equally sized chunks
#[test]
fn chunking() {
    let vector = vec![1, 2, 1, 2];
    for chunk in vector.chunks(2) {
        assert_eq!(chunk, &[1, 2]);
    }
}

// Vecs can be split at a specified index
#[test]
fn splitting() {
    let vector = vec!["Ruby", "Rust", "Python", "C++"];
    let (langs1, langs2) = vector.split_at(2);
    assert_eq!(langs1, &["Ruby", "Rust"]);
    assert_eq!(langs2, &["Python", "C++"]);
}

// Or if you don't know the specific index, you can supply a condition at which to split
// The new groups will not include the elements that match the condition
#[test]
fn more_splitting() {
    let vector = vec![1, 3, 4, 7, 9];
    for num in vector.split(|&x| x == 4) {
        assert!(!num.contains(&4));
    }
}
