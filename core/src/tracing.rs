//! Allows to listen to runtime events.

use crate::{Capture, ExitReason, Memory, Opcode, Stack, Trap};
use primitive_types::{H160, H256};

environmental::environmental!(listener: dyn EventListener + 'static);

pub trait EventListener {
	fn event(&mut self, event: Event<'_>);
}

#[derive(Debug, Copy, Clone)]
pub enum Event<'a> {
	DebuggingWithOperand {
        opcode: Opcode,
        operands: &'a Vec<H256>,
        position: &'a Result<usize, ExitReason>,
        stack: &'a Stack,
		memory: &'a Memory,
    },

    DebuggingWithoutOperand {
        opcode: Opcode,
        position: &'a Result<usize, ExitReason>,
        stack: &'a Stack,
		memory: &'a Memory,
    }
}

// Expose `listener::with` to allow flexible tracing.
pub fn with<F: FnOnce(&mut (dyn EventListener + 'static))>(f: F) {
	listener::with(f);
}

/// Run closure with provided listener.
pub fn using<R, F: FnOnce() -> R>(new: &mut (dyn EventListener + 'static), f: F) -> R {
	listener::using(new, f)
}
