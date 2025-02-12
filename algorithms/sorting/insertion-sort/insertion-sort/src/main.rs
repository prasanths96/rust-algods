use rand;

// const INPUT: [i32; 6] = [5,3,2,7,5,4];
const N: i32 = 10;

fn main() {

    // let mut input_vec: [i32; 8] = rand::random();
    // let mut input_vec: Vec<i32> = vec![];
    let mut input_vec: Vec<i32> = Vec::new();

    for _i in 0..N {
        input_vec.push(rand::random_range(-100..100));
    }


    // let mut input_vec: Vec<i32> = INPUT.to_vec();
    // println!("Hello, world! {:?}", input_vec);

    println!("Input array: {:?}", input_vec);

    // Insertion sort
    for i in 1..input_vec.len() {
        let key: i32 = input_vec[i];

        let mut j: i32 = (i as i32) - 1;

        while j > -1 && input_vec[j as usize] > key {
            input_vec[(j+1) as usize] = input_vec[j as usize];
            j -= 1;
        }

        input_vec[(j+1) as usize] = key;
    }


    println!("Result: {:?}", input_vec);
}
