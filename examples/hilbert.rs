use gcode_gen::state::Printer;

const GRID_SIZE: f32 = 2.0;

fn go(p: &mut Printer, direction: Dir) {
    match direction {
        Dir::UP => p.extrude_line_relative(0.0, GRID_SIZE, 0.0),
        Dir::DOWN => p.extrude_line_relative(0.0, GRID_SIZE * -1.0, 0.0),
        Dir::RIGHT => p.extrude_line_relative(GRID_SIZE, 0.0, 0.0),
        Dir::LEFT => p.extrude_line_relative(GRID_SIZE * -1.0, 0.0, 0.0),
    }
}

enum Dir {
    UP,
    LEFT,
    DOWN,
    RIGHT,
}

/// Recursively draw a hilbert curve `level` levels deep
fn hilbert(p: &mut Printer, level: u32, direction: Dir) {
    if level == 1 {
        match direction {
            Dir::LEFT => {
                go(p, Dir::RIGHT);
                go(p, Dir::DOWN);
                go(p, Dir::LEFT);
            }
            Dir::RIGHT => {
                go(p, Dir::LEFT);
                go(p, Dir::UP);
                go(p, Dir::RIGHT);
            }
            Dir::UP => {
                go(p, Dir::DOWN);
                go(p, Dir::RIGHT);
                go(p, Dir::UP);
            }
            Dir::DOWN => {
                go(p, Dir::UP);
                go(p, Dir::LEFT);
                go(p, Dir::DOWN);
            }
        }
    } else {
        // level is not 1, recurse
        match direction {
            Dir::LEFT => {
                hilbert(p, level - 1, Dir::UP);
                go(p, Dir::RIGHT);
                hilbert(p, level - 1, Dir::LEFT);
                go(p, Dir::DOWN);
                hilbert(p, level - 1, Dir::LEFT);
                go(p, Dir::LEFT);
                hilbert(p, level - 1, Dir::DOWN);
            }
            Dir::RIGHT => {
                hilbert(p, level - 1, Dir::DOWN);
                go(p, Dir::LEFT);
                hilbert(p, level - 1, Dir::RIGHT);
                go(p, Dir::UP);
                hilbert(p, level - 1, Dir::RIGHT);
                go(p, Dir::RIGHT);
                hilbert(p, level - 1, Dir::UP);
            }
            Dir::UP => {
                hilbert(p, level - 1, Dir::LEFT);
                go(p, Dir::DOWN);
                hilbert(p, level - 1, Dir::UP);
                go(p, Dir::RIGHT);
                hilbert(p, level - 1, Dir::UP);
                go(p, Dir::UP);
                hilbert(p, level - 1, Dir::RIGHT);
            }
            Dir::DOWN => {
                hilbert(p, level - 1, Dir::RIGHT);
                go(p, Dir::UP);
                hilbert(p, level - 1, Dir::DOWN);
                go(p, Dir::LEFT);
                hilbert(p, level - 1, Dir::DOWN);
                go(p, Dir::DOWN);
                hilbert(p, level - 1, Dir::LEFT);
            }
        }
    }
}

/// When this is run it will print some gcode that would draw a level-6 hilbert curve
fn main() {
    let mut p = Printer::default();
    p.prepare();

    p.move_without_extrusion(20.0, 200.0, 0.2);
    // Now draw the lines
    hilbert(&mut p, 3, Dir::UP);

    p.finish();

    println!("{}", p.commands_str());
}
