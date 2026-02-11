 # When to choose what template

 - use ```cp.rs ``` when input test cases around <= 10e4 
 its basically BufRead + println! , reads line-by-line 

- use ```tmp2.rs1``` when input test cases constrain >= 10e5 or 10e6 , as bufreader works best for large inputs ..
this ver badsically uses Scanner + BufWriter,
reads all stdin once


## Comparing Floats

floats are messy , they tend to be less precise , 0.99999 is sometimes NOT 1 , whereas problems require 1 .

a better way to compare floats is chceking approximity

```rust
if (num::abs(a-b) < 1e-9){
    //a and b are aproximately equal
}
```
suppose 
``` 
x = 0.3 * 3.0 + 0.1 
```
this should give (or atleast expected value) is 1.
but due the rounding errors we get something like 0.99999999
which in turn may get you a ```WA```

A fix is to convert floats to integers before calc and then print out the result after dividing.
like , 
``` 
x = 3 * 3 +1 
```
then print out ``` x/10 ```

### Caveats

- If inputs are arbitrary floats (like results of sqrt(2)), scaling to integers won’t help; you must use epsilon comparison.

- Integer scaling works best in competitive programming problems where input is specified as decimals with ≤k digits after the decimal point.

in summary : For fixed decimal inputs → scale to integers, do all calculations in integers, then divide back.

---
## trimming 
### macros

we can define a macro which can be used in your program in one line (think of it as calling a function)
```rust
macro_rules! fctr{
    ($n:expr) => {{
        fn fact(mut n: u64) -> u64 {
            let mut res = 1;
            while n > 1 {
                res *= n;
                n -= 1;
            }
            res
        }
        fact($n)
    }};
}
```

so in your code just call fctr(n) 

## nCr, nPr and Factorials  

when dealing with combinatorics (choose / arrange problems), you often need factorials.  
but factorials grow too fast → `20!` already overflows u64.  
so in CP we always calculate them modulo a big prime like `1_000_000_007` or `998_244_353`.  

### precomputation  

instead of recomputing factorials and inverses every time, we precompute them once in O(n).  
this gives:  
- `fact[i] = i! % MOD`  
- `inv_fact[i] = (i!)^-1 % MOD`  

after that, each query of nCr / nPr is O(1).  

```rust
fn ncr(n: usize, r: usize, fact: &Vec<u64>, inv_fact: &Vec<u64>) -> u64 {
    if r > n { return 0; }
    fact[n] * inv_fact[r] % MOD * inv_fact[n - r] % MOD
}

fn npr(n: usize, r: usize, fact: &Vec<u64>, inv_fact: &Vec<u64>) -> u64 {
    if r > n { return 0; }
    fact[n] * inv_fact[n - r] % MOD
}
```

## Modular Power  +Inverse
largest value that we can store in Rust with numerical datatype is 2^128 , i.e using u128/i128
hence for finding 2^1000 modulo MOD will lead to an overflow hence we perform modulo exponentiation using divide and conquer algo O(log(exp)).

needed for computing inverse factorials:
```rust
fn modexp(mut base: usize, mut exp: usize, m: usize) -> usize { ... }
fn modinv(x: u64) -> u64 { modexp(x, MOD-2, MOD) }
```

## NT properties(ctm)
- (a + b) mod m = ((a mod m) + (b mod m)) mod m
- (a x b) mod m = ((a mod m) x (b mod m)) mod m

## Integer Square Root
if you have to find a floor of square root of. say, 10^18 , using the 
```rust 
sqrt(n)
```
function and then taking its floor will give inaccurate results (because of f64)
hence we use a "trick" , the common algo as follows 
``` rust
let mut i = 0;
if n <= 1 {
    return n;
}
while (i+1)*(i+1) <= n{
    i+=1;
    
}
return i;
```
The above algo is has Time complexity of O(sqrt(n)), which can improved to O(log(n)) using simple Binary search implementation.

### Algorithm 
- If the number is 0 or 1, return the number itself.
- low = 1 and high = 1.1 × 10⁹.
- midpoint: mid = low + (high - low)/2.
- If mid × mid equals the number, return mid.
- If mid × mid is less than the number, store mid as the current answer and set low = mid + 1.
- Otherwise, set high = mid − 1.
- Repeat steps 3 to 6 until low exceeds high.
- When the loop ends, return the last stored value of mid.

## Listing Factors 
sometimes we have to list factors of a number, one efficient approach i can think of is looking it as pairs.
Because factors come in pairs one factor in each pair must be less than or equal to √n it is impossible for two numbers both greater than √n to multiply together and give n.
therefore, to find all factors of a number,we only need to check numbers from 1 to √n.
For every factor found in this range, we can compute its paired factor using n / i.
This reduces the time complexity significantly compared to checking all numbers up to n. i.e from O(n) to O(sqrt(n)).

## Prime numbers
We loop through all numbers from 2 up to the square root of the given number and check if any of them divide it evenly. If a divisor is found, we return false. If no divisors are found, we return true, since the number is not divisible by any number in that range.
why this works?

Suppose a number N is not prime. Then it can be expressed as:
` N = A x B`
where `1<A≤B<N` Notice that `A` must be less than or equal to `sqrt(N)`; otherwise, `AxB` will exceed `N` since `AxA>N` and `A<=B` `=>` `AxB>N`. 
Therefore, it is sufficient to check for divisors only up to `sqrt(N)`

```rust
fn prime(n:usize) -> bool{
    let mut i:usize = 2;
    while i*i<=n {
        if n%i == 0 {
            return false;
        }
        i+=1;
    }
    return true;
}
```

### Sieve of Eratosthenes
The Sieve of Eratosthenes is an efficient algorithm to find all primes up to a specified integer N.
with Time Complexity O(log(log(n))).


#### Rust Implementation
```rust
fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::new();
    for p in 2..=n {
        if is_prime[p] {
            primes.push(p);
            // Start marking multiples from p * p
            // because smaller multiples would have already been marked
            let mut i = p * p;
            while i <= n {
                is_prime[i] = false;
                i += p;
            }
        }
    }
    primes
}
```

### prime factorization
#### Algorithm (O(sqrt(n)))
1. **Handle 2 Separately:** We check for divisibility by 2 first. This allows us to skip all even numbers in the main loop, effectively doubling the speed.
2. **Trial Division:** Starting from $i = 3$, we check every odd number up to $\sqrt{N}$. 
3. **Reduction:** Whenever a factor $i$ is found, we divide the number by $i$ repeatedly until it is no longer divisible, counting the occurrences.
4. **Final Prime:** If after the loop the number is still greater than 1, the remaining value must be a prime number.
