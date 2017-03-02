# PoolTer
This is a very basic Thread Pool library for Rust, mainly designed as a concurrent task running mechanism inspired from JavaScript Promise structure.<br/>
```yaml
[dependencies]
poolter = "*"
```

# Usage
Library itself is very simple and with 0 overhead, but giving a lot of benefits in terms of functional programming and splitting logic into multiple concurrent parts using limited resources.
```rust
extern crate poolter;

use poolter::PoolTer;

.......
let mut pool = PoolTer::init();
```
`init()` function will start threads based on CPU cores count and will start listening callback functions to be running for CPU cores using basic Round Rubin over available threads. This is giving
advantage of splitting load over multiple cores and keep simplicity.

So now we can run functions in thread loop using basic JavaScript Promise like code syntax
```rust
// This will be executing in Thread 1
pool.exec(Box::new(move || {
  println!("This will run first in Thread 1 !");
})).then(Box::new(move || {
  println!("After that we will call this one in Thread 1 !");
}));

// This will be executing in Thread 2
pool.exec(Box::new(move || {
  println!("This will run first in Thread 2 !");
})).then(Box::new(move || {
  println!("After that we will call this one in Thread 2 !");
}));
```

# Contributions are welcome
This is basic library, but if you find some bugs or wrong typed code logic please feel free to make a pull request or open an issue.
