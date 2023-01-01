// https://marlinfw.org/meta/gcode/

use std::fmt::Display;

pub enum Gcode {
    /// G0(E, F, X, Y, Z): Performs a linear move, conventionally used for non-extruding move.
    G0(f32, f32, f32, f32, f32),
    /// G1(E, F, X, Y, Z): Performs a linear move, conventionally used for extruding move.
    G1(f32, f32, f32, f32, f32),
    /// G28: Auto-home
    G28,
    /// G90: Set absolute positioning mode
    G90,
    /// G91: Set relative positioning mode
    G91,
    /// M17(E, X, Y, Z): Enable steppers
    M17(bool, bool, bool, bool),
    /// M18(E, S, X, Y, Z): Disable steppers, after S idle timeout
    M18(bool, i32, bool, bool, bool),
    /// M82: Set E absolute positioning
    M82,
    /// M83: Set E relative positioning
    M83,
    /// M104(temp): Set hotend temperature, but don't wait for it to be reached
    M104(i32),
    /// M106(0-255): Set fan duty cycle (out of 255)
    M106(u8),
    /// M107: Fan off
    M107,
    /// M109(temp): Set hotend temperature and only continue when the target temp is reached
    M109(i32),
    /// M112: Emergency stop
    M112,
    /// M140(temp): Set bed temperature, but don't wait for it to be reached
    M140(i32),
    /// M190(temp): Set bed temperature and only continue when the target temp is reached
    M190(i32),
}

impl Display for Gcode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gcode::G0(e, f, x, y, z) => write!(fmt, "G0 E{e} f{f} X{x} Y{y} Z{z}"),
            Gcode::G1(e, f, x, y, z) => write!(fmt, "G1 E{e} f{f} X{x} Y{y} Z{z}"),
            Gcode::G28 => write!(fmt, "G28"),
            Gcode::G90 => write!(fmt, "G90"),
            Gcode::G91 => write!(fmt, "G91"),
            Gcode::M17(e, x, y, z) => {
                let mut s: String = "M17".into();
                if *e {
                    s.push_str(" E");
                }
                if *x {
                    s.push_str(" X");
                }
                if *y {
                    s.push_str(" Y");
                }
                if *z {
                    s.push_str(" Z");
                }
                write!(fmt, "{s}")
            }
            Gcode::M18(e, timeout, x, y, z) => {
                let mut s: String = "M18".into();
                if *e {
                    s.push_str(" E");
                }
                s.push_str(&format!("{timeout} "));
                if *x {
                    s.push_str(" X");
                }
                if *y {
                    s.push_str(" Y");
                }
                if *z {
                    s.push_str(" Z");
                }
                write!(fmt, "{s}")
            }
            Gcode::M82 => todo!(),
            Gcode::M83 => todo!(),
            Gcode::M104(_) => todo!(),
            Gcode::M106(_) => todo!(),
            Gcode::M107 => todo!(),
            Gcode::M109(_) => todo!(),
            Gcode::M112 => todo!(),
            Gcode::M140(_) => todo!(),
            Gcode::M190(_) => todo!(),
        }
    }
}

pub(crate) struct Command {
    pub(crate) gcode: Gcode,
    pub(crate) comment: String,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ; {}", self.gcode, self.comment)
    }
}
