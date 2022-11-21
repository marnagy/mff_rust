use std::collections::HashMap;
use std::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    // <variable_name>, <power, quotient>
    variables: HashMap<String, HashMap<i32, i32>>,
}

impl Polynomial {
    pub fn builder() -> PolynomialBuilder {
        PolynomialBuilder::new()
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let keys1: Vec<String> = self.variables.keys().cloned().collect();
        let keys2: Vec<String> = other.variables.keys().cloned().collect();
        let mut builder = Polynomial::builder();

        let keys_only_in1: Vec<&String> = keys1.iter().filter(|k| !keys2.contains(*k)).collect();
        for key in &keys_only_in1 {
            let sub_dict = self.variables.get(*key).unwrap();
            for power in sub_dict.keys() {
                builder = builder.add(*sub_dict.get(power).unwrap(), key, *power);
            }
        }

        let keys_only_in2: Vec<&String> = keys2.iter().filter(|k| !keys1.contains(*k)).collect();
        for key in &keys_only_in2 {
            let sub_dict = self.variables.get(*key).unwrap();
            for power in sub_dict.keys() {
                builder = builder.add(*sub_dict.get(power).unwrap(), key, *power);
            }
        }

        let other_keys: Vec<&String> = keys1
            .iter()
            .filter(|k| !keys_only_in1.contains(k))
            .collect();
        // combine power and quotients for common variable
        for key in other_keys {
            let powers1: Vec<i32> = self.variables.get(key).unwrap().keys().copied().collect();

            let powers2: Vec<i32> = other.variables.get(key).unwrap().keys().copied().collect();

            let powers_only1: Vec<i32> = powers1
                .iter()
                .copied()
                .filter(|p| !powers2.contains(p))
                .collect();
            for power in powers_only1 {
                builder = builder.add(
                    *self.variables.get(key).unwrap().get(&power).unwrap(),
                    key,
                    power,
                );
            }
            let powers_only2: Vec<i32> = powers2
                .iter()
                .copied()
                .filter(|p| !powers1.contains(p))
                .collect();

            for power in powers_only2 {
                builder = builder.add(
                    *other.variables.get(key).unwrap().get(&power).unwrap(),
                    key,
                    power,
                );
            }

            let combined_powers: Vec<i32> = powers1
                .into_iter()
                .filter(|p| powers2.contains(p))
                .collect();

            for power in combined_powers {
                let quotient1 = self.variables.get(key).unwrap().get(&power).unwrap();
                let quotient2 = other.variables.get(key).unwrap().get(&power).unwrap();
                let result_quotient = quotient1 + quotient2;
                builder = builder.add(result_quotient, key, power);
            }
        }

        builder.clean();

        builder.build()
    }
}

pub struct PolynomialBuilder {
    variables: HashMap<String, HashMap<i32, i32>>,
}

impl PolynomialBuilder {
    pub fn new() -> Self {
        PolynomialBuilder {
            variables: HashMap::new(),
        }
    }
    fn clean(&mut self) {
        let keys: Vec<String> = self.variables.keys().cloned().collect();
        for key in keys {
            let sub_dict = self.variables.get_mut(&key).unwrap();
            let sub_dict_keys: Vec<i32> = sub_dict.keys().copied().collect();
            for power in sub_dict_keys {
                let quotient = sub_dict.get(&power).unwrap();
                if *quotient == 0 {
                    sub_dict.remove(&power);
                }
            }

            if sub_dict.is_empty() {
                self.variables.remove(&key);
            }
        }
    }
    pub fn add(mut self, quotient: i32, variable_name: &str, power: i32) -> Self {
        if !self.variables.contains_key(variable_name) {
            let mut new_hm = HashMap::new();
            new_hm.insert(power, quotient);
            self.variables.insert(variable_name.to_string(), new_hm);
        } else {
            // builder contains variable name
            let sub_dict = self.variables.get_mut(variable_name).unwrap();
            let new_quotient = match sub_dict.get(&power) {
                Some(prev_quotient) => prev_quotient + quotient,
                None => quotient,
            };
            sub_dict.insert(power, new_quotient);
        }

        self.clean();

        self
    }
    pub fn build(mut self) -> Polynomial {
        self.clean();
        Polynomial {
            variables: self.variables,
        }
    }
}
