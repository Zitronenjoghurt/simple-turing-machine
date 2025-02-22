use std::time::Duration;
use crate::compiler::layers::base::BaseLayer;
use crate::compiler::layers::pattern::PatternLayer;
use crate::compiler::layers::primitive::PrimitiveLayer;
use crate::compiler::layers::program_builder::ProgramBuilder;
use crate::compiler::structures::pattern::Pattern;
use crate::compiler::turing_compiler::TuringCompiler;
use crate::enums::display_style::DisplayStyle;
use crate::enums::movement::Movement;
use crate::machine::turing_machine::TuringMachine;
use crate::machine::turing_program::TuringProgram;

mod enums;
mod machine;
mod compiler;

fn main() {
    let current_programs = [
        build_mark_start_do_stuff_find_start()
    ];
    
    let mut tm = TuringMachine::default()
        .with_debug_mode(DisplayStyle::VisualFormal, Duration::from_millis(100));

    for program in current_programs {
        tm.set_program(program);
        tm.run_program();
        tm.reset_state_but_persist_tape();
    }
}

fn build_mark_start_do_stuff_find_start() -> TuringProgram {
    let start_pattern = Pattern::new(vec![true, true, false, true, true, false, true, true, false, true, true]);
    
    let mut compiler = TuringCompiler::default();
    
    let mark_start = compiler.allocate_state();
    let move_away = compiler.allocate_state();
    let mark_other_pattern = compiler.allocate_state();
    let find_start = compiler.allocate_state();
    let done = compiler.halt(None);
    
    compiler.write_pattern(start_pattern.clone(), Movement::Right, Movement::Right, Some(mark_start), Some(move_away));
    compiler.move_right_x(4, Some(move_away), Some(mark_other_pattern));
    compiler.write_pattern(Pattern::new(vec![true, true, false, true]), Movement::Right, Movement::Stay, Some(mark_other_pattern), Some(find_start));
    compiler.scan_pattern(start_pattern, Movement::Left, Movement::Stay, Some(find_start), Some(done));
    
    compiler.get_program()
}

fn build_set_bit_x_and_find_it_again(x: usize) -> TuringProgram {
    let mut compiler = TuringCompiler::default();

    let move_right_x = compiler.allocate_state();
    let set_one = compiler.allocate_state();
    let move_left_x = compiler.allocate_state();
    let scan_start = compiler.allocate_state();
    let done = compiler.allocate_state();

    compiler.move_right_x(x, Some(move_right_x), Some(set_one));
    compiler.mark(Some(set_one), Some(move_left_x));
    compiler.move_left_x(x, Some(move_left_x), Some(scan_start));
    compiler.scan_single(true, Movement::Right, Movement::Stay, Some(scan_start), Some(done));
    compiler.halt(Some(done));

    compiler.get_program()
}

fn build_set_bit_x_one(x: usize) -> TuringProgram {
    let mut compiler = TuringCompiler::default();

    // Less efficient but more intuitive variant:
    let move_right_x = compiler.allocate_state();
    let set_one = compiler.allocate_state();
    let move_left_x = compiler.allocate_state();
    let done = compiler.allocate_state();

    compiler.move_right_x(x, Some(move_right_x), Some(set_one));
    compiler.mark(Some(set_one), Some(move_left_x));
    compiler.move_left_x(x, Some(move_left_x), Some(done));
    compiler.halt(Some(done));

    // But if we write the program end to start, we can scrap the manual state allocation
    // let start_state = compiler.allocate_state();
    //
    // let done = compiler.halt(None);
    // let (move_left_x, _) = compiler.move_left_x(x, None, Some(done));
    // let (set_one, _) = compiler.mark(None, Some(move_left_x));
    // compiler.move_right_x(x, Some(start_state), Some(set_one));

    compiler.get_program()
}

fn build_move_right_till_one() -> TuringProgram {
    let mut compiler = TuringCompiler::default();

    let check_if_marked = compiler.allocate_state();
    let done = compiler.halt(None);

    compiler.branch_move(
        Some(check_if_marked), 
        Some(done), 
        Some(check_if_marked), 
        Movement::Stay, 
        Movement::Right
    );

    compiler.get_program()
}