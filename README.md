# gcode_gen

`gcode_gen` is intended to be a library to help you generate marlin gcode for 3d printers (https://marlinfw.org/meta/gcode/).

## Architecture

The main struct to use is `Printer`. It contains the state and lets you add commands to the list, or entire commonly-used sequences.

The main tasks you'll do:

- prep
  - auto-home
  - set absolute positioning mode
  - set temperatures of bed and hotend
  - put down line along edge to get filament flowing
  - set fan
- add sequence of commands
  - Both exturding (prefer G1) and non-extruding (prefer G0) commands
  - bump layer as needed
- end sequence
  - turn off fan
  - set temperatures to 0
  - move up and away from print
  - shut down steppers