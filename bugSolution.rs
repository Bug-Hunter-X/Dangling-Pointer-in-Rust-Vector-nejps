fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using a raw pointer, work directly with the vector.
    v[0] = 10; 
    println!("{:?}", v);
    // Or use a safe, indexed approach if direct manipulation is needed:
    // v.iter_mut().for_each(|x| *x *= 2); //doubles the values
    // println!("{:?}", v);
} 