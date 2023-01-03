```console
$ weave
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
G1 E17.918858 F5000 X0.4 Y0 Z0.2 ; retract filament
G0 E17.918858 F5000 X20 Y20 Z0.3 ; Move to (x,y,z) without extruding
G1 E20.918858 F1500 X20 Y20 Z0.3 ; extrude filament
G1 E26.143347 F1500 X120 Y20 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E23.143347 F5000 X120 Y20 Z0.3 ; retract filament
G0 E23.143347 F5000 X20 Y20 Z0.3 ; Move to (x,y,z) without extruding
G1 E26.143347 F1500 X20 Y20 Z0.3 ; extrude filament
G1 E31.367836 F1500 X20 Y120 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E28.367836 F5000 X20 Y120 Z0.3 ; retract filament
G0 E28.367836 F5000 X20 Y21 Z0.3 ; Move to (x,y,z) without extruding
G1 E31.367836 F1500 X20 Y21 Z0.3 ; extrude filament
G1 E36.592327 F1500 X120 Y21 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E33.592327 F5000 X120 Y21 Z0.3 ; retract filament
G0 E33.592327 F5000 X21 Y20 Z0.3 ; Move to (x,y,z) without extruding
G1 E36.592327 F1500 X21 Y20 Z0.3 ; extrude filament
G1 E41.81682 F1500 X21 Y120 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E38.81682 F5000 X21 Y120 Z0.3 ; retract filament
G0 E38.81682 F5000 X20 Y22 Z0.3 ; Move to (x,y,z) without extruding
G1 E41.81682 F1500 X20 Y22 Z0.3 ; extrude filament
G1 E47.04131 F1500 X120 Y22 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E44.04131 F5000 X120 Y22 Z0.3 ; retract filament
G0 E44.04131 F5000 X22 Y20 Z0.3 ; Move to (x,y,z) without extruding
G1 E47.04131 F1500 X22 Y20 Z0.3 ; extrude filament
G1 E52.2658 F1500 X22 Y120 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E49.2658 F5000 X22 Y120 Z0.3 ; retract filament
G0 E49.2658 F5000 X20 Y23 Z0.3 ; Move to (x,y,z) without extruding
G1 E52.2658 F1500 X20 Y23 Z0.3 ; extrude filament
G1 E57.49029 F1500 X120 Y23 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E54.49029 F5000 X120 Y23 Z0.3 ; retract filament
G0 E54.49029 F5000 X23 Y20 Z0.3 ; Move to (x,y,z) without extruding
G1 E57.49029 F1500 X23 Y20 Z0.3 ; extrude filament
G1 E62.714783 F1500 X23 Y120 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E59.714783 F5000 X23 Y120 Z0.3 ; retract filament
G0 E59.714783 F5000 X20 Y24 Z0.3 ; Move to (x,y,z) without extruding
G1 E62.714783 F1500 X20 Y24 Z0.3 ; extrude filament
G1 E67.93927 F1500 X120 Y24 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E64.93927 F5000 X120 Y24 Z0.3 ; retract filament
G0 E64.93927 F5000 X24 Y20 Z0.3 ; Move to (x,y,z) without extruding
G1 E67.93927 F1500 X24 Y20 Z0.3 ; extrude filament
G1 E73.16376 F1500 X24 Y120 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E70.16376 F5000 X24 Y120 Z0.3 ; retract filament
G0 E70.16376 F5000 X20 Y25 Z0.3 ; Move to (x,y,z) without extruding
G1 E73.16376 F1500 X20 Y25 Z0.3 ; extrude filament
G1 E78.388245 F1500 X120 Y25 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E75.388245 F5000 X120 Y25 Z0.3 ; retract filament
G0 E75.388245 F5000 X25 Y20 Z0.3 ; Move to (x,y,z) without extruding
G1 E78.388245 F1500 X25 Y20 Z0.3 ; extrude filament
G1 E83.61273 F1500 X25 Y120 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E80.61273 F5000 X25 Y120 Z0.3 ; retract filament
G0 E80.61273 F5000 X20 Y26 Z0.3 ; Move to (x,y,z) without extruding
G1 E83.61273 F1500 X20 Y26 Z0.3 ; extrude filament
G1 E88.83722 F1500 X120 Y26 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E85.83722 F5000 X120 Y26 Z0.3 ; retract filament
G0 E85.83722 F5000 X26 Y20 Z0.3 ; Move to (x,y,z) without extruding
G1 E88.83722 F1500 X26 Y20 Z0.3 ; extrude filament
G1 E94.06171 F1500 X26 Y120 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E91.06171 F5000 X26 Y120 Z0.3 ; retract filament
G0 E91.06171 F5000 X20 Y27 Z0.3 ; Move to (x,y,z) without extruding
G1 E94.06171 F1500 X20 Y27 Z0.3 ; extrude filament
G1 E99.286194 F1500 X120 Y27 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E96.286194 F5000 X120 Y27 Z0.3 ; retract filament
G0 E96.286194 F5000 X27 Y20 Z0.3 ; Move to (x,y,z) without extruding
G1 E99.286194 F1500 X27 Y20 Z0.3 ; extrude filament
G1 E104.51068 F1500 X27 Y120 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E101.51068 F5000 X27 Y120 Z0.3 ; retract filament
G0 E101.51068 F5000 X20 Y28 Z0.3 ; Move to (x,y,z) without extruding
G1 E104.51068 F1500 X20 Y28 Z0.3 ; extrude filament
G1 E109.73517 F1500 X120 Y28 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E106.73517 F5000 X120 Y28 Z0.3 ; retract filament
G0 E106.73517 F5000 X28 Y20 Z0.3 ; Move to (x,y,z) without extruding
G1 E109.73517 F1500 X28 Y20 Z0.3 ; extrude filament
G1 E114.959656 F1500 X28 Y120 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E111.959656 F5000 X28 Y120 Z0.3 ; retract filament
G0 E111.959656 F5000 X20 Y29 Z0.3 ; Move to (x,y,z) without extruding
G1 E114.959656 F1500 X20 Y29 Z0.3 ; extrude filament
G1 E120.18414 F1500 X120 Y29 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E117.18414 F5000 X120 Y29 Z0.3 ; retract filament
G0 E117.18414 F5000 X29 Y20 Z0.3 ; Move to (x,y,z) without extruding
G1 E120.18414 F1500 X29 Y20 Z0.3 ; extrude filament
G1 E125.40863 F1500 X29 Y120 Z0.3 ; Extrude line from current location to destination (x,y,z)
G1 E122.40863 F5000 X29 Y120 Z0.3 ; retract filament
G1 E117.40863 F5000 X29 Y120 Z0.3 ; retract filament
M140 S0 ; Set bed temp and do not wait
M104 S0 ; Set hotend temp and do not wait
M106 S0 ; Set fan speed in 0-255 range
G0 E117.40863 F5000 X0 Y200 Z10.3 ; Move to (x,y,z) without extruding


```