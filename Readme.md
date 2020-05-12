# Evil number generator in rust

This application will generate evil numbers up to a given number and outputs the largest evil number and the largest "evil at heart" number found in that range. 
The application will automatically detect number of cores and spawns threads accordingly.

---
Aditionally to the rust compiler, a c++ compiler is also requred, because the application uses [primesieve](https://github.com/kimwalisch/primesieve) library to generate prime numbers.

---
### Build
```
git clone --depth=1 --recurse-submodules https://github.com/chamatht/evil-numbers-rs

cd evil-numbers-rs

cargo build --release
```

---
### Run 
```
cargo run --release 10000
```
```
USAGE:
  evil-numbers-rs STOP_VALUE [START_VALUE]
```
Here the START_VALUE is optional.

---
### Evil criteria
- A number is evil if the number is a prime number and its sum of digits is divisible by 7.
- An evil number is "evil at heart", if its position in the vector of evil numbers is also an evil number.
