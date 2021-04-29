fn main() {
    let v1 = vec![1, 2, 3];
    iterate(v1.clone());
    do_loop(v1.clone());
    do_loop_ref(v1.clone());
    while_loop(v1.clone());
    let clone = v1.clone();
    let v1_iter = clone.iter();
    let total: i32 = v1_iter.sum();
    println!("Sum: {}", total);
    do_map(v1.clone());
    iterate(v1);
}

fn iterate(vec: Vec<i32>) {
    println!("iterating");
    // ! cannot do iter_mut since vec is not set as mutable
    vec.iter().for_each(|v| println!("{}", v)); // * iter() DOES NOT take ownership
    vec.into_iter().for_each(|v| println!("{}", v)); // * into_iter takes ownership
}

fn iterate_mut(mut vec: Vec<i32>) {
    vec.iter().for_each(|v| println!("{}", v)); // * iter() DOES NOT take ownership
    vec.iter_mut().for_each(|v| println!("{}", v)); // * iter_mut borrows it, mutates it and gives it back, however the vector must be declared as mut
    vec.into_iter().for_each(|v| println!("{}", v)); // * into_iter takes ownership
}

fn do_loop(vec: Vec<i32>) {
    println!("for loop");
    // for loop takes owenership because it automatically converts it into_iter
    for v in vec {
        println!("{}", v);
    }
}

fn do_loop_ref(vec: Vec<i32>) {
    println!("for loop ref");
    let v_iter = vec.iter();
    // ? for loop BUT we as a ref only?
    for v in v_iter {
        println!("{}", v);
    }
}

fn while_loop(vec: Vec<i32>) {
    println!("while loop");
    let mut v_iter = vec.iter(); // needs to be mutable for next to work
    loop {
        match v_iter.next() {
            Some(v) => println!("{}", v),
            None => break,
        };
    }
}

fn do_map(vec: Vec<i32>) {
    println!("map");
    let mut v_iter = vec.iter(); //
    let mapped: Vec<i32> = vec.iter().map(|x| x + 1).collect();

    println!("{:?}", mapped);
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum(); // * SUM takes ownership (consumes) the iterator

        assert_eq!(total, 6);
    }
}
