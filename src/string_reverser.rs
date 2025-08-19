pub fn reverse_string(input: &str) -> String {
    let mut stack = Vec::new();

    for ch in input.chars() {
        stack.push(ch);
    }

    let mut reversed = String::new();
    while let Some(ch) = stack.pop() {
        reversed.push(ch);
    }

    reversed
}
