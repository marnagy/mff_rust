use std::collections::HashMap;
use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Polynomial {
    variables: HashMap<String, HashMap<i32, i32>>
}

impl Polynomial {
    pub fn builder() -> PolynomialBuilder {
        PolynomialBuilder::new()
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // TODO: continue here
    }
}

pub struct PolynomialBuilder {
    variables: HashMap<String, HashMap<i32, i32>>
}

impl PolynomialBuilder {
    pub fn new() -> Self {
        PolynomialBuilder { variables: HashMap::new() }
    }
    pub fn add(mut self, quotient: i32, variable_name: &str, power: i32) -> Self {
        if !self.variables.contains_key(variable_name) {
            let mut new_hm = HashMap::new();
            new_hm.insert(power, quotient);
            self.variables.insert(variable_name.to_string(), new_hm);
        }
        else {
            // builder contains variable name
            let sub_dict = self.variables.get_mut(variable_name).unwrap();
            match sub_dict.get(&power) {
                Some(_) => panic!("Quotient for variable {} and power {} is already set.", variable_name, power),
                None => sub_dict.insert(power, quotient)
            };
        }
        
        return self;
    }
    pub fn build(self) -> Polynomial {
        Polynomial { variables: self.variables }
    }
} 

struct Variable {
    power: i32,
    quotient: i32
}