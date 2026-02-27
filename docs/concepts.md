## Rust Language

### Basic data types


### Memory management

Stack vs Heap

Stack: Values with a known, fixed size at compile time are stored on the stack, very fast but limited.

Heap: Values with dynamic sizes, like Strings and Vectors are stored on the heap, and the stack holds a pointer to the heap data. Takes time to allocate but can hold data that grows or lives beyond the scope.

#### Ownership

Three main rules are the basis:

1. Each value has one owner.
2. There can be only one owner per value at a time
3. If the owner of the value goes out of scope, the value is dropped. (freed from memory)

And assignment is per default (on complex data structures), a move and not a copy.

An example:


```rust
{

let s = String::from("Hello"); // s is the owner of the string
let s2 = s; // s2 now is the owner, s is invalid

}
// Owner went out of scope, the value is automatically dropped.

```


However, for the simple types that live on the stack, copying is pretty cheap so:


```rust

let a = 5;
let b = a;

println!("{}", a);
println!("{}", b); // Both will work!

```

Rust automatically implements copy for these cheap types.
For complex types you really want to copy, explicitly call .clone()


#### Borrowing

Finally, we can understand that move semantics can be pretty annoying sometimes, especially when passing values to functions (the value would be droppec once the function ends, because it's owner would go out of scope)

##### Immutable References (`&T`)

```rust
fn main() {
	let s1 = String::from("Important Value");
	let len = calculate_length(&s1); // &[identifier] returns the variable's immutable reference

	println!("The length of the string is: {}", len);
}

fn calculate_length(s: &String) -> usize {
	s.len() // (implicit return)
}
```

##### Mutable References (`&mut T`)

```rust
fn main() {
	let s1 = String::from("Hi");
	let s2 = append_message(&mut s1);
	println!(s2);
}

fn append_message(s: &mut String) {
	s.push_str(" mom!");
}
```

Important rule on mutable references: 


### Using structs

Structs are first declared with:

struct [name] {
	[member]: [type]
}

And then implementations are written using

impl [name] {
	fn [function_name]() -> [return_type] {

	}
}

Direct instantiaton can happen through:

[name] {}


So an example would be:

struct Bread {
	type: str,
	is_whole: bool
}

impl struct Bread {
	// A constructor function
	fn new(type: str, is_whole: bool) {
		Bread {type, is_whole} // Inferred return.
	}
}

## Classes

### Arc


### Mutex