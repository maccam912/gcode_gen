use gcode_gen::state::Printer;

const GRID_SIZE: f32 = 1.0;

fn main() {
    let mut p = Printer::default();
    p.prepare();

    let z = 0.2;

    p.retract(2.0);

    p.move_without_extrusion(20.4, 0.0, z);
    p.extrude_line_relative(0.0, 200.0, z);
    p.retract(2.0);
    p.move_without_extrusion(20.0, 0.0, z);
    // Lines will alternate flat and raised going from x=0 to x=100.
    let mut row = 0;
    while p.state.y < 200.0 {
        if row % 2 == 0 {
            // even lines are flat
            p.move_without_extrusion(20.0, (row as f32) * GRID_SIZE, z);
            p.extrude_in_place(2.0);
            p.extrude_line_relative(100.0, 0.0, 0.0);
            p.retract(2.0);
        } else {
            // odd lines are raised. Both initially to not stick, and a kicker at the end to make it possible to weave vertical fibers
            p.move_without_extrusion(20.0, (row as f32) * GRID_SIZE, z);
            p.extrude_in_place(2.5); // Add a little dab to "glue" it down
            p.extrude_line_relative(89.44, 0.0, 10.0); // airborne so it doesn't stick
            p.move_without_extrusion(110.0, p.state.y, z); // lay it down
            p.extrude_line_relative(10.0, 0.0, 10.0); // Add kicker
            p.retract(2.0);
        }
        row += 1;
    }

    p.finish();

    println!("{}", p.commands_str());
}
