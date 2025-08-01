use std::thread;
use std::time::Duration;
fn main() {
    let simulated_user_specified_value = 5;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

    using_other_iterator_trait_methods();

    let v = vec![0];
    for i in v.iter().skip(1) {
        println!("{}", i);
    }
    let u = vec![2];
    let z = v.iter().zip(u.iter().skip(1));
    for (i, j) in z {
        println!("i: {}, j: {}", i, j);
    }
}
fn generate_workout(intensity: u32, random_number: u32) {
    // let mut expensive_result = Cacher::new(|x| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    // x});
    // if intensity < 25 {
    //     println!("Today, do {} pushups!", expensive_result.value(intensity));
    //     println!("Next, do {} situps!", expensive_result.value(intensity));
    // } else if intensity < 50 {
    //     if random_number == 3 {
    //         println!("Take a break today! Remember to stay hydrated!");
    //     } else {
    //         println!("Run for {} minutes!", expensive_result.value(intensity));
    //     }
    // } else {
    //     println!("You are too intense!");
    // }

    // Iterators
    let shoe = vec![
        Shoe {
            size: 9,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 9,
            style: String::from("boot"),
        },
    ];
    let my_shoes = my_shoes(shoe, 9);
    assert_eq!(
        my_shoes,
        vec!(
            Shoe {
                size: 9,
                style: String::from("sneaker")
            },
            Shoe {
                size: 9,
                style: String::from("boot")
            }
        )
    );
}
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
fn my_shoes(s: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    s.into_iter().filter(|s| s.size == size).collect()
}
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculate: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculate: T) -> Cacher<T> {
        Cacher {
            calculate,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculate)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
