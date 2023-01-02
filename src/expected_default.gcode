G90 ; Initialize with absolute positioning mode
M82 ; Initialize with extruder absolute positioning mode
M140 S60 ; Set bed temp but don't wait yet. First set hotend temp so they both warm at the same time
M104 S200 ; Set hotend temp
M190 S60 ; Wait for bed to come to temp
M109 S200 ; Waiti for hotend to come to temp
M106 S255 ; Set fan speed
G0 E0 F1200 X0 Y0 Z0 ; Move to (x,y,z) with extruder and feed rate set to match state
G28 ; Auto-home
G0 E0 F1200 X0 Y0 Z0.2 ; Move to (x,y,z) without extruding
G1 E10.448979 F1200 X0 Y200 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E10.469877 F1200 X0.4 Y200 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E20.918858 F1200 X0.4 Y0 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E15.918858 F1200 X0.4 Y0 Z0.2 ; retract filament
M140 S0 ; Set bed temp and do not wait
M104 S0 ; Set hotend temp and do not wait
M106 S0 ; Set fan speed in 0-255 range
G0 E15.918858 F1200 X0 Y200 Z10.2 ; Move to (x,y,z) without extruding