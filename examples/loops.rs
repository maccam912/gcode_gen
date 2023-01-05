use gcode_gen::state::Printer;

fn main() {
    let mut p = Printer::default();
    p.prepare();

    let width = 100.0;
    let height = 100.0;

    let mut z = 0.35;
    // First draw base
    p.profile.extrusion_speed = 3000.0;
    p.move_without_extrusion(19.6, 19.6, z);
    p.extrude_line_relative(0.0, height + 0.8, 0.0);
    p.extrude_line_relative(width + 0.8, 0.0, 0.0);
    p.extrude_line_relative(0.0, -1.0 * height - 0.8, 0.0);
    p.extrude_line_relative(-1.0 * height - 0.8, 0.0, 0.0);
    p.extrude_line(20.0, 20.0, z);
    while p.state.x < 20.0 + width {
        p.extrude_line_relative(0.0, height, 0.0);
        p.extrude_line_relative(0.4, 0.0, 0.0);
        p.extrude_line_relative(0.0, -1.0 * height, 0.0);
        p.extrude_line_relative(0.4, 0.0, 0.0);
    }
    p.retract(3.0);
    z = 0.50;
    p.move_without_extrusion(p.state.x, p.state.y, z);
    p.move_without_extrusion(20.0, 20.0, z);

    p.profile.extrusion_speed = 50.0;

    p.set_bed_temp(35);

    let pattern_width = 1.5;

    let mut x = 20.0;
    while p.state.x < 20.0 + width - 6.0 {
        let y = 20.0;
        p.move_without_extrusion(x, y, z + 2.0);
        p.move_without_extrusion(x, y, z);
        p.extrude_in_place(3.0);
        while p.state.y < 20.0 + height - pattern_width {
            p.extrude_line_relative(-1.0, -1.0, 1.5);
            p.extrude_line_relative(1.0, 1.0, 1.5);
            p.profile.nozzle_diameter = 0.25;
            p.extrude_line_relative(1.5, 1.5, -3.0);
            p.profile.nozzle_diameter = 0.45;
            p.extrude_in_place(0.05);
            p.extrude_line_relative(1.0, -1.0, 1.5);
            p.extrude_line_relative(-1.0, 1.0, 1.5);
            p.profile.nozzle_diameter = 0.25;
            p.extrude_line_relative(-1.5, 1.5, -3.0);
            p.profile.nozzle_diameter = 0.45;
            p.extrude_in_place(0.05);
        }

        p.retract(3.0);
        p.move_without_extrusion(p.state.x, p.state.y, z + 2.0);
        x += 6.0;
    }

    p.finish();

    println!("{}", p.commands_str());
}
