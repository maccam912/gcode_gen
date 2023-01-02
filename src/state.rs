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
    hotend_temperature: i32,
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
            bed_temperature: 60,
            hotend_temperature: 200,
            fan: 255,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            e: 0.0,
            f: 1200.0,
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
        let state = State::new();
        let profile = Profile::new();
        Self::new(state, profile)
    }
}

impl Printer {
    /// Create a new instance of a Printer with sane default values and a couple commands to make sure printer is set to those defaults
    pub fn new(state: State, profile: Profile) -> Printer {
        let mut commands: Vec<Command> = vec![];
        // absolute_positioning_mode: true,
        if state.absolute_positioning_mode {
            // absolute_positioning_mode: true,
            commands.push(Command {
                gcode: Gcode::G90,
                comment: "Initialize with absolute positioning mode".into(),
            });
        } else {
            // absolute_positioning_mode: false,
            commands.push(Command {
                gcode: Gcode::G91,
                comment: "Initialize with relative positioning mode".into(),
            });
        }
        if state.extruder_absolute_positioning_mode {
            // extruder_absolute_positioning_mode: true,
            commands.push(Command {
                gcode: Gcode::M82,
                comment: "Initialize with extruder absolute positioning mode".into(),
            });
        } else {
            // extruder_absolute_positioning_mode: false,
            commands.push(Command {
                gcode: Gcode::M83,
                comment: "Initialize with extruder relative positioning mode".into(),
            });
        }
        // bed_temperature
        commands.push(Command { gcode: Gcode::M140(state.bed_temperature), comment: "Set bed temp but don't wait yet. First set hotend temp so they both warm at the same time".into() });
        // hotend_temperature
        commands.push(Command {
            gcode: Gcode::M104(state.hotend_temperature),
            comment: "Set hotend temp".into(),
        });

        commands.push(Command {
            gcode: Gcode::M190(state.bed_temperature),
            comment: "Wait for bed to come to temp".into(),
        });
        commands.push(Command {
            gcode: Gcode::M109(state.hotend_temperature),
            comment: "Waiti for hotend to come to temp".into(),
        });
        // fan
        commands.push(Command {
            gcode: Gcode::M106(state.fan),
            comment: "Set fan speed".into(),
        });
        // x, y, z, e, f
        commands.push(Command {
            gcode: Gcode::G0(state.e, state.f, state.x, state.y, state.z),
            comment: "Move to (x,y,z) with extruder and feed rate set to match state".into(),
        });
        Printer {
            state,
            commands,
            danger_mode: false,
            profile,
        }
    }

    /// Prepare for printing by auto-homing, setting temp, and drawing test line to get filament ready
    pub fn prepare(&mut self) {
        self.commands.push(Command {
            gcode: Gcode::G28,
            comment: "Auto-home".into(),
        });
        // we're moving 200 units up, then 200 back
        self.extrude_line(self.state.x, self.state.y + 200.0, self.state.z);
        self.extrude_line(
            self.state.x + self.profile.nozzle_diameter,
            self.state.y,
            self.state.z,
        );
        self.extrude_line(self.state.x, self.state.y - 200.0, self.state.z);
    }

    /// When done, retract filament a bit, move up and away from print, set temps back to 0 and fan to 0
    pub fn finish(&mut self) {
        self.retract(5.0);
        self.set_bed_temp_no_wait(0);
        self.set_hotend_temp_no_wait(0);
        self.set_fan_speed(0);
        self.move_without_extrusion(0.0, 200.0, self.state.z + 10.0);
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

    /// Stay in place but retract the filament a bit to avoid oozing
    pub fn retract(&mut self, amount: f32) {
        self.commands.push(Command {
            gcode: Gcode::G1(
                self.state.e - amount,
                self.state.f,
                self.state.x,
                self.state.y,
                self.state.z,
            ),
            comment: "retract filament".into(),
        });
        self.state.e -= amount;
    }

    /// Set the bed temperature and wait for temp to be reached
    pub fn set_bed_temp(&mut self, temp: i32) {
        self.commands.push(Command {
            gcode: Gcode::M190(temp),
            comment: "Set bed temp and wait".into(),
        });
        self.state.bed_temperature = temp;
    }

    /// Set the bed temperature and continue immediately without waiting for temp to be reached
    pub fn set_bed_temp_no_wait(&mut self, temp: i32) {
        self.commands.push(Command {
            gcode: Gcode::M140(temp),
            comment: "Set bed temp and do not wait".into(),
        });
        self.state.bed_temperature = temp;
    }

    /// Set the hotend temperature and wait for temp to be reached
    pub fn set_hotend_temp(&mut self, temp: i32) {
        self.commands.push(Command {
            gcode: Gcode::M109(temp),
            comment: "Set hotend temp and wait".into(),
        });
        self.state.hotend_temperature = temp;
    }

    /// Set the hotend temperature and continue immediately without waiting for temp to be reached
    pub fn set_hotend_temp_no_wait(&mut self, temp: i32) {
        self.commands.push(Command {
            gcode: Gcode::M104(temp),
            comment: "Set hotend temp and do not wait".into(),
        });
        self.state.hotend_temperature = temp;
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

    /// Move in line from current (x,y) to new (x,y), printing a line as you go
    pub fn extrude_line(&mut self, dest_x: f32, dest_y: f32, dest_z: f32) {
        if !self.state.absolute_positioning_mode {
            panic!("Only absolute positioning mode is supported at the moment.");
        }

        let dist = ((self.state.x - dest_x).abs().powi(2)
            + (self.state.y - dest_y).abs().powi(2)
            + (self.state.z - dest_z).abs().powi(2))
        .sqrt();
        let extrude_amount = dist * self.profile.extruder_ratio();
        self.commands.push(Command {
            gcode: Gcode::G1(
                self.state.e + extrude_amount,
                self.state.f,
                dest_x,
                dest_y,
                dest_z,
            ),
            comment: "Extrude line from current location to destination (x,y,z)".into(),
        });
        self.state.x = dest_x;
        self.state.y = dest_y;
        self.state.z = dest_z;
        self.state.e += extrude_amount;
    }

    pub fn move_without_extrusion(&mut self, dest_x: f32, dest_y: f32, dest_z: f32) {
        if !self.state.absolute_positioning_mode {
            panic!("Only absolute positioning mode is supported at the moment.");
        }

        self.commands.push(Command {
            gcode: Gcode::G1(self.state.e, self.state.f, dest_x, dest_y, dest_z),
            comment: "Move to (x,y,z) without extruding".into(),
        });

        self.state.x = dest_x;
        self.state.y = dest_y;
        self.state.z = dest_z;
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
    fn commands_exported_for_default() {
        let mut printer = Printer::default();
        printer.prepare();
        printer.finish();
        let str = printer.commands_str();
        let expected = include_str!("expected_default.gcode");
        debug_assert_eq!(str.trim(), expected.trim());
    }
}
