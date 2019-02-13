// Given a list of integers, use a vector and return
// * the mean (the average value),
// * median (when sorted, the value in the middle position),
// * and mode (the value that occurs most often; a hash map will be helpful here)
// of the list.

use std::collections::HashMap;

fn main() {
    let numbers_list = vec![4,5,76,8,5,34,56,8,98,7,6,4,3,2,4,69,8,54,25,6,1,2,3,4,4,3,43,56,67,5,3,3,76,567,8,0,8,45,43,6,1,0,4,5];
    // let mut numbers_sum = 0;

    // get mean by summing all nummbers
    // for num in &numbers_list {
    //     nummbers_sum += num
    // }

    // there has to be a common sum() func right? yep!

    // made this a func so could reuse in median
    fn get_mean(list: &[i32]) -> f64 {
        let numbers_sum:i32 = list.iter().sum();
        f64::from(numbers_sum) / (list.len() as f64)
    }

    println!("mean is {}", get_mean(&numbers_list) );

    // median time!
    fn get_median(list: &[i32]) -> f64 {
        let len = list.len();
        // gotta sort the numbers_list
        let mut sorted_num_list = list.to_vec();
        sorted_num_list.sort();
        // println!("sort {}", sorted_num_list);
        let half_len = len / 2; // but what if len() is odd number?
        // println!("half len is {}", half_len );

        if len % 2 == 0 {
            get_mean(&sorted_num_list[(half_len - 1)..(half_len + 1)])
        } else {
            f64::from(sorted_num_list[half_len])
        }
    }

    println!("median is {}", get_median(&numbers_list) );

    // a la mode
    fn get_mode(list: &[i32]) -> (i32, i32) {
        let mut occurrences: HashMap<&i32, i32> = HashMap::new();
        let mut max_and_count: (i32, i32) = (0, 0);

        for num in list {
            let count = occurrences.entry(num).or_insert(0);
            *count += 1;
        }

        for (&&k, &v) in &occurrences {
            if v > max_and_count.1 {
                // there is a new max!
                max_and_count = (k, v)
            }
        }
        max_and_count
    }

    let mode = get_mode(&numbers_list);
    println!("mode is {}, it occurs {} times", mode.0, mode.1);
}
