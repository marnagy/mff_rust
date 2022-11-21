mod solution;

use solution::Polynomial;

fn main() {
    let _a = Polynomial::builder()
        .add(3, "x", 3)
        .add(3, "y", 5)
        .add(4, "y", 4)
        .build();

    let b = Polynomial::builder().add(2, "x", 1).add(-1, "x", 1).build();

    println!("Polynomial b: {:?}", b);

    let c = Polynomial::builder().add(1, "x", 1).build();

    println!("Is equal? {}", b == c);

    println!("b + c = {:?}", b + c);
}
