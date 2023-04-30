pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod random {
    use rand::prelude::*;
    pub struct Dice {
        pub max: i32,
    }

    impl Dice {
        pub fn get(&self) -> i32 {
            let mut rng = thread_rng();
            rng.gen_range(1, self.max)
        }

        pub fn fill(&self, data: &mut Vec<i32>) {
            for it in data {
                *it = self.get();
            }
        }
    }
}