fn main() {
    // construct mutable array
    let mut array = [1, 2, 3];
    println!("array = {:?}", array);
    println!("array = {:#?}", array);

    // access array element
    println!(
        "2nd element in array: {}",
        array[1]
    );

    // mutate array element value
    array[1] = 20;
    println!(
        "2nd element is now: {}",
        array[1]
    );

    // find # of elements in array
    println!(
        "array has {} elements",
        array.len()
    );

    // sum array elements
    let total: i32 = array.iter().sum();
    println!(
        "array sum is {}",
        total
    );

    // sort array elements
    // in descending index order
    array.reverse();
    println!(
        "descending index sort: {:?}",
        array
    );

    // sort array elements
    // in ascending value order
    array.sort();
    println!(
        "ascending value sort: {:?}",
        array
    );

    // sort array elements
    // in descending index order
    array.reverse();
    println!(
        "descending value sort: {:?}",
        array
    );

    // construct array of arrays
    let array_of_arrays = [
        [1, 2, 3],
        [4, 5, 6]
    ];
    println!(
        "array_of_arrays: {:#?}",
        array_of_arrays
    );

    // access array_of_arrays elements
    println!(
        "array 1, element 2: {}",
        array_of_arrays[0][1]
    );
    println!(
        "array 2, element 3: {}",
        array_of_arrays[1][2]
    );

}
