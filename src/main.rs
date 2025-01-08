use std::time::Duration;
use crate::compiler::layers::primitive::PrimitiveLayer;
use crate::compiler::layers::program_builder::ProgramBuilder;
use crate::compiler::turing_compiler::TuringCompiler;
use crate::enums::display_style::DisplayStyle;
use crate::machine::turing_machine::TuringMachine;
use crate::machine::turing_tape::TuringTape;

mod enums;
mod machine;
mod compiler;

fn main() {
    let mut compiler = TuringCompiler::default();

    let check_if_marked = compiler.allocate_state();
    let move_to_next = compiler.allocate_state();
    let found_mark = compiler.allocate_state();

    compiler.branch(Some(check_if_marked), found_mark, move_to_next);
    compiler.move_right(Some(move_to_next), Some(check_if_marked));
    compiler.halt(Some(found_mark));
    
    let program = compiler.get_program();
    
    let mut manipulated_tape = TuringTape::new(2);
    manipulated_tape.set(13).unwrap();
    
    let mut tm = TuringMachine::new(program, 2)
        .with_tape(manipulated_tape)
        .with_debug_mode(DisplayStyle::VisualFormal, Duration::from_millis(500));
    
    tm.run_program()
}
