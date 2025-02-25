### Quasi-Pure Architecture  

#### Objectives  
- Enhance code predictability and maintainability.  
- Limit the scope of side effects to improve testability.  
- Leverage Rust’s performance while maintaining compatibility with asynchronous processing.  

#### Core Principles  
- **Declarative Implementation via Iterators**  
  - Avoid implementing algorithms using `while` loops or recursive functions.  
  - Strive for clear, composable code while minimizing unnecessary object regeneration.  
  - Utilize `std::ops::ControlFlow` or `try_○○` methods for conditional early exits.  

- **State Mutations Must Not Affect Callers**  
  - Prohibit the use of mutable references like `&mut T` in function parameters.  
  - Forbid accepting mutable values such as `mut T`.  
  - State-modifying operations must consume the original value and return a new one in the form of `T -> T`.  
  - Require the use of `#[must_use]` to enforce handling of returned values.  

- **Conditions for Using Interior Mutability**  
  Interior mutability is permitted only when **all** of the following conditions are met:  
  - Copying incurs a high cost.  
  - The data must persist beyond the current scope.  
  - Multiple read/write operations occur within a short timeframe.  

#### Challenges  
- Increased complexity.  
- Enforced handling of state-modified values.  