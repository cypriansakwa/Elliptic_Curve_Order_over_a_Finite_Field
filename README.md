# Elliptic_Curve_Order_over_a_Finite_Field
## Overview
- This Rust program computes the order of an elliptic curve defined over a finite field, by counting all valid points that satisfy the curve equation, including the point at infinity. The program uses a brute-force approach to find all solutions 
$(x,y)$ to the curve equation:
$y^2=x^3+ax+b \mod p$
## How It Works
- Elliptic Curve Equation: The program solves the elliptic curve equation $y^2=x^3+ax+b \mod p$.
- Iterating Over Possible Points: It iterates over all integer values of $x$ and $y$ in the range $[0,p-1]$. For each pair, it checks if they satisfy the equation modulo $p$.
- Counting the Points: The program counts all solutions $(x,y)$ and includes the point at infinity to determine the order of the elliptic curve.
## Usage
- To get started, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine. You can then clone the repository and build the project:
- Clone the repository:
```
git clone https://github.com/cypriansakwa/Elliptic_Curve_Order_over_a_Finite_Field.git
cd Elliptic_Curve_Order_over_a_Finite_Field
```
- Compile and run the program:
```
cargo run
```
## Example 
For $y^2=x^3+ax+b \mod p$ where $a=13, b=2$ and $p=17$, it outputs
```
Point (0, 6)
Point (0, 11)
Point (1, 4)
Point (1, 13)
Point (2, 6)
Point (2, 11)
Point (3, 0)
Point (4, 4)
Point (4, 13)
Point (9, 7)
Point (9, 10)
Point (12, 4)
Point (12, 13)
Point (14, 2)
Point (14, 15)
Point (15, 6)
Point (15, 11)
Elliptic curve is of order: 18
```
## Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request if you would like to improve this project.
