mod utils;
use utils::*;

// ── Complex expressions ──────────────────────────────────────────

#[test]
fn test_nested_arithmetic() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int((x + y) + (x - y))
}",
        inputs: VecDeque::from(vec![Value::I64(10), Value::I64(3)]),
        // (10+3) + (10-3) = 13 + 7 = 20
        expected_outputs: VecDeque::from(vec![Value::I64(20)]),
    });
}

#[test]
fn test_deeply_nested_expression() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(1 + (2 + (3 + (4 + (5 + 6))))) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![Value::I64(21)]),
    });
}

#[test]
fn test_expression_with_multiple_reads() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(read_int() + read_int() + read_int()) }",
        inputs: VecDeque::from(vec![Value::I64(11), Value::I64(22), Value::I64(33)]),
        expected_outputs: VecDeque::from(vec![Value::I64(66)]),
    });
}

// ── Multiplication ──────────────────────────────────────────────

#[test]
fn test_multiply_expr() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int((read_int() * read_int()) + (read_int() * read_int())) }",
        inputs: VecDeque::from(vec![
            Value::I64(10),
            Value::I64(20),
            Value::I64(30),
            Value::I64(40),
        ]),
        expected_outputs: VecDeque::from(vec![Value::I64((10 * 20) + (30 * 40))]),
    });
}

#[test]
fn test_multiply_constants() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(3 * 4) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![Value::I64(12)]),
    });
}

#[test]
fn test_multiply_by_zero() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x * 0)
}",
        inputs: VecDeque::from(vec![Value::I64(42)]),
        expected_outputs: VecDeque::from(vec![Value::I64(0)]),
    });
}

#[test]
fn test_multiply_by_one() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x * 1)
}",
        inputs: VecDeque::from(vec![Value::I64(99)]),
        expected_outputs: VecDeque::from(vec![Value::I64(99)]),
    });
}

#[test]
fn test_multiply_negative() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x * y)
}",
        inputs: VecDeque::from(vec![Value::I64(-3), Value::I64(7)]),
        expected_outputs: VecDeque::from(vec![Value::I64(-21)]),
    });
}

#[test]
fn test_multiply_both_negative() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x * y)
}",
        inputs: VecDeque::from(vec![Value::I64(-4), Value::I64(-5)]),
        expected_outputs: VecDeque::from(vec![Value::I64(20)]),
    });
}

#[test]
fn test_multiply_variables() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
z = x * y
print_int(z)
}",
        inputs: VecDeque::from(vec![Value::I64(6), Value::I64(7)]),
        expected_outputs: VecDeque::from(vec![Value::I64(42)]),
    });
}

#[test]
fn test_multiply_self_assign() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
x = x * x
print_int(x)
}",
        inputs: VecDeque::from(vec![Value::I64(5)]),
        expected_outputs: VecDeque::from(vec![Value::I64(25)]),
    });
}

#[test]
fn test_multiply_chained() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(read_int() * read_int() * read_int()) }",
        inputs: VecDeque::from(vec![Value::I64(2), Value::I64(3), Value::I64(4)]),
        expected_outputs: VecDeque::from(vec![Value::I64(24)]),
    });
}

#[test]
fn test_multiply_mixed_with_add_sub() {
    execute_test_case(TestCase {
        input: "fn main() -> int { a = read_int()
b = read_int()
c = read_int()
print_int(a * b + c)
}",
        inputs: VecDeque::from(vec![Value::I64(3), Value::I64(4), Value::I64(5)]),
        expected_outputs: VecDeque::from(vec![Value::I64(3 * 4 + 5)]),
    });
}

#[test]
fn test_multiply_in_conditional() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
if x > 0 { print_int(x * 10) }
else { print_int(x * 20) }
}",
        inputs: VecDeque::from(vec![Value::I64(3)]),
        expected_outputs: VecDeque::from(vec![Value::I64(30)]),
    });
}

#[test]
fn test_multiply_in_loop() {
    // Compute factorial of 5 iteratively
    execute_test_case(TestCase {
        input: "fn main() -> int { n = read_int()
result = 1
while n > 0 {
    result = result * n
    n = n - 1
}
print_int(result)
}",
        inputs: VecDeque::from(vec![Value::I64(5)]),
        expected_outputs: VecDeque::from(vec![Value::I64(120)]),
    });
}

#[test]
fn test_multiply_large_values() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x * y)
}",
        inputs: VecDeque::from(vec![Value::I64(100000), Value::I64(100000)]),
        expected_outputs: VecDeque::from(vec![Value::I64(10000000000_i64)]),
    });
}

#[test]
fn test_multiply_register_pressure() {
    // Many live variables across a multiply to stress register allocation
    execute_test_case(TestCase {
        input: "fn main() -> int { a = read_int()
b = read_int()
c = read_int()
d = read_int()
e = read_int()
f = read_int()
print_int(a * b)
print_int(c * d)
print_int(e * f)
print_int(a + b + c + d + e + f)
}",
        inputs: VecDeque::from(vec![
            Value::I64(2),
            Value::I64(3),
            Value::I64(4),
            Value::I64(5),
            Value::I64(6),
            Value::I64(7),
        ]),
        expected_outputs: VecDeque::from(vec![
            Value::I64(6),
            Value::I64(20),
            Value::I64(42),
            Value::I64(2 + 3 + 4 + 5 + 6 + 7),
        ]),
    });
}

// ── Division ──────────────────────────────────────────────────────

#[test]
fn test_divide_constants() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(12 / 4) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![Value::I64(3)]),
    });
}

#[test]
fn test_divide_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x / 3)
}",
        inputs: VecDeque::from(vec![Value::I64(21)]),
        expected_outputs: VecDeque::from(vec![Value::I64(7)]),
    });
}

#[test]
fn test_divide_two_variables() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x / y)
}",
        inputs: VecDeque::from(vec![Value::I64(100), Value::I64(4)]),
        expected_outputs: VecDeque::from(vec![Value::I64(25)]),
    });
}

#[test]
fn test_divide_by_one() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x / 1)
}",
        inputs: VecDeque::from(vec![Value::I64(99)]),
        expected_outputs: VecDeque::from(vec![Value::I64(99)]),
    });
}

#[test]
fn test_divide_truncates() {
    // Integer division truncates toward zero
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(7 / 2) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![Value::I64(3)]),
    });
}

#[test]
fn test_divide_negative_truncates() {
    // -7 / 2 truncates toward zero => -3
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x / 2)
}",
        inputs: VecDeque::from(vec![Value::I64(-7)]),
        expected_outputs: VecDeque::from(vec![Value::I64(-3)]),
    });
}

#[test]
fn test_divide_both_negative() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x / y)
}",
        inputs: VecDeque::from(vec![Value::I64(-20), Value::I64(-4)]),
        expected_outputs: VecDeque::from(vec![Value::I64(5)]),
    });
}

#[test]
fn test_divide_stored_in_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = x / 6
print_int(y)
}",
        inputs: VecDeque::from(vec![Value::I64(42)]),
        expected_outputs: VecDeque::from(vec![Value::I64(7)]),
    });
}

#[test]
fn test_divide_mixed_with_multiply() {
    execute_test_case(TestCase {
        input: "fn main() -> int { a = read_int()
b = read_int()
print_int(a * b / 2)
}",
        inputs: VecDeque::from(vec![Value::I64(6), Value::I64(4)]),
        // 6 * 4 / 2 = 12
        expected_outputs: VecDeque::from(vec![Value::I64(12)]),
    });
}

#[test]
fn test_divide_register_pressure() {
    execute_test_case(TestCase {
        input: "fn main() -> int { a = read_int()
b = read_int()
c = read_int()
d = read_int()
e = read_int()
f = read_int()
print_int(a / b)
print_int(c / d)
print_int(e / f)
}",
        inputs: VecDeque::from(vec![
            Value::I64(100),
            Value::I64(5),
            Value::I64(81),
            Value::I64(9),
            Value::I64(64),
            Value::I64(8),
        ]),
        expected_outputs: VecDeque::from(vec![Value::I64(20), Value::I64(9), Value::I64(8)]),
    });
}

// ── Shifts ──────────────────────────────────────────────────────

#[test]
fn test_left_shift_constants() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(1 << 3) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![Value::I64(8)]),
    });
}

#[test]
fn test_right_shift_constants() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(16 >> 2) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![Value::I64(4)]),
    });
}

#[test]
fn test_left_shift_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x << 4)
}",
        inputs: VecDeque::from(vec![Value::I64(3)]),
        expected_outputs: VecDeque::from(vec![Value::I64(48)]),
    });
}

#[test]
fn test_right_shift_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x >> 3)
}",
        inputs: VecDeque::from(vec![Value::I64(64)]),
        expected_outputs: VecDeque::from(vec![Value::I64(8)]),
    });
}

#[test]
fn test_left_shift_by_zero() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x << 0)
}",
        inputs: VecDeque::from(vec![Value::I64(42)]),
        expected_outputs: VecDeque::from(vec![Value::I64(42)]),
    });
}

#[test]
fn test_right_shift_by_zero() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x >> 0)
}",
        inputs: VecDeque::from(vec![Value::I64(42)]),
        expected_outputs: VecDeque::from(vec![Value::I64(42)]),
    });
}

#[test]
fn test_right_shift_arithmetic() {
    // Right shift on a negative value should sign-extend (arithmetic shift)
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x >> 1)
}",
        inputs: VecDeque::from(vec![Value::I64(-8)]),
        expected_outputs: VecDeque::from(vec![Value::I64(-4)]),
    });
}

#[test]
fn test_left_shift_stored_in_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = x << 2
print_int(y)
}",
        inputs: VecDeque::from(vec![Value::I64(5)]),
        expected_outputs: VecDeque::from(vec![Value::I64(20)]),
    });
}

#[test]
fn test_shift_in_expression() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int((x << 3) + (x >> 1))
}",
        inputs: VecDeque::from(vec![Value::I64(4)]),
        // (4 << 3) + (4 >> 1) = 32 + 2 = 34
        expected_outputs: VecDeque::from(vec![Value::I64(34)]),
    });
}

#[test]
fn test_shift_register_pressure() {
    execute_test_case(TestCase {
        input: "fn main() -> int { a = read_int()
b = read_int()
c = read_int()
d = read_int()
print_int(a << 1)
print_int(b >> 1)
print_int(c << 2)
print_int(d >> 2)
}",
        inputs: VecDeque::from(vec![
            Value::I64(1),
            Value::I64(8),
            Value::I64(3),
            Value::I64(32),
        ]),
        expected_outputs: VecDeque::from(vec![
            Value::I64(2),
            Value::I64(4),
            Value::I64(12),
            Value::I64(8),
        ]),
    });
}

// ── Modulo ───────────────────────────────────────────────────────

#[test]
fn test_modulo_constants() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(10 % 3) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![Value::I64(1)]),
    });
}

#[test]
fn test_modulo_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x % 5)
}",
        inputs: VecDeque::from(vec![Value::I64(17)]),
        expected_outputs: VecDeque::from(vec![Value::I64(2)]),
    });
}

#[test]
fn test_modulo_two_variables() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x % y)
}",
        inputs: VecDeque::from(vec![Value::I64(23), Value::I64(7)]),
        expected_outputs: VecDeque::from(vec![Value::I64(2)]),
    });
}

#[test]
fn test_modulo_evenly_divisible() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(12 % 4) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![Value::I64(0)]),
    });
}

#[test]
fn test_modulo_by_one() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x % 1)
}",
        inputs: VecDeque::from(vec![Value::I64(99)]),
        expected_outputs: VecDeque::from(vec![Value::I64(0)]),
    });
}

#[test]
fn test_modulo_negative_dividend() {
    // -7 % 3 = -1 (C/x86 semantics: remainder has sign of dividend)
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x % 3)
}",
        inputs: VecDeque::from(vec![Value::I64(-7)]),
        expected_outputs: VecDeque::from(vec![Value::I64(-1)]),
    });
}

#[test]
fn test_modulo_negative_divisor() {
    // 7 % -3 = 1 (remainder has sign of dividend)
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x % y)
}",
        inputs: VecDeque::from(vec![Value::I64(7), Value::I64(-3)]),
        expected_outputs: VecDeque::from(vec![Value::I64(1)]),
    });
}

#[test]
fn test_modulo_both_negative() {
    // -7 % -3 = -1
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x % y)
}",
        inputs: VecDeque::from(vec![Value::I64(-7), Value::I64(-3)]),
        expected_outputs: VecDeque::from(vec![Value::I64(-1)]),
    });
}

#[test]
fn test_modulo_stored_in_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = x % 7
print_int(y)
}",
        inputs: VecDeque::from(vec![Value::I64(25)]),
        expected_outputs: VecDeque::from(vec![Value::I64(4)]),
    });
}

#[test]
fn test_modulo_mixed_with_arithmetic() {
    execute_test_case(TestCase {
        input: "fn main() -> int { a = read_int()
b = read_int()
r = a % b
q = a / b
print_int(r + q)
}",
        inputs: VecDeque::from(vec![Value::I64(17), Value::I64(5)]),
        // 17 % 5 + 17 / 5 = 2 + 3 = 5
        expected_outputs: VecDeque::from(vec![Value::I64(5)]),
    });
}

#[test]
fn test_modulo_in_conditional() {
    // Check if even or odd
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
if x % 2 == 0 { print_int(1) }
else { print_int(0) }
}",
        inputs: VecDeque::from(vec![Value::I64(6)]),
        expected_outputs: VecDeque::from(vec![Value::I64(1)]),
    });
}

#[test]
fn test_modulo_in_loop() {
    // Print numbers 0-4 that are divisible by 2
    execute_test_case(TestCase {
        input: "fn main() -> int { i = 0
while i < 5 {
    if i % 2 == 0 { print_int(i) }
    i = i + 1
}
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![Value::I64(0), Value::I64(2), Value::I64(4)]),
    });
}

#[test]
fn test_modulo_register_pressure() {
    execute_test_case(TestCase {
        input: "fn main() -> int { a = read_int()
b = read_int()
c = read_int()
d = read_int()
e = read_int()
f = read_int()
print_int(a % b)
print_int(c % d)
print_int(e % f)
}",
        inputs: VecDeque::from(vec![
            Value::I64(17),
            Value::I64(5),
            Value::I64(23),
            Value::I64(7),
            Value::I64(100),
            Value::I64(9),
        ]),
        expected_outputs: VecDeque::from(vec![Value::I64(2), Value::I64(2), Value::I64(1)]),
    });
}
