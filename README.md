# Elliptic_Curve_Order_over_a_Finite_Field
## Overview
- This Rust program computes the order of an elliptic curve defined over a finite field, by counting all valid points that satisfy the curve equation, including the point at infinity. The program uses a brute-force approach to find all solutions 
$(x,y)$ to the curve equation:
$y^2=x^3+ax+b \mod p$
## How It Works
- Elliptic Curve Equation: The program solves the elliptic curve equation $y^2=x^3+ax+b \mod p$.
- Iterating Over Possible Points: It iterates over all integer values of $x$ and $$y in the range $[0,p-1]$. For each pair, it checks if they satisfy the equation modulo $p$.
- Counting the Points: The program counts all solutions $(x,y)$ and includes the point at infinity to determine the order of the elliptic curve.
