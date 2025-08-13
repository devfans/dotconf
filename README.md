# envconf-rs: A super light-weight dotenv library

## Examples

Sample `.env` file:

```
a = hi
b = -123
c = false
```

```rust
use envconf::{init, init_with_path};

init().expect("Failed to load env conf file (default: .env)");
init_with_path(".dotenvfile").expect("Failed to load from the specified env conf file");

let a = envconf::var("a").to_string().unwrap();
let b = envconf::var("b").to_isize().unwrap();
let c = envconf::var("c").to_bool().unwrap();
```
