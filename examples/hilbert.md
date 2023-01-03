```console
$ hilbert
G90 ; Initialize with absolute positioning mode
M82 ; Initialize with extruder absolute positioning mode
M140 S60 ; Set bed temp but don't wait yet. First set hotend temp so they both warm at the same time
M104 S200 ; Set hotend temp
M190 S60 ; Wait for bed to come to temp
M109 S200 ; Waiti for hotend to come to temp
M106 S255 ; Set fan speed
G28 ; Auto home
G0 E0 F1200 X0 Y0 Z0 ; Move to (x,y,z) with extruder and feed rate set to match state
G28 ; Auto-home
G0 E0 F5000 X0 Y0 Z0.2 ; Move to (x,y,z) without extruding
G1 E10.448979 F1500 X0 Y200 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E10.469877 F1500 X0.4 Y200 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E20.918858 F1500 X0.4 Y0 Z0.2 ; Extrude line from current location to destination (x,y,z)
G0 E20.918858 F5000 X20 Y200 Z0.2 ; Move to (x,y,z) without extruding
G1 E21.023348 F1500 X20 Y198 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E21.127838 F1500 X22 Y198 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E21.232328 F1500 X22 Y200 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E21.336819 F1500 X24 Y200 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E21.441309 F1500 X26 Y200 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E21.5458 F1500 X26 Y198 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E21.65029 F1500 X24 Y198 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E21.75478 F1500 X24 Y196 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E21.85927 F1500 X26 Y196 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E21.96376 F1500 X26 Y194 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E22.06825 F1500 X24 Y194 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E22.17274 F1500 X22 Y194 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E22.277231 F1500 X22 Y196 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E22.381721 F1500 X20 Y196 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E22.486212 F1500 X20 Y194 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E22.590702 F1500 X20 Y192 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E22.695192 F1500 X22 Y192 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E22.799683 F1500 X22 Y190 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E22.904173 F1500 X20 Y190 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E23.008663 F1500 X20 Y188 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E23.113153 F1500 X20 Y186 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E23.217644 F1500 X22 Y186 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E23.322134 F1500 X22 Y188 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E23.426624 F1500 X24 Y188 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E23.531115 F1500 X24 Y186 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E23.635605 F1500 X26 Y186 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E23.740095 F1500 X26 Y188 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E23.844585 F1500 X26 Y190 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E23.949076 F1500 X24 Y190 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E24.053566 F1500 X24 Y192 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E24.158056 F1500 X26 Y192 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E24.262547 F1500 X28 Y192 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E24.367037 F1500 X30 Y192 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E24.471527 F1500 X30 Y190 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E24.576017 F1500 X28 Y190 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E24.680508 F1500 X28 Y188 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E24.784998 F1500 X28 Y186 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E24.889488 F1500 X30 Y186 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E24.993979 F1500 X30 Y188 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E25.098469 F1500 X32 Y188 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E25.20296 F1500 X32 Y186 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E25.30745 F1500 X34 Y186 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E25.41194 F1500 X34 Y188 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E25.51643 F1500 X34 Y190 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E25.62092 F1500 X32 Y190 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E25.72541 F1500 X32 Y192 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E25.8299 F1500 X34 Y192 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E25.934391 F1500 X34 Y194 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E26.038881 F1500 X34 Y196 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E26.143372 F1500 X32 Y196 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E26.247862 F1500 X32 Y194 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E26.352352 F1500 X30 Y194 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E26.456842 F1500 X28 Y194 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E26.561333 F1500 X28 Y196 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E26.665823 F1500 X30 Y196 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E26.770313 F1500 X30 Y198 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E26.874804 F1500 X28 Y198 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E26.979294 F1500 X28 Y200 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E27.083784 F1500 X30 Y200 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E27.188274 F1500 X32 Y200 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E27.292765 F1500 X32 Y198 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E27.397255 F1500 X34 Y198 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E27.501745 F1500 X34 Y200 Z0.2 ; Extrude line from current location to destination (x,y,z)
G1 E22.501745 F5000 X34 Y200 Z0.2 ; retract filament
M140 S0 ; Set bed temp and do not wait
M104 S0 ; Set hotend temp and do not wait
M106 S0 ; Set fan speed in 0-255 range
G0 E22.501745 F5000 X0 Y200 Z10.2 ; Move to (x,y,z) without extruding


```