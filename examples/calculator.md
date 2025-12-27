# Exercise: Calculator with Error Handling

::: exercise
id: calculator
difficulty: intermediate
time: 25 minutes
prerequisites: [hello-world]
:::

Now that you can write basic functions, let's tackle something more
challenging: a calculator that handles errors gracefully.

In real-world code, things go wrong. Users pass invalid input. Operations
fail. Good Rust code uses the type system to handle these cases explicitly.

::: objectives
thinking:
  - Understand when and why to use `Result` for error handling
  - Learn the difference between recoverable and unrecoverable errors
  - Recognize patterns for validating input before operations

doing:
  - Implement a function that returns `Result<T, E>`
  - Handle division by zero as an error case
  - Use pattern matching to process different operations
:::

::: discussion
Before we implement, let's think about error handling:

- What should happen if someone tries to divide by zero?
- In other languages, how do you typically handle this? (exceptions, null, etc.)
- Why might returning a Result be better than throwing an exception?
- What information should an error message include?
:::

## Your Task

Implement a `calculate` function that:
1. Takes two numbers and an operation (add, subtract, multiply, divide)
2. Returns `Ok(result)` for valid operations
3. Returns `Err(message)` for invalid operations (like division by zero)

::: starter file="src/lib.rs"
```rust
use std::fmt;

/// Supported mathematical operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Errors that can occur during calculation
#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorError {
    DivisionByZero,
    Overflow,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Cannot divide by zero"),
            CalculatorError::Overflow => write!(f, "Arithmetic overflow occurred"),
        }
    }
}

impl std::error::Error for CalculatorError {}

/// Performs a calculation on two numbers.
///
/// # Arguments
///
/// * `a` - First operand
/// * `b` - Second operand
/// * `op` - The operation to perform
///
/// # Returns
///
/// * `Ok(f64)` - The result of the calculation
/// * `Err(CalculatorError)` - If the operation cannot be performed
///
/// # Examples
///
/// ```
/// use calculator::{calculate, Operation};
///
/// let result = calculate(10.0, 5.0, Operation::Add);
/// assert_eq!(result, Ok(15.0));
///
/// let error = calculate(10.0, 0.0, Operation::Divide);
/// assert!(error.is_err());
/// ```
pub fn calculate(a: f64, b: f64, op: Operation) -> Result<f64, CalculatorError> {
    // TODO: Implement the calculation
    //
    // Steps:
    // 1. Match on the operation
    // 2. For division, check if b is zero BEFORE dividing
    // 3. Return Ok(result) for valid operations
    // 4. Return Err(CalculatorError::DivisionByZero) for division by zero

    todo!("Implement the calculate function")
}
```
:::

## Hints

::: hint level=1 title="Using match on enums"
Use a `match` expression to handle each operation:

```rust
match op {
    Operation::Add => { /* ... */ },
    Operation::Subtract => { /* ... */ },
    Operation::Multiply => { /* ... */ },
    Operation::Divide => { /* ... */ },
}
```

Each arm should return a `Result<f64, CalculatorError>`.
:::

::: hint level=2 title="Handling Division"
For division, check for zero before performing the operation:

```rust
Operation::Divide => {
    if b == 0.0 {
        return Err(CalculatorError::DivisionByZero);
    }
    Ok(a / b)
}
```

Note: We check `b == 0.0` before dividing to prevent the error.
:::

::: hint level=3 title="Complete Implementation"
```rust
pub fn calculate(a: f64, b: f64, op: Operation) -> Result<f64, CalculatorError> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == 0.0 {
                Err(CalculatorError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }
    }
}
```
:::

## Solution

::: solution
```rust
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorError {
    DivisionByZero,
    Overflow,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Cannot divide by zero"),
            CalculatorError::Overflow => write!(f, "Arithmetic overflow occurred"),
        }
    }
}

impl std::error::Error for CalculatorError {}

pub fn calculate(a: f64, b: f64, op: Operation) -> Result<f64, CalculatorError> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == 0.0 {
                Err(CalculatorError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }
    }
}
```

### Explanation

**Why use `Result` instead of panicking?**

In Rust, we distinguish between:
- **Recoverable errors**: Use `Result<T, E>` - the caller can handle the error
- **Unrecoverable errors**: Use `panic!` - the program cannot continue

Division by zero is recoverable - the caller might want to show an error
message, use a default value, or try a different calculation.

**Why check before dividing?**

With floating-point numbers, `10.0 / 0.0` actually produces `inf` (infinity)
rather than panicking. But for a calculator, we want to treat this as an
error. By checking first, we can return a clear error message.

**The `match` pattern**

Each arm of the match returns the same type: `Result<f64, CalculatorError>`.
This is enforced by the compiler - all arms must return the same type.

**Error type design**

We created a custom `CalculatorError` enum rather than using a string. This:
- Allows pattern matching on specific error types
- Provides type safety
- Implements `std::error::Error` for compatibility with `?` operator
:::

## Tests

Since this exercise uses custom types, run these tests locally with `cargo test`:

::: tests mode=local
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate(5.0, 3.0, Operation::Add), Ok(8.0));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate(10.0, 4.0, Operation::Subtract), Ok(6.0));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate(7.0, 6.0, Operation::Multiply), Ok(42.0));
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate(15.0, 3.0, Operation::Divide), Ok(5.0));
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(
            calculate(10.0, 0.0, Operation::Divide),
            Err(CalculatorError::DivisionByZero)
        );
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(calculate(-5.0, 3.0, Operation::Add), Ok(-2.0));
    }

    #[test]
    fn test_floating_point() {
        let result = calculate(0.1, 0.2, Operation::Add).unwrap();
        assert!((result - 0.3).abs() < 0.0001);
    }
}
```
:::

## Reflection

::: reflection
After completing this exercise, consider:

1. **Error handling**: What are the tradeoffs between returning `Result`,
   using `Option`, and panicking? When would you choose each?

2. **Type design**: We used an enum for `CalculatorError`. What if we had
   used `String` for error messages instead? What would we lose?

3. **Floating-point**: The test for `0.1 + 0.2` uses approximate comparison.
   Why can't we just use `==` for floating-point numbers?

4. **Extension**: How would you add a `Power` operation? What about handling
   negative exponents?

5. **Real-world**: In production code, would you use `f64` for a financial
   calculator? Why or why not?
:::
