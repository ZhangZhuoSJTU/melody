#![forbid(unsafe_code)]
#![allow(clippy::unused_unit)]
#![allow(clippy::module_name_repetitions)]

use std::collections::HashMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compiler(source: &str) -> Result<String, JsError> {
    let output = melody_compiler::compiler(source);
    output.map_err(|error| JsError::new(&error.to_string()))
}

#[wasm_bindgen]
pub struct TuringResult {
    halted: bool,
    steps: u64,
    ones: usize,
}

#[wasm_bindgen]
impl TuringResult {
    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn halted(&self) -> bool {
        self.halted
    }

    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn steps(&self) -> u64 {
        self.steps
    }

    #[must_use]
    #[wasm_bindgen(getter)]
    pub fn ones(&self) -> usize {
        self.ones
    }
}

#[derive(Clone, Copy)]
struct Rule {
    write: u8,
    movement: i32,
    next_state: usize,
}

#[wasm_bindgen]
pub fn run_turing_machine(rules: &[i32], max_steps: u64) -> Result<TuringResult, JsError> {
    if rules.len() % 5 != 0 {
        return Err(JsError::new(
            "rules length must be a multiple of 5 (state, read, write, movement, next_state)",
        ));
    }

    let mut table: HashMap<(usize, u8), Rule> = HashMap::with_capacity(rules.len() / 5);

    for chunk in rules.chunks_exact(5) {
        let state = to_state(chunk[0], "state")?;
        let read = to_symbol(chunk[1], "read symbol")?;
        let write = to_symbol(chunk[2], "write symbol")?;
        let movement = match chunk[3] {
            -1 | 1 => chunk[3],
            other => {
                return Err(JsError::new(&format!(
                    "movement must be -1 or 1, got {other}"
                )))
            }
        };
        let next_state = to_state(chunk[4], "next state")?;

        if table
            .insert(
                (state, read),
                Rule {
                    write,
                    movement,
                    next_state,
                },
            )
            .is_some()
        {
            return Err(JsError::new(&format!(
                "duplicate rule for state {state} reading {read}"
            )));
        }
    }

    let mut tape: HashMap<i64, u8> = HashMap::new();
    let mut head: i64 = 0;
    let mut state: usize = 0;
    let mut steps: u64 = 0;

    let halted = loop {
        if steps >= max_steps {
            break false;
        }

        let symbol = *tape.get(&head).unwrap_or(&0);

        let rule = match table.get(&(state, symbol)) {
            Some(rule) => *rule,
            None => break true,
        };

        tape.insert(head, rule.write);
        head += i64::from(rule.movement);
        state = rule.next_state;
        steps += 1;
    };

    let ones = tape.values().filter(|&&cell| cell == 1).count();

    Ok(TuringResult {
        halted,
        steps,
        ones,
    })
}

fn to_symbol(value: i32, label: &str) -> Result<u8, JsError> {
    match value {
        0 => Ok(0),
        1 => Ok(1),
        other => Err(JsError::new(&format!("{label} must be 0 or 1, got {other}"))),
    }
}

fn to_state(value: i32, label: &str) -> Result<usize, JsError> {
    usize::try_from(value).map_err(|_| JsError::new(&format!("{label} must be >= 0, got {value}")))
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
