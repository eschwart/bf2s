enum BfToken {
    IncrementPtr,  // >
    DecrementPtr,  // <
    IncrementByte, // +
    DecrementByte, // -
    OutputByte,    // .
    JumpForward,   // [
    JumpBackward,  // ]
}

impl BfToken {
    const fn from_char(value: char) -> Self {
        match value {
            '>' => BfToken::IncrementPtr,
            '<' => BfToken::DecrementPtr,
            '+' => BfToken::IncrementByte,
            '-' => BfToken::DecrementByte,
            '.' => BfToken::OutputByte,
            '[' => BfToken::JumpForward,
            ']' => BfToken::JumpBackward,
            _ => {
                if value == ',' {
                    panic!("input byte not allowed.")
                } else {
                    panic!("invalid bf token.")
                }
            }
        }
    }
}

/// interpret brainfuck code into a string output.
pub fn bf_to_str(code: &str) -> String {
    let mut tape = [0u8; 255];
    let mut ptr = 0usize;
    let mut pc = 0usize;
    let code_bytes = code.as_bytes();
    let mut output = String::new();
    let mut loop_stack = Vec::new();
    let mut steps = 0;

    let mut incr = move |pc: &mut usize| {
        steps += 1;
        if steps == u16::MAX {
            panic!("bf code is too long/complex.")
        }
        *pc += 1;
    };

    while pc < code_bytes.len() {
        let c = code_bytes[pc] as char;
        if let ' ' | '\n' = c {
            incr(&mut pc);
            continue;
        }

        match BfToken::from_char(c) {
            BfToken::IncrementPtr => ptr += 1,
            BfToken::DecrementPtr => ptr = ptr.saturating_sub(1),
            BfToken::IncrementByte => tape[ptr] = tape[ptr].wrapping_add(1),
            BfToken::DecrementByte => tape[ptr] = tape[ptr].wrapping_sub(1),
            BfToken::OutputByte => output.push(tape[ptr] as char),
            BfToken::JumpForward => {
                if tape[ptr] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        pc += 1;
                        if pc >= code_bytes.len() {
                            break;
                        }
                        match BfToken::from_char(code_bytes[pc] as char) {
                            BfToken::JumpForward => depth += 1,
                            BfToken::JumpBackward => depth -= 1,
                            _ => (),
                        }
                    }
                } else {
                    loop_stack.push(pc);
                }
            }
            BfToken::JumpBackward => {
                if tape[ptr] != 0 {
                    if let Some(&start) = loop_stack.last() {
                        pc = start;
                        continue;
                    }
                } else {
                    loop_stack.pop();
                }
            }
        }
        incr(&mut pc)
    }
    output
}
