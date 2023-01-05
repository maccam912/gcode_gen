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
    /// M0(msec): Pause for number of milliseconds
    M0(i32),
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
            Gcode::G0(e, f, x, y, z) => write!(fmt, "G0 E{e} F{f} X{x} Y{y} Z{z}"),
            Gcode::G1(e, f, x, y, z) => write!(fmt, "G1 E{e} F{f} X{x} Y{y} Z{z}"),
            Gcode::G28 => write!(fmt, "G28"),
            Gcode::G90 => write!(fmt, "G90"),
            Gcode::G91 => write!(fmt, "G91"),
            Gcode::M0(msec) => write!(fmt, "M0 P{msec}"),
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
            Gcode::M82 => write!(fmt, "M82"),
            Gcode::M83 => write!(fmt, "M83"),
            Gcode::M104(temp) => write!(fmt, "M104 S{temp}"),
            Gcode::M106(power) => write!(fmt, "M106 S{power}"),
            Gcode::M107 => write!(fmt, "M107"),
            Gcode::M109(temp) => write!(fmt, "M109 S{temp}"),
            Gcode::M112 => write!(fmt, "M112"),
            Gcode::M140(temp) => write!(fmt, "M140 S{temp}"),
            Gcode::M190(temp) => write!(fmt, "M190 S{temp}"),
        }
    }
}

pub(crate) struct Command {
    pub(crate) gcode: Gcode,
    pub(crate) comment: String,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} ; {}", self.gcode, self.comment)
    }
}
