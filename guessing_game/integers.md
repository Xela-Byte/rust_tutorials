🔢 Difference Between Signed and Unsigned Integers in Rust

1. Signed Integers (i8, i16, i32, i64, i128, isize)

   - Can store both positive and negative numbers.
   - Example: i8 range = -128 to 127
   - Uses one bit for the sign (+/-).

2. Unsigned Integers (u8, u16, u32, u64, u128, usize)

   - Can store only non-negative numbers (0 and up).
   - Example: u8 range = 0 to 255
   - Uses all bits for the numeric value.

3. Key Differences
   - Signed: includes negative values.
   - Unsigned: doubles the positive range (no negatives).
   - Overflow: wrapping occurs if you exceed the type’s max.
   - Common use: signed for math, unsigned for sizes or indexes.

🦀 Example:
let a: i8 = -10; // signed
let b: u8 = 200; // unsigned

📏 Integer Ranges in Rust

## Signed Integers (can be negative or positive)

i8 : -128 to 127
i16 : -32,768 to 32,767
i32 : -2,147,483,648 to 2,147,483,647
i64 : -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
i128 : -2^127 to 2^127 - 1
isize: depends on system
→ 32-bit: -2,147,483,648 to 2,147,483,647
→ 64-bit: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

## Unsigned Integers (only positive)

u8 : 0 to 255
u16 : 0 to 65,535
u32 : 0 to 4,294,967,295
u64 : 0 to 18,446,744,073,709,551,615
u128 : 0 to 2^128 - 1
usize: depends on system
→ 32-bit: 0 to 4,294,967,295
→ 64-bit: 0 to 18,446,744,073,709,551,615

- the formula is 2 raised to power (n - 1) - 1 for signed number
- the formula is 2 raised to power (n + 1) - 1 for unsigned number
