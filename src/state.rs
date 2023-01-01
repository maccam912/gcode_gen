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

pub struct Printer {
    state: State,
    commands: Vec<Command>,
    danger_mode: bool,
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
}