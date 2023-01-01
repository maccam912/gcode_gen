use std::f32::consts::PI;

use crate::command::{Command, Gcode};

pub struct State {
    /// `true` if absolute positioning mode is set, false if relative positioning mode is set
    absolute_positioning_mode: bool,
    /// `true` if extruder absolute positioning mode is set, false if extruder relative positioning mode is set
    extruder_absolute_positioning_mode: bool,
    /// Target bed temperature in C
    bed_temperature: i32,
    /// Hotend bed temperature in C
    hotend_temp: i32,
    /// Fan speed 0-255
    fan: u8,
    /// Hotend X coordinate
    x: f32,
    /// Hotend Y coordinate
    y: f32,
    /// Hotend Z coordinate
    z: f32,
    /// Hotend E coordinate (i.e. distance extruded)
    e: f32,
    /// Feed rate in mm/min
    f: f32,
}

impl State {
    /// Create a brand new state with absolute positioning of hotend and extruder, bed temp and hotend temp of 0, fan of 0, (x,y,z) at (0,0,0), e at 0, and f a 1200
    fn new() -> State {
        State {
            absolute_positioning_mode: true,
            extruder_absolute_positioning_mode: true,
            bed_temperature: 0,
            hotend_temp: 0,
            fan: 0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            e: 0.0,
            f: 0.0,
        }
    }
}

pub struct Profile {
    /// Diameter of nozzle in mm
    nozzle_diameter: f32,
    /// Diameter of filament in mm
    filament_diameter: f32,
}

impl Profile {
    fn new() -> Profile {
        Profile {
            nozzle_diameter: 0.4,
            filament_diameter: 1.75,
        }
    }

    fn extruder_ratio(&self) -> f32 {
        let filament_radius = self.filament_diameter / 2.0;
        let area_of_filament = PI * filament_radius * filament_radius;
        let nozzle_radius = self.nozzle_diameter / 2.0;
        let area_of_nozzle = PI * nozzle_radius * nozzle_radius;
        area_of_nozzle / area_of_filament
    }
}

pub struct Printer {
    state: State,
    commands: Vec<Command>,
    danger_mode: bool,
    profile: Profile,
}

impl Default for Printer {
    fn default() -> Self {
        Self::new()
    }
}

impl Printer {
    /// Create a new instance of a Printer with sane default values and an empty command list
    pub fn new() -> Printer {
        Printer {
            state: State::new(),
            commands: vec![],
            danger_mode: false,
            profile: Profile::new(),
        }
    }

    /// Lets you enable or disable danger mode. With danger mode enabled, checks that nothing bad happens will be run.
    pub fn set_danger_mode(&mut self, danger_mode: bool) {
        self.danger_mode = danger_mode;
    }

    /// Set either absolute (if absolute is `true`) or relative (if absolute is `false`) positioning mode
    pub fn set_absolute_positioning_mode(&mut self, absolute: bool) {
        if absolute {
            self.commands.push(Command {
                gcode: Gcode::G90,
                comment: "Setting absolute positioning mode".into(),
            });
        } else {
            self.commands.push(Command {
                gcode: Gcode::G91,
                comment: "Setting relative positioning mode".into(),
            });
        }
        self.state.absolute_positioning_mode = absolute;
    }

    /// Set either absolute (if absolute is `true`) or relative (if absolute is `false`) positioning mode for the extruder
    pub fn set_extruder_absolute_positioning_mode(&mut self, absolute: bool) {
        if absolute {
            self.commands.push(Command {
                gcode: Gcode::M82,
                comment: "Setting extruder absolute positioning mode".into(),
            });
        } else {
            self.commands.push(Command {
                gcode: Gcode::M83,
                comment: "Setting extruder relative positioning mode".into(),
            });
        }
        self.state.extruder_absolute_positioning_mode = absolute;
    }

    /// Set the bed temperature and wait for temp to be reached
    pub fn set_bed_temp(&mut self, temp: i32) {
        self.commands.push(Command {
            gcode: Gcode::M190(temp),
            comment: "Set bed temp and wait".into(),
        });
        self.state.bed_temperature = temp;
    }

    /// Set the hotend temperature and wait for temp to be reached
    pub fn set_hotend_temp(&mut self, temp: i32) {
        self.commands.push(Command {
            gcode: Gcode::M140(temp),
            comment: "Set hotend temp and wait".into(),
        });
        self.state.hotend_temp = temp;
    }

    /// Set the fan duty cycle from 0-255
    pub fn set_fan_speed(&mut self, speed: u8) {
        self.commands.push(Command {
            gcode: Gcode::M106(speed),
            comment: "Set fan speed in 0-255 range".into(),
        });
        self.state.fan = speed;
    }

    /// Add autohome command and move coordonates to (0,0,0)
    pub fn autohome(&mut self) {
        self.commands.push(Command {
            gcode: Gcode::G28,
            comment: "auto-home".into(),
        });
        self.state.x = 0.0;
        self.state.y = 0.0;
        self.state.z = 0.0;
    }

    pub fn extrude_line_absolute(&mut self, dest_x: f32, dest_y: f32) {
        let dist =
            ((self.state.x - dest_x).abs().powi(2) + (self.state.y - dest_y).abs().powi(2)).sqrt();
        let extrude_amount = dist * self.profile.extruder_ratio();
        self.commands.push(Command {
            gcode: Gcode::G1(
                self.state.e + extrude_amount,
                self.state.f,
                dest_x,
                dest_y,
                self.state.z,
            ),
            comment: "Extrude line from current location to destination x and y".into(),
        });
        self.state.x = dest_x;
        self.state.y = dest_y;
        self.state.e += extrude_amount;
    }

    pub fn commands_str(&self) -> String {
        let mut s = String::new();

        for c in &self.commands {
            s.push_str(&c.to_string());
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::state::Profile;

    use super::Printer;

    #[test]
    fn extruder_ratio() {
        let profile = Profile {
            filament_diameter: 1.75,
            nozzle_diameter: 0.4,
        };
        debug_assert!(profile.extruder_ratio() < 0.053);
        debug_assert!(profile.extruder_ratio() > 0.052);
    }

    #[test]
    fn commands_exported() {
        let mut printer = Printer::new();
        printer.autohome();
        let str = printer.commands_str();
        debug_assert_eq!(str, "G28 ; auto-home".to_string());
    }
}
