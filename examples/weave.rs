use gcode_gen::state::Printer;

const GRID_SIZE: f32 = 1.0;

fn main() {
    let mut p = Printer::default();
    p.prepare();

    let z = 0.3;

    p.retract(3.0);
    // Lines will alternate flat and raised going from x=0 to x=100.
    for i in 0..10 {
        p.move_without_extrusion(20.0, (i as f32) * GRID_SIZE + 20.0, z);
        p.extrude_in_place(3.0);
        p.extrude_line_relative(100.0, 0.0, 0.0);
        p.retract(3.0);
        p.move_without_extrusion((i as f32) * GRID_SIZE + 20.0, 20.0, z);
        p.extrude_in_place(3.0);
        p.extrude_line_relative(0.0, 100.0, 0.0);
        p.retract(3.0);
    }

    p.finish();

    println!("{}", p.commands_str());
}
