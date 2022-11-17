mod solution;

use solution::Polynomial;

fn main() {
    let a = Polynomial::builder()
        .add(3, "x", 3)
        .add(3, "y", 5)
        .add(4, "y", 4)
        .build();
    
    let b = Polynomial::builder()
        .add(2, "x", 1)
        .build();
    
    let c = a + b;
}
