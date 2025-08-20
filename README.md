# debag-utilities

Rust procedural macros for simplified debugging.

## Features

* `#[Trace]` - An attribute for showing the input and output of a function, including the received parameters and the result of the function;
* `dbg_here!(variable)` - Macro for debugging the contents of a variable.

## Usage

First, you need to add the library to your project:

```cargo
cargo add debag_utilities
```

Then, you need to add a dependency to highlight debug messages in the terminal:

```cargo
cargo add colored
```

### `#[trace]`

Adds a debug output to the beginning and end of the function, showing the function entry (along with the input parameters) and exit (with the returned result, if any).

```rust
use debag_utilities::*;

#[trace]
fn get_name(id: i32) -> String {
    if id == 0 {
        println!("Not valid id");
        "Invalid id".to_string()
    } else {
        println!("Valid id");
        "Some Name".to_string()
    }
}

fn main() {
    let name = get_name(1);
    println!("{}", name);
}


/* output:

--> Entering function: 'get_name'
    Param 'id' = 1

Valid id

<-- Exiting function: 'get_name' with result: "Some Name"

Some Name
*/
```

### `dbg_here!(variable)`

Displays the value of the specified variable in the debug message.

```rust
use debag_utilities::*;

fn main() {
    let a = 42;
    dbg_here!(a);
}

/* output:
[example\src\main.rs:5] in example: 'a' = 42
*/
```
