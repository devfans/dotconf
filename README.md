# dotconf: A super light-weight dotenv library

With less than 20 lines of code of the core part, but meet most of the requirements of a dotenv. JUST KEEP IT SIMPLE!

## Examples

Sample `.env` file:

```
a = hi  # This is a comment
b = -123
c = false
```

```rust
use dotconf::{init, init_with_path};

init().ok() // Ignore the error even if `.env` does not exist.
init().expect("Failed to load env conf file (default: .env)");
init_with_path(".dotenvfile").expect("Failed to load from the specified env conf file");

// Read value with env::var with some simple type conversions
let a = dotconf::var("a").to_string().unwrap(); 
let b = dotconf::var("b").to_isize().unwrap();
let c = dotconf::var("c").to_bool().unwrap();
```
