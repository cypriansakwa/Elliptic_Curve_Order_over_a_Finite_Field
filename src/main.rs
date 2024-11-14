#[derive(Debug, Clone, Copy)]
struct Point {
    x: Option<i64>,
    y: Option<i64>,
}
impl Point {
    fn new(x: Option<i64>, y: Option<i64>) -> Point {
        Point { x, y }
    }

    // Check if the point is at infinity
    fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}
fn main() {
    let a = 13; // coefficient a
    let b = 2; // coefficient b
    let p = 17; // prime modulus

    let mut count = 0;

    // Iterate over all possible x-coordinates
    for x in 0..p {
        let x_cubed = (x * x * x) % p;
        let ax = (a * x) % p;
        let rhs = (x_cubed + ax + b) % p;

        // Iterate over all possible y-coordinates
        for y in 0..p {
            let y_squared = (y * y) % p;

            // Check if the point satisfies the elliptic curve equation
            if y_squared == rhs {
                println!("Point ({}, {})", x, y);
                count += 1;
            }
        }
    }

    // Account for the point at infinity
    count += 1;

    println!("Elliptic curve is of order: {}", count);
    
}
