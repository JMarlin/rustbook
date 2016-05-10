fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 'iter()' for vecs yeilds '&i32'. Destructure to 'i32'.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // 'into_iter()' for vecs yeilds 'i32'. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 'iter()' for arrays yeilds '&i32'.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // 'into_iter()' for arrays unusually yeilds '&i32'.
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}
