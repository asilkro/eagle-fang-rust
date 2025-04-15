/* This is something I had worked on during a tech screen to do a running average of values as they get provided.
TODO: Revisit and finish
*/

use std;

struct RunningAverage {

    sum: f64,
    count: usize,

}

impl RunningAverage {

    fn new() -> Self {
        RunningAverage { sum: 0.0, count: 0 }
    }

 
    fn add(&mut self, values: &[f64]) -> f64 {

        for &value in values
        {
            self.sum += value;
            self.count += 1;
        }

        self.sum
    } 

    fn count(&mut self, count: usize) {
        self.count = count;
    }

    fn get_average(&self) -> f64 {

        if self.count == 0 {

            0.0
        } else {

            self.sum / self.count as f64
        }
    }
}


#[cfg(test)]
mod tests {

    use std::sync::{Arc, Mutex};
    use super::*;

    #[test]
    fn running_average_works_as_expected()

    {

        //Given
        let result = 2.0;
        let values = [1.0, 2.0, 3.0];

        let mut ra = RunningAverage::new();

        // When
        let sum = ra.add(&values);
        let sut = ra.get_average();

        // Then
        assert_eq!(result, sut);

    }

    #[test]
    fn running_average_is_threadsafe()

    {

        // Given...
        let safe_ra = Arc::new(Mutex::new(RunningAverage::new()));

        let mut thread_handles = vec![];

        let expected_sum: f64 = (0..10).map(|x| x as f64 * 10.0).sum(); // 0-9, each added 10 times
        let expected_count = 10 * 10; // 10 numbers added 10 times
        let expected_average = expected_sum / expected_count as f64; // 4.5

        // When...
        for i in 0..10 {

            let ra_clone = Arc::clone(&safe_ra);

            thread_handles.push(std::thread::spawn(move ||

                {

                    let mut ra = ra_clone.lock().unwrap();

                    ra.add(&[i as f64; 10]);

                }));

            // Would ideally address mutex poisoning / race condition here
        }

        // Then...
            let ra = safe_ra.lock().unwrap();

            println!("Sum is: {}", ra.sum);
            println!("Count is: {}", ra.count);
            println!("Average is: {}", ra.get_average());

            let final_average = ra.get_average();

            // Assert the final average is as expected
            assert_eq!(final_average, expected_average);

        }

    }
