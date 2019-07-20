use std::collections::VecDeque;

pub fn brackets_are_balanced(text: &str) -> bool {
    if text.is_empty() {
        return true;
    }
    // maintain stack
    let mut stack = VecDeque::new();
    for c in text.chars() {
        println!("stack : {:?} ", &stack);

        if c == '[' || c == '{' || c == '(' {
            stack.push_front(c);
            continue;
        }

        match c {
            ']' => {
                let matching = stack.pop_front();
                println!("matching {:?} ", c);
                if matching != Some('[') {
                    return false;
                }
            }
            '}' => {
                let matching = stack.pop_front();
                println!("matching {:?} ", c);
                if matching != Some('{') {
                    return false;
                }
            }
            ')' => {
                let matching = stack.pop_front();
                println!("matching {:?} ", c);
                if matching != Some('(') {
                    return false;
                }
            }
            _ => {}
        }
    }

    return stack.is_empty();
}
