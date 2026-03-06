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
        inputs: VecDeque::from(vec![10, 3]),
        // (10+3) + (10-3) = 13 + 7 = 20
        expected_outputs: VecDeque::from(vec![20]),
    });
}

#[test]
fn test_deeply_nested_expression() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(1 + (2 + (3 + (4 + (5 + 6))))) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![21]),
    });
}

#[test]
fn test_expression_with_multiple_reads() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(read_int() + read_int() + read_int()) }",
        inputs: VecDeque::from(vec![11, 22, 33]),
        expected_outputs: VecDeque::from(vec![66]),
    });
}

// ── Multiplication ──────────────────────────────────────────────

#[test]
fn test_multiply_expr() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int((read_int() * read_int()) + (read_int() * read_int())) }",
        inputs: VecDeque::from([10, 20, 30, 40]),
        expected_outputs: VecDeque::from([(10 * 20) + (30 * 40)]),
    });
}

#[test]
fn test_multiply_constants() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(3 * 4) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from([12]),
    });
}

#[test]
fn test_multiply_by_zero() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x * 0)
}",
        inputs: VecDeque::from([42]),
        expected_outputs: VecDeque::from([0]),
    });
}

#[test]
fn test_multiply_by_one() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x * 1)
}",
        inputs: VecDeque::from([99]),
        expected_outputs: VecDeque::from([99]),
    });
}

#[test]
fn test_multiply_negative() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x * y)
}",
        inputs: VecDeque::from([-3, 7]),
        expected_outputs: VecDeque::from([-21]),
    });
}

#[test]
fn test_multiply_both_negative() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x * y)
}",
        inputs: VecDeque::from([-4, -5]),
        expected_outputs: VecDeque::from([20]),
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
        inputs: VecDeque::from([6, 7]),
        expected_outputs: VecDeque::from([42]),
    });
}

#[test]
fn test_multiply_self_assign() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
x = x * x
print_int(x)
}",
        inputs: VecDeque::from([5]),
        expected_outputs: VecDeque::from([25]),
    });
}

#[test]
fn test_multiply_chained() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(read_int() * read_int() * read_int()) }",
        inputs: VecDeque::from([2, 3, 4]),
        expected_outputs: VecDeque::from([24]),
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
        inputs: VecDeque::from([3, 4, 5]),
        expected_outputs: VecDeque::from([3 * 4 + 5]),
    });
}

#[test]
fn test_multiply_in_conditional() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
if x > 0 { print_int(x * 10) }
else { print_int(x * 20) }
}",
        inputs: VecDeque::from([3]),
        expected_outputs: VecDeque::from([30]),
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
        inputs: VecDeque::from([5]),
        expected_outputs: VecDeque::from([120]),
    });
}

#[test]
fn test_multiply_large_values() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x * y)
}",
        inputs: VecDeque::from([100000, 100000]),
        expected_outputs: VecDeque::from([10000000000_i64]),
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
        inputs: VecDeque::from([2, 3, 4, 5, 6, 7]),
        expected_outputs: VecDeque::from([6, 20, 42, 2 + 3 + 4 + 5 + 6 + 7]),
    });
}

// ── Division ──────────────────────────────────────────────────────

#[test]
fn test_divide_constants() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(12 / 4) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from([3]),
    });
}

#[test]
fn test_divide_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x / 3)
}",
        inputs: VecDeque::from([21]),
        expected_outputs: VecDeque::from([7]),
    });
}

#[test]
fn test_divide_two_variables() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x / y)
}",
        inputs: VecDeque::from([100, 4]),
        expected_outputs: VecDeque::from([25]),
    });
}

#[test]
fn test_divide_by_one() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x / 1)
}",
        inputs: VecDeque::from([99]),
        expected_outputs: VecDeque::from([99]),
    });
}

#[test]
fn test_divide_truncates() {
    // Integer division truncates toward zero
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(7 / 2) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from([3]),
    });
}

#[test]
fn test_divide_negative_truncates() {
    // -7 / 2 truncates toward zero => -3
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x / 2)
}",
        inputs: VecDeque::from([-7]),
        expected_outputs: VecDeque::from([-3]),
    });
}

#[test]
fn test_divide_both_negative() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = read_int()
print_int(x / y)
}",
        inputs: VecDeque::from([-20, -4]),
        expected_outputs: VecDeque::from([5]),
    });
}

#[test]
fn test_divide_stored_in_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = x / 6
print_int(y)
}",
        inputs: VecDeque::from([42]),
        expected_outputs: VecDeque::from([7]),
    });
}

#[test]
fn test_divide_mixed_with_multiply() {
    execute_test_case(TestCase {
        input: "fn main() -> int { a = read_int()
b = read_int()
print_int(a * b / 2)
}",
        inputs: VecDeque::from([6, 4]),
        // 6 * 4 / 2 = 12
        expected_outputs: VecDeque::from([12]),
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
        inputs: VecDeque::from([100, 5, 81, 9, 64, 8]),
        expected_outputs: VecDeque::from([20, 9, 8]),
    });
}

// ── Shifts ──────────────────────────────────────────────────────

#[test]
fn test_left_shift_constants() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(1 << 3) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from([8]),
    });
}

#[test]
fn test_right_shift_constants() {
    execute_test_case(TestCase {
        input: "fn main() -> int { print_int(16 >> 2) }",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from([4]),
    });
}

#[test]
fn test_left_shift_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x << 4)
}",
        inputs: VecDeque::from([3]),
        expected_outputs: VecDeque::from([48]),
    });
}

#[test]
fn test_right_shift_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x >> 3)
}",
        inputs: VecDeque::from([64]),
        expected_outputs: VecDeque::from([8]),
    });
}

#[test]
fn test_left_shift_by_zero() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x << 0)
}",
        inputs: VecDeque::from([42]),
        expected_outputs: VecDeque::from([42]),
    });
}

#[test]
fn test_right_shift_by_zero() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x >> 0)
}",
        inputs: VecDeque::from([42]),
        expected_outputs: VecDeque::from([42]),
    });
}

#[test]
fn test_right_shift_arithmetic() {
    // Right shift on a negative value should sign-extend (arithmetic shift)
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int(x >> 1)
}",
        inputs: VecDeque::from([-8]),
        expected_outputs: VecDeque::from([-4]),
    });
}

#[test]
fn test_left_shift_stored_in_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
y = x << 2
print_int(y)
}",
        inputs: VecDeque::from([5]),
        expected_outputs: VecDeque::from([20]),
    });
}

#[test]
fn test_shift_in_expression() {
    execute_test_case(TestCase {
        input: "fn main() -> int { x = read_int()
print_int((x << 3) + (x >> 1))
}",
        inputs: VecDeque::from([4]),
        // (4 << 3) + (4 >> 1) = 32 + 2 = 34
        expected_outputs: VecDeque::from([34]),
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
        inputs: VecDeque::from([1, 8, 3, 32]),
        expected_outputs: VecDeque::from([2, 4, 12, 8]),
    });
}
