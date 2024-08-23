# Pseudo Random Number Generator (PRNG)
A PRNG produces rumbers that appear random but are in fact not random.

For this project we use a Linear Congruential Generator (LCG) that uses an equation of the following form: 

Xn+1 = (A x Xn + C) mod M // the n+1 and n are lower 

Here, A, C, and M are constants that you pick for your generator. You start with some initial seed value as X0 and then use each value Xn to generate the next value Xn+1. The LCG doesn't produce extremely random numbers.


