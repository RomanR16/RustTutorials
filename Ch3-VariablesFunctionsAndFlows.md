# Chapter 3
This chapter discusses variables in Rust.

Mutable VS. Immutable variables:
- Variables are immutable by default.
- Use 'mut' to declare a variable as mutable.
- Immutable variables can be redeclared as different types but mutable variables cannot.

Constants: 
- Use 'const' to declare a constant value.
- Constants must be known at runtime.
- Constants can contain calculations such as '12 + 3 * 7'

Type declaration:
- Some variables do not need an explicit type.
- Some variables do need explicit types such as the guess variable from the guessing game. This is because .parse() can convert a string to multiple number types, so the compiler needs to know the exact type that should be used.
- Types are delcared using ':'
	- Ex. let guess: u32 = "123".parse().expect("Must be number");


## Variable types
There are 2 categories of variables in Rust. 

Scalar types: These are variables that represent a single value. There are 4 commond types of Scalar values
- Integers: There are a wide range of bit sizes for integers from 8 to 128, and they can all be signed (i) or unsigned (u). There are also architecture based integers (usize/isize)
	- Ex: i32 - signed 32 bit int
- Floating point numbers: Decimal numbers with 2 types that are f32 and f64. f64 is the default floating point type since most architectures are 64 bit. 
- Boolean values: True/false. Uses bool for type declaration. Stored using 1 byte.
- Characters: Declared using char. Characters are stored using 4 bytes meaning chars can represent more than just typical ASCII characters, like emojis or different language characters using unicode. This is interesting. There will be more about chars in a chapter 8.

Compound types: Used to group multiple values into 1 type. There are two main compound types which are tuples and arrays.
Tuples: Used to store values of different types. Similar to database tuples. Once the tuple is declared, its size cannot be changed. You do not need to declare the type of each attribute in the tuple, but it is a nice option to have.
- Ex. let tup: (i32, f64, u8) = (500, 6.4, 1);
- There are two ways to get attributes from a tuple.
- let (x, y, z) = tup;// passes in each attribute into x, y, and z seperately
- let x = tup.0;// Get individual attribute using index
- There exist tuples without any values called a unit, and they are returned by expressions that do not return any values. Represented using ()

Arrays: Similar to java arrays since they cannot be resized. They also only store a single variable type. They are used when you want the values stored on the stack rather than the heap. 
- Declared using let arr = [1, 2, 3, 4];
- Also declared using let arr: [u32; 5] = [1, 2, 3, 4, 5]; // Declare by specifying type and length
- Also declared using let arr = [3; 5]; // Declared an array of length 5 with all values set to 3.
- Access values using regular array index notation: let x = arr[0];
- Rust prevents out of bounds index accessing by stopping the program before an out of bounds memory address is accessed. There is still a crash and error message displayed, but the memory is not accessed.

## Functions
Functions are declared using the fn keyword.

- Ex: fn another_function(){...}
- Functions can be declared before or after the function that calls it, unlike C.
- Functions can have parameters and the type must be declared: 
	- Ex. fn another_function(x: i32, c: char){...}

Statements VS. Expressions: It is important to know the difference between an expression and statmeent.
- Statements do not return a value.
- Expressions do return values.
- Some functions are expressions and others are statements based on what they do and if they return any values.

Functions with return types: Functions return the last line of the function block if it does not have a ; at the end. Functions can also return a value early using the return keyword. The return type is declared using an arrow (->) followed by the type.For example:

```
fn five() -> u32 {
	5 // Returns 5
}
```

Or

```
fn plus_one(x: i32){
	x + 1 // Returns the value of x plus 1 
}
```


## Control Flow
Conditionals are used to control the flow of a program based on if a condition is true or false. There are conditionals in if statements and loops.

If expressions do not put the conditional within parenthesis like other programming languages. The concpet of if-else expressions are exactly the same as other porgramming languages. Here is an example of an if-else expression

```
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

You can also use if-else expressions when declaring variables:
```
let number = if condition { 5 } else { 6 };

```

Loops: There are 3 main types of loops in Rust, which are loops, while, and for loops.

The 'loop' loop is used to repeat a code block until the program is stopped or until the code meets a condition that breaks the loop. The following example repeats until the user manually terminates the program:

```
loop {
	println!("again!");
}
```

You can break loops using the break keyword, and you can skip an iteration of a loop using continue. You can also return values from a loop by placing the return value after the break keyword. For example:

```
let mut counter = 0;

let result = loop {
	counter += 1;

	if counter == 10 {
		break counter * 2;// The result of this expression is returned
	}
};

println!("The result is {result}");
```

Loops can be labeled to specify which loop should be broken out of when using the break keyword. For example:

```
let mut count = 0;

'counting_up: loop {// Loop labeled as 'counting_up'
	println!("count = {count}");
	let mut remaining = 10;

	loop {
		println!("remaining = {remaining}");
		if remaining == 9 {
			break;// Breaks inner loop
		}
		if count == 2 {
			break 'counting_up;// breaks outer loop
		}
		remaining -= 1;
	}

	count += 1;
}

println!("End count = {count}");
```

While loops work as expected. The loop repeats until a condition is not true. Below is an example of the syntax used to make a while loop:

```
while number != 0 {
	println!("{number}!");

	number -= 1;
}
```

For loops can be used to iterate through values of a collection like an array. For example, instead of using awhile loop and iterating through each index, you can loop through each value within the array using the following notation:

```
let a = [1, 2, 3, 4, 5];

for element in a {// Element is the variable that holds the current value read from the array.
	println!("the value is: {element}");
}
```

You can also loop through a rnage of numbers using a range. An example is shown below:

```
for number in (1..4) {// 1 to 4 is the number range, and number is the current number for the iteration.
	println!("{number}!");
}
```






