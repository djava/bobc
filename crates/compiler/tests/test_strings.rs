mod utils;
use utils::*;

fn str_output(s: &str) -> Value {
    Value::Array(s.chars().chain(std::iter::once('\0')).map(Value::Char).collect())
}

fn char_output(c: char) -> Value {
    Value::Array(vec![Value::Char(c)])
}

// ── Basic print_str ──────────────────────────────────────────────

#[test]
fn test_print_str_literal() {
    execute_test_case(TestCase {
        input: r#"fn main() -> int { print_str("hello") }"#,
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("hello")]),
    });
}

#[test]
fn test_print_str_assigned_variable() {
    execute_test_case(TestCase {
        input: "fn main() -> int {
s = \"hello\"
print_str(s)
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("hello")]),
    });
}

#[test]
fn test_print_str_empty() {
    execute_test_case(TestCase {
        input: r#"fn main() -> int { print_str("") }"#,
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("")]),
    });
}

#[test]
fn test_print_str_single_char() {
    execute_test_case(TestCase {
        input: r#"fn main() -> int { print_str("a") }"#,
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("a")]),
    });
}

#[test]
fn test_print_str_multiple_calls() {
    execute_test_case(TestCase {
        input: "fn main() -> int {
print_str(\"hello\")
print_str(\"world\")
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("hello"), str_output("world")]),
    });
}

// ── String assignment ────────────────────────────────────────────

#[test]
fn test_string_reassign() {
    execute_test_case(TestCase {
        input: "fn main() -> int {
s = \"hello\"
s = \"world\"
print_str(s)
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("world")]),
    });
}

#[test]
fn test_string_multiple_variables() {
    execute_test_case(TestCase {
        input: "fn main() -> int {
a = \"foo\"
b = \"bar\"
print_str(a)
print_str(b)
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("foo"), str_output("bar")]),
    });
}

// ── String indexing ──────────────────────────────────────────────

#[test]
fn test_string_index_first_char() {
    execute_test_case(TestCase {
        input: "fn main() -> int {
s = \"hello\"
print_str([s[0]])
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![char_output('h')]),
    });
}

#[test]
fn test_string_index_multiple_chars() {
    execute_test_case(TestCase {
        input: "fn main() -> int {
s = \"abc\"
print_str([s[0]])
print_str([s[1]])
print_str([s[2]])
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![
            char_output('a'),
            char_output('b'),
            char_output('c'),
        ]),
    });
}

#[test]
fn test_string_index_null_terminator() {
    execute_test_case(TestCase {
        input: "fn main() -> int {
s = \"hi\"
print_str([s[2]])
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![char_output('\0')]),
    });
}

#[test]
fn test_string_index_variable_index() {
    execute_test_case(TestCase {
        input: "fn main() -> int {
s = \"xyz\"
i = 1
print_str([s[i]])
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![char_output('y')]),
    });
}

// ── Char literals ────────────────────────────────────────────────

#[test]
fn test_char_literal() {
    execute_test_case(TestCase {
        input: r#"fn main() -> int { print_str(['a']) }"#,
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![char_output('a')]),
    });
}

#[test]
fn test_char_literal_assigned() {
    execute_test_case(TestCase {
        input: "fn main() -> int {
c = 'x'
print_str([c])
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![char_output('x')]),
    });
}

#[test]
fn test_char_passed_to_function() {
    execute_test_case(TestCase {
        input: "fn print_char(c: char) {
    print_str([c])
}
fn main() -> int {
    print_char('z')
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![char_output('z')]),
    });
}

#[test]
fn test_char_returned_from_function() {
    execute_test_case(TestCase {
        input: "fn get_char() -> char {
    return 'q'
}
fn main() -> int {
    print_str([get_char()])
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![char_output('q')]),
    });
}

#[test]
fn test_char_from_string_index_to_function() {
    execute_test_case(TestCase {
        input: "fn first_char(s: string) -> char {
    return s[0]
}
fn main() -> int {
    print_str([first_char(\"hello\")])
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![char_output('h')]),
    });
}

// ── Passing strings to and from functions ────────────────────────

#[test]
fn test_string_passed_to_function() {
    execute_test_case(TestCase {
        input: "fn print_greeting(s: string) {
    print_str(s)
}
fn main() -> int {
    print_greeting(\"hello\")
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("hello")]),
    });
}

#[test]
fn test_string_returned_from_function() {
    execute_test_case(TestCase {
        input: "fn get_str() -> string {
    return \"hello\"
}
fn main() -> int {
    print_str(get_str())
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("hello")]),
    });
}

#[test]
fn test_string_passed_and_returned() {
    execute_test_case(TestCase {
        input: "fn echo(s: string) -> string {
    return s
}
fn main() -> int {
    print_str(echo(\"hello\"))
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("hello")]),
    });
}

// ── Strings in control flow ──────────────────────────────────────

#[test]
fn test_string_in_if_true_branch() {
    execute_test_case(TestCase {
        input: "fn main() -> int {
x = 1
if x == 1 {
    print_str(\"yes\")
} else {
    print_str(\"no\")
}
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("yes")]),
    });
}

#[test]
fn test_string_in_if_false_branch() {
    execute_test_case(TestCase {
        input: "fn main() -> int {
x = 0
if x == 1 {
    print_str(\"yes\")
} else {
    print_str(\"no\")
}
}",
        inputs: VecDeque::new(),
        expected_outputs: VecDeque::from(vec![str_output("no")]),
    });
}
