```console
$ loops 
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
G0 E20.918858 F5000 X19.6 Y19.6 Z0.35 ; Move to (x,y,z) without extruding
G1 E26.185143 F3000 X19.6 Y120.4 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E31.451427 F3000 X120.4 Y120.4 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E36.717712 F3000 X120.4 Y19.599998 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E41.983997 F3000 X19.599998 Y19.599998 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E42.01355 F3000 X20 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E47.23804 F3000 X20 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E47.258938 F3000 X20.4 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E52.48343 F3000 X20.4 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E52.504326 F3000 X20.8 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E57.728817 F3000 X20.8 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E57.749714 F3000 X21.199999 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E62.974205 F3000 X21.199999 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E62.995102 F3000 X21.599998 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E68.21959 F3000 X21.599998 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E68.240486 F3000 X21.999998 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E73.46497 F3000 X21.999998 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E73.48587 F3000 X22.399998 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E78.71036 F3000 X22.399998 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E78.731255 F3000 X22.799997 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E83.95574 F3000 X22.799997 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E83.97664 F3000 X23.199997 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E89.201126 F3000 X23.199997 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E89.22202 F3000 X23.599997 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E94.44651 F3000 X23.599997 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E94.46741 F3000 X23.999996 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E99.691895 F3000 X23.999996 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E99.71279 F3000 X24.399996 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E104.93728 F3000 X24.399996 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E104.958176 F3000 X24.799995 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E110.18266 F3000 X24.799995 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E110.20356 F3000 X25.199995 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E115.42805 F3000 X25.199995 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E115.448944 F3000 X25.599995 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E120.67343 F3000 X25.599995 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E120.69433 F3000 X25.999994 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E125.918816 F3000 X25.999994 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E125.93971 F3000 X26.399994 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E131.1642 F3000 X26.399994 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E131.1851 F3000 X26.799994 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E136.40959 F3000 X26.799994 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E136.4305 F3000 X27.199993 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E141.65498 F3000 X27.199993 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E141.67589 F3000 X27.599993 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E146.90038 F3000 X27.599993 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E146.92128 F3000 X27.999992 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E152.14577 F3000 X27.999992 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E152.16667 F3000 X28.399992 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E157.39116 F3000 X28.399992 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E157.41206 F3000 X28.799992 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E162.63655 F3000 X28.799992 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E162.65746 F3000 X29.199991 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E167.88194 F3000 X29.199991 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E167.90285 F3000 X29.59999 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E173.12733 F3000 X29.59999 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E173.14824 F3000 X29.99999 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E178.37273 F3000 X29.99999 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E178.39363 F3000 X30.39999 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E183.61812 F3000 X30.39999 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E183.63902 F3000 X30.79999 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E188.86351 F3000 X30.79999 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E188.88441 F3000 X31.19999 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E194.1089 F3000 X31.19999 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E194.1298 F3000 X31.599989 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E199.3543 F3000 X31.599989 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E199.3752 F3000 X31.999989 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E204.59969 F3000 X31.999989 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E204.62059 F3000 X32.39999 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E209.84508 F3000 X32.39999 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E209.86598 F3000 X32.79999 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E215.09047 F3000 X32.79999 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E215.11137 F3000 X33.199993 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E220.33586 F3000 X33.199993 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E220.35677 F3000 X33.599995 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E225.58125 F3000 X33.599995 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E225.60216 F3000 X33.999996 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E230.82664 F3000 X33.999996 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E230.84755 F3000 X34.399998 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E236.07204 F3000 X34.399998 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E236.09294 F3000 X34.8 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E241.31743 F3000 X34.8 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E241.33833 F3000 X35.2 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E246.56282 F3000 X35.2 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E246.58372 F3000 X35.600002 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E251.80821 F3000 X35.600002 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E251.82912 F3000 X36.000004 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E257.05362 F3000 X36.000004 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E257.07452 F3000 X36.400005 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E262.299 F3000 X36.400005 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E262.31992 F3000 X36.800007 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E267.5444 F3000 X36.800007 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E267.5653 F3000 X37.20001 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E272.7898 F3000 X37.20001 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E272.8107 F3000 X37.60001 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E278.0352 F3000 X37.60001 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E278.0561 F3000 X38.00001 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E283.28058 F3000 X38.00001 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E283.30148 F3000 X38.400013 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E288.52597 F3000 X38.400013 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E288.54688 F3000 X38.800014 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E293.77136 F3000 X38.800014 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E293.79227 F3000 X39.200016 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E299.01675 F3000 X39.200016 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E299.03766 F3000 X39.600018 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E304.26215 F3000 X39.600018 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E304.28305 F3000 X40.00002 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E309.50754 F3000 X40.00002 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E309.52844 F3000 X40.40002 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E314.75293 F3000 X40.40002 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E314.77383 F3000 X40.800022 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E319.99832 F3000 X40.800022 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E320.01923 F3000 X41.200024 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E325.2437 F3000 X41.200024 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E325.26462 F3000 X41.600025 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E330.4891 F3000 X41.600025 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E330.51 F3000 X42.000027 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E335.7345 F3000 X42.000027 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E335.7554 F3000 X42.40003 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E340.9799 F3000 X42.40003 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E341.0008 F3000 X42.80003 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E346.22528 F3000 X42.80003 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E346.2462 F3000 X43.20003 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E351.47067 F3000 X43.20003 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E351.49158 F3000 X43.600033 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E356.71606 F3000 X43.600033 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E356.73697 F3000 X44.000034 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E361.96146 F3000 X44.000034 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E361.98236 F3000 X44.400036 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E367.20685 F3000 X44.400036 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E367.22775 F3000 X44.800037 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E372.45224 F3000 X44.800037 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E372.47314 F3000 X45.20004 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E377.69763 F3000 X45.20004 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E377.71854 F3000 X45.60004 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E382.94302 F3000 X45.60004 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E382.96393 F3000 X46.000042 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E388.18842 F3000 X46.000042 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E388.20932 F3000 X46.400043 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E393.4338 F3000 X46.400043 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E393.4547 F3000 X46.800045 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E398.6792 F3000 X46.800045 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E398.7001 F3000 X47.200047 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E403.9246 F3000 X47.200047 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E403.9455 F3000 X47.600048 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E409.16998 F3000 X47.600048 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E409.1909 F3000 X48.00005 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E414.41537 F3000 X48.00005 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E414.43628 F3000 X48.40005 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E419.66077 F3000 X48.40005 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E419.68167 F3000 X48.800053 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E424.90616 F3000 X48.800053 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E424.92706 F3000 X49.200054 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E430.15155 F3000 X49.200054 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E430.17245 F3000 X49.600056 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E435.39694 F3000 X49.600056 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E435.41785 F3000 X50.000057 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E440.64233 F3000 X50.000057 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E440.66324 F3000 X50.40006 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E445.88773 F3000 X50.40006 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E445.90863 F3000 X50.80006 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E451.13312 F3000 X50.80006 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E451.15402 F3000 X51.20006 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E456.3785 F3000 X51.20006 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E456.3994 F3000 X51.600063 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E461.6239 F3000 X51.600063 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E461.6448 F3000 X52.000065 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E466.8693 F3000 X52.000065 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E466.8902 F3000 X52.400066 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E472.1147 F3000 X52.400066 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E472.1356 F3000 X52.800068 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E477.36008 F3000 X52.800068 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E477.38098 F3000 X53.20007 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E482.60547 F3000 X53.20007 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E482.62637 F3000 X53.60007 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E487.85086 F3000 X53.60007 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E487.87177 F3000 X54.000072 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E493.09625 F3000 X54.000072 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E493.11716 F3000 X54.400074 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E498.34164 F3000 X54.400074 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E498.36255 F3000 X54.800076 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E503.58704 F3000 X54.800076 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E503.60794 F3000 X55.200077 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E508.83243 F3000 X55.200077 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E508.85333 F3000 X55.60008 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E514.0778 F3000 X55.60008 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E514.0987 F3000 X56.00008 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E519.3232 F3000 X56.00008 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E519.34406 F3000 X56.40008 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E524.56854 F3000 X56.40008 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E524.5894 F3000 X56.800083 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E529.8139 F3000 X56.800083 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E529.8348 F3000 X57.200085 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E535.05927 F3000 X57.200085 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E535.08014 F3000 X57.600086 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E540.3046 F3000 X57.600086 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E540.3255 F3000 X58.000088 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E545.55 F3000 X58.000088 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E545.57086 F3000 X58.40009 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E550.79535 F3000 X58.40009 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E550.8162 F3000 X58.80009 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E556.0407 F3000 X58.80009 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E556.0616 F3000 X59.200092 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E561.2861 F3000 X59.200092 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E561.30695 F3000 X59.600094 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E566.53143 F3000 X59.600094 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E566.5523 F3000 X60.000095 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E571.7768 F3000 X60.000095 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E571.79767 F3000 X60.400097 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E577.02216 F3000 X60.400097 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E577.043 F3000 X60.8001 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E582.2675 F3000 X60.8001 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E582.2884 F3000 X61.2001 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E587.5129 F3000 X61.2001 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E587.53375 F3000 X61.6001 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E592.75824 F3000 X61.6001 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E592.7791 F3000 X62.000103 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E598.0036 F3000 X62.000103 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E598.0245 F3000 X62.400105 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E603.24896 F3000 X62.400105 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E603.26984 F3000 X62.800106 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E608.4943 F3000 X62.800106 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E608.5152 F3000 X63.200108 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E613.7397 F3000 X63.200108 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E613.76056 F3000 X63.60011 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E618.98505 F3000 X63.60011 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E619.0059 F3000 X64.00011 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E624.2304 F3000 X64.00011 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E624.2513 F3000 X64.40011 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E629.47577 F3000 X64.40011 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E629.49664 F3000 X64.80011 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E634.7211 F3000 X64.80011 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E634.742 F3000 X65.20011 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E639.9665 F3000 X65.20011 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E639.98737 F3000 X65.60011 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E645.21185 F3000 X65.60011 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E645.2327 F3000 X66.000114 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E650.4572 F3000 X66.000114 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E650.4781 F3000 X66.400116 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E655.7026 F3000 X66.400116 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E655.72345 F3000 X66.80012 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E660.94794 F3000 X66.80012 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E660.9688 F3000 X67.20012 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E666.1933 F3000 X67.20012 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E666.2142 F3000 X67.60012 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E671.43866 F3000 X67.60012 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E671.45953 F3000 X68.00012 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E676.684 F3000 X68.00012 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E676.7049 F3000 X68.40012 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E681.9294 F3000 X68.40012 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E681.95026 F3000 X68.800125 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E687.17474 F3000 X68.800125 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E687.1956 F3000 X69.20013 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E692.4201 F3000 X69.20013 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E692.441 F3000 X69.60013 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E697.66547 F3000 X69.60013 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E697.68634 F3000 X70.00013 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E702.9108 F3000 X70.00013 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E702.9317 F3000 X70.40013 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E708.1562 F3000 X70.40013 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E708.17706 F3000 X70.80013 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E713.40155 F3000 X70.80013 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E713.4224 F3000 X71.200134 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E718.6469 F3000 X71.200134 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E718.6678 F3000 X71.600136 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E723.8923 F3000 X71.600136 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E723.91315 F3000 X72.00014 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E729.13763 F3000 X72.00014 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E729.1585 F3000 X72.40014 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E734.383 F3000 X72.40014 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E734.4039 F3000 X72.80014 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E739.62836 F3000 X72.80014 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E739.64923 F3000 X73.20014 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E744.8737 F3000 X73.20014 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E744.8946 F3000 X73.60014 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E750.1191 F3000 X73.60014 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E750.13995 F3000 X74.000145 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E755.36444 F3000 X74.000145 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E755.3853 F3000 X74.40015 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E760.6098 F3000 X74.40015 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E760.6307 F3000 X74.80015 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E765.85516 F3000 X74.80015 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E765.87604 F3000 X75.20015 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E771.1005 F3000 X75.20015 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E771.1214 F3000 X75.60015 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E776.3459 F3000 X75.60015 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E776.36676 F3000 X76.00015 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E781.59125 F3000 X76.00015 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E781.6121 F3000 X76.400154 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E786.8366 F3000 X76.400154 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E786.8575 F3000 X76.800156 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E792.082 F3000 X76.800156 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E792.10284 F3000 X77.20016 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E797.32733 F3000 X77.20016 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E797.3482 F3000 X77.60016 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E802.5727 F3000 X77.60016 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E802.59357 F3000 X78.00016 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E807.81805 F3000 X78.00016 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E807.8389 F3000 X78.40016 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E813.0634 F3000 X78.40016 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E813.0843 F3000 X78.80016 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E818.3088 F3000 X78.80016 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E818.32965 F3000 X79.200165 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E823.55414 F3000 X79.200165 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E823.575 F3000 X79.60017 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E828.7995 F3000 X79.60017 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E828.8204 F3000 X80.00017 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E834.04486 F3000 X80.00017 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E834.06573 F3000 X80.40017 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E839.2902 F3000 X80.40017 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E839.3111 F3000 X80.80017 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E844.5356 F3000 X80.80017 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E844.55646 F3000 X81.20017 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E849.78094 F3000 X81.20017 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E849.8018 F3000 X81.600174 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E855.0263 F3000 X81.600174 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E855.0472 F3000 X82.000175 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E860.27167 F3000 X82.000175 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E860.29254 F3000 X82.40018 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E865.517 F3000 X82.40018 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E865.5379 F3000 X82.80018 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E870.7624 F3000 X82.80018 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E870.78326 F3000 X83.20018 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E876.00775 F3000 X83.20018 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E876.0286 F3000 X83.60018 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E881.2531 F3000 X83.60018 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E881.274 F3000 X84.00018 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E886.4985 F3000 X84.00018 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E886.51935 F3000 X84.400185 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E891.74384 F3000 X84.400185 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E891.7647 F3000 X84.800186 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E896.9892 F3000 X84.800186 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E897.0101 F3000 X85.20019 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E902.23456 F3000 X85.20019 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E902.25543 F3000 X85.60019 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E907.4799 F3000 X85.60019 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E907.5008 F3000 X86.00019 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E912.7253 F3000 X86.00019 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E912.74615 F3000 X86.40019 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E917.97064 F3000 X86.40019 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E917.9915 F3000 X86.80019 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E923.216 F3000 X86.80019 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E923.2369 F3000 X87.200195 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E928.46136 F3000 X87.200195 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E928.48224 F3000 X87.6002 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E933.7067 F3000 X87.6002 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E933.7276 F3000 X88.0002 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E938.9521 F3000 X88.0002 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E938.97296 F3000 X88.4002 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E944.19745 F3000 X88.4002 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E944.2183 F3000 X88.8002 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E949.4428 F3000 X88.8002 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E949.4637 F3000 X89.2002 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E954.6882 F3000 X89.2002 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E954.70905 F3000 X89.600204 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E959.93353 F3000 X89.600204 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E959.9544 F3000 X90.000206 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E965.1789 F3000 X90.000206 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E965.19977 F3000 X90.40021 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E970.42426 F3000 X90.40021 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E970.4451 F3000 X90.80021 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E975.6696 F3000 X90.80021 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E975.6905 F3000 X91.20021 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E980.915 F3000 X91.20021 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E980.93585 F3000 X91.60021 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E986.16034 F3000 X91.60021 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E986.1812 F3000 X92.00021 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E991.4057 F3000 X92.00021 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E991.4266 F3000 X92.400215 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E996.65106 F3000 X92.400215 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E996.67194 F3000 X92.80022 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1001.8964 F3000 X92.80022 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1001.9173 F3000 X93.20022 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1007.1418 F3000 X93.20022 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1007.16266 F3000 X93.60022 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1012.38715 F3000 X93.60022 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1012.408 F3000 X94.00022 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1017.6325 F3000 X94.00022 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1017.6534 F3000 X94.40022 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1022.87787 F3000 X94.40022 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1022.89874 F3000 X94.800224 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1028.1233 F3000 X94.800224 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1028.1442 F3000 X95.200226 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1033.3687 F3000 X95.200226 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1033.3895 F3000 X95.60023 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1038.614 F3000 X95.60023 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1038.6349 F3000 X96.00023 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1043.8594 F3000 X96.00023 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1043.8802 F3000 X96.40023 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1049.1047 F3000 X96.40023 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1049.1256 F3000 X96.80023 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1054.3501 F3000 X96.80023 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1054.371 F3000 X97.20023 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1059.5955 F3000 X97.20023 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1059.6163 F3000 X97.600235 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1064.8408 F3000 X97.600235 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1064.8617 F3000 X98.00024 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1070.0862 F3000 X98.00024 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1070.107 F3000 X98.40024 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1075.3315 F3000 X98.40024 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1075.3524 F3000 X98.80024 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1080.5769 F3000 X98.80024 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1080.5978 F3000 X99.20024 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1085.8223 F3000 X99.20024 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1085.8431 F3000 X99.60024 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1091.0676 F3000 X99.60024 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1091.0885 F3000 X100.000244 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1096.313 F3000 X100.000244 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1096.3339 F3000 X100.400246 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1101.5583 F3000 X100.400246 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1101.5792 F3000 X100.80025 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1106.8037 F3000 X100.80025 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1106.8246 F3000 X101.20025 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1112.0491 F3000 X101.20025 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1112.07 F3000 X101.60025 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1117.2944 F3000 X101.60025 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1117.3153 F3000 X102.00025 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1122.5398 F3000 X102.00025 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1122.5607 F3000 X102.40025 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1127.7852 F3000 X102.40025 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1127.806 F3000 X102.800255 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1133.0305 F3000 X102.800255 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1133.0514 F3000 X103.20026 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1138.2759 F3000 X103.20026 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1138.2968 F3000 X103.60026 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1143.5212 F3000 X103.60026 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1143.5421 F3000 X104.00026 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1148.7666 F3000 X104.00026 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1148.7875 F3000 X104.40026 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1154.012 F3000 X104.40026 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1154.0328 F3000 X104.80026 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1159.2573 F3000 X104.80026 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1159.2782 F3000 X105.200264 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1164.5027 F3000 X105.200264 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1164.5236 F3000 X105.600266 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1169.748 F3000 X105.600266 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1169.7689 F3000 X106.00027 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1174.9934 F3000 X106.00027 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1175.0143 F3000 X106.40027 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1180.2388 F3000 X106.40027 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1180.2596 F3000 X106.80027 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1185.4841 F3000 X106.80027 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1185.505 F3000 X107.20027 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1190.7295 F3000 X107.20027 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1190.7504 F3000 X107.60027 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1195.9749 F3000 X107.60027 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1195.9957 F3000 X108.000275 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1201.2202 F3000 X108.000275 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1201.2411 F3000 X108.400276 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1206.4656 F3000 X108.400276 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1206.4865 F3000 X108.80028 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1211.7109 F3000 X108.80028 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1211.7318 F3000 X109.20028 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1216.9563 F3000 X109.20028 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1216.9772 F3000 X109.60028 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1222.2017 F3000 X109.60028 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1222.2225 F3000 X110.00028 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1227.447 F3000 X110.00028 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1227.4679 F3000 X110.40028 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1232.6924 F3000 X110.40028 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1232.7133 F3000 X110.800285 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1237.9377 F3000 X110.800285 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1237.9586 F3000 X111.20029 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1243.1831 F3000 X111.20029 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1243.204 F3000 X111.60029 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1248.4285 F3000 X111.60029 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1248.4493 F3000 X112.00029 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1253.6738 F3000 X112.00029 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1253.6947 F3000 X112.40029 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1258.9192 F3000 X112.40029 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1258.9401 F3000 X112.80029 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1264.1646 F3000 X112.80029 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1264.1854 F3000 X113.200294 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1269.4099 F3000 X113.200294 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1269.4308 F3000 X113.600296 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1274.6553 F3000 X113.600296 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1274.6761 F3000 X114.0003 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1279.9006 F3000 X114.0003 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1279.9215 F3000 X114.4003 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1285.146 F3000 X114.4003 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1285.1669 F3000 X114.8003 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1290.3914 F3000 X114.8003 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1290.4122 F3000 X115.2003 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1295.6367 F3000 X115.2003 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1295.6576 F3000 X115.6003 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1300.8821 F3000 X115.6003 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1300.903 F3000 X116.000305 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1306.1274 F3000 X116.000305 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1306.1483 F3000 X116.40031 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1311.3728 F3000 X116.40031 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1311.3937 F3000 X116.80031 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1316.6182 F3000 X116.80031 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1316.639 F3000 X117.20031 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1321.8635 F3000 X117.20031 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1321.8844 F3000 X117.60031 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1327.1089 F3000 X117.60031 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1327.1298 F3000 X118.00031 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1332.3542 F3000 X118.00031 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1332.3751 F3000 X118.400314 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1337.5996 F3000 X118.400314 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1337.6205 F3000 X118.800316 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1342.845 F3000 X118.800316 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1342.8658 F3000 X119.20032 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1348.0903 F3000 X119.20032 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1348.1112 F3000 X119.60032 Y120 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1353.3357 F3000 X119.60032 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1353.3566 F3000 X120.00032 Y20 Z0.35 ; Extrude line from current location to destination (x,y,z)
G1 E1350.3566 F5000 X120.00032 Y20 Z0.35 ; retract filament
G0 E1350.3566 F5000 X120.00032 Y20 Z0.5 ; Move to (x,y,z) without extruding
G0 E1350.3566 F5000 X20 Y20 Z0.5 ; Move to (x,y,z) without extruding
M190 S45 ; Set bed temp and wait
G0 E1350.3566 F5000 X20 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1350.3566 F5000 X20 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1353.3566 F50 X20 Y20 Z0.5 ; extrude filament
G1 E1353.4642 F50 X19 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1353.5719 F50 X20 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1353.6469 F50 X21.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1353.6969 F50 X21.5 Y21.5 Z0.5 ; extrude filament
G1 E1353.8333 F50 X22.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1353.9696 F50 X21.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1354.0446 F50 X20 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1354.0946 F50 X20 Y23 Z0.5 ; extrude filament
G1 E1354.231 F50 X19 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1354.3673 F50 X20 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1354.4423 F50 X21.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1354.4923 F50 X21.5 Y24.5 Z0.5 ; extrude filament
G1 E1354.6287 F50 X22.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1354.765 F50 X21.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1354.84 F50 X20 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1354.89 F50 X20 Y26 Z0.5 ; extrude filament
G1 E1355.0264 F50 X19 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1355.1627 F50 X20 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1355.2377 F50 X21.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1355.2877 F50 X21.5 Y27.5 Z0.5 ; extrude filament
G1 E1355.4241 F50 X22.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1355.5604 F50 X21.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1355.6354 F50 X20 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1355.6854 F50 X20 Y29 Z0.5 ; extrude filament
G1 E1355.8218 F50 X19 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1355.9581 F50 X20 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1356.0331 F50 X21.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1356.0831 F50 X21.5 Y30.5 Z0.5 ; extrude filament
G1 E1356.2195 F50 X22.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1356.3558 F50 X21.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1356.4308 F50 X20 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1356.4808 F50 X20 Y32 Z0.5 ; extrude filament
G1 E1356.6172 F50 X19 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1356.7535 F50 X20 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1356.8285 F50 X21.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1356.8785 F50 X21.5 Y33.5 Z0.5 ; extrude filament
G1 E1357.0149 F50 X22.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1357.1512 F50 X21.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1357.2262 F50 X20 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1357.2762 F50 X20 Y35 Z0.5 ; extrude filament
G1 E1357.4126 F50 X19 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1357.549 F50 X20 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1357.6239 F50 X21.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1357.674 F50 X21.5 Y36.5 Z0.5 ; extrude filament
G1 E1357.8103 F50 X22.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1357.9467 F50 X21.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1358.0216 F50 X20 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1358.0717 F50 X20 Y38 Z0.5 ; extrude filament
G1 E1358.208 F50 X19 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1358.3444 F50 X20 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1358.4193 F50 X21.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1358.4694 F50 X21.5 Y39.5 Z0.5 ; extrude filament
G1 E1358.6057 F50 X22.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1358.7421 F50 X21.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1358.817 F50 X20 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1358.8671 F50 X20 Y41 Z0.5 ; extrude filament
G1 E1359.0034 F50 X19 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1359.1398 F50 X20 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1359.2147 F50 X21.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1359.2648 F50 X21.5 Y42.5 Z0.5 ; extrude filament
G1 E1359.4011 F50 X22.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1359.5375 F50 X21.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1359.6124 F50 X20 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1359.6625 F50 X20 Y44 Z0.5 ; extrude filament
G1 E1359.7988 F50 X19 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1359.9352 F50 X20 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1360.0101 F50 X21.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1360.0602 F50 X21.5 Y45.5 Z0.5 ; extrude filament
G1 E1360.1965 F50 X22.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1360.3329 F50 X21.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1360.4078 F50 X20 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1360.4579 F50 X20 Y47 Z0.5 ; extrude filament
G1 E1360.5942 F50 X19 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1360.7306 F50 X20 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1360.8055 F50 X21.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1360.8556 F50 X21.5 Y48.5 Z0.5 ; extrude filament
G1 E1360.992 F50 X22.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1361.1283 F50 X21.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1361.2032 F50 X20 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1361.2533 F50 X20 Y50 Z0.5 ; extrude filament
G1 E1361.3896 F50 X19 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1361.526 F50 X20 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1361.601 F50 X21.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1361.651 F50 X21.5 Y51.5 Z0.5 ; extrude filament
G1 E1361.7874 F50 X22.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1361.9237 F50 X21.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1361.9987 F50 X20 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1362.0487 F50 X20 Y53 Z0.5 ; extrude filament
G1 E1362.185 F50 X19 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1362.3214 F50 X20 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1362.3964 F50 X21.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1362.4464 F50 X21.5 Y54.5 Z0.5 ; extrude filament
G1 E1362.5828 F50 X22.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1362.7191 F50 X21.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1362.7941 F50 X20 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1362.8441 F50 X20 Y56 Z0.5 ; extrude filament
G1 E1362.9805 F50 X19 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1363.1168 F50 X20 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1363.1918 F50 X21.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1363.2418 F50 X21.5 Y57.5 Z0.5 ; extrude filament
G1 E1363.3782 F50 X22.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1363.5145 F50 X21.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1363.5895 F50 X20 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1363.6395 F50 X20 Y59 Z0.5 ; extrude filament
G1 E1363.7759 F50 X19 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1363.9122 F50 X20 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1363.9872 F50 X21.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1364.0372 F50 X21.5 Y60.5 Z0.5 ; extrude filament
G1 E1364.1736 F50 X22.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1364.3099 F50 X21.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1364.3849 F50 X20 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1364.4349 F50 X20 Y62 Z0.5 ; extrude filament
G1 E1364.5713 F50 X19 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1364.7076 F50 X20 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1364.7826 F50 X21.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1364.8326 F50 X21.5 Y63.5 Z0.5 ; extrude filament
G1 E1364.969 F50 X22.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1365.1053 F50 X21.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1365.1803 F50 X20 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1365.2303 F50 X20 Y65 Z0.5 ; extrude filament
G1 E1365.3667 F50 X19 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1365.503 F50 X20 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1365.578 F50 X21.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1365.628 F50 X21.5 Y66.5 Z0.5 ; extrude filament
G1 E1365.7644 F50 X22.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1365.9008 F50 X21.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1365.9757 F50 X20 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1366.0258 F50 X20 Y68 Z0.5 ; extrude filament
G1 E1366.1621 F50 X19 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1366.2985 F50 X20 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1366.3734 F50 X21.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1366.4235 F50 X21.5 Y69.5 Z0.5 ; extrude filament
G1 E1366.5598 F50 X22.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1366.6962 F50 X21.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1366.7711 F50 X20 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1366.8212 F50 X20 Y71 Z0.5 ; extrude filament
G1 E1366.9575 F50 X19 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1367.0939 F50 X20 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1367.1688 F50 X21.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1367.2189 F50 X21.5 Y72.5 Z0.5 ; extrude filament
G1 E1367.3552 F50 X22.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1367.4916 F50 X21.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1367.5665 F50 X20 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1367.6166 F50 X20 Y74 Z0.5 ; extrude filament
G1 E1367.7529 F50 X19 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1367.8893 F50 X20 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1367.9642 F50 X21.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1368.0143 F50 X21.5 Y75.5 Z0.5 ; extrude filament
G1 E1368.1506 F50 X22.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1368.287 F50 X21.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1368.3619 F50 X20 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1368.412 F50 X20 Y77 Z0.5 ; extrude filament
G1 E1368.5483 F50 X19 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1368.6847 F50 X20 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1368.7596 F50 X21.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1368.8097 F50 X21.5 Y78.5 Z0.5 ; extrude filament
G1 E1368.946 F50 X22.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1369.0824 F50 X21.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1369.1573 F50 X20 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1369.2074 F50 X20 Y80 Z0.5 ; extrude filament
G1 E1369.3438 F50 X19 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1369.4801 F50 X20 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1369.555 F50 X21.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1369.6051 F50 X21.5 Y81.5 Z0.5 ; extrude filament
G1 E1369.7415 F50 X22.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1369.8778 F50 X21.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1369.9528 F50 X20 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1370.0028 F50 X20 Y83 Z0.5 ; extrude filament
G1 E1370.1392 F50 X19 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1370.2755 F50 X20 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1370.3505 F50 X21.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1370.4005 F50 X21.5 Y84.5 Z0.5 ; extrude filament
G1 E1370.5369 F50 X22.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1370.6732 F50 X21.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1370.7482 F50 X20 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1370.7982 F50 X20 Y86 Z0.5 ; extrude filament
G1 E1370.9346 F50 X19 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1371.0709 F50 X20 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1371.1459 F50 X21.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1371.1959 F50 X21.5 Y87.5 Z0.5 ; extrude filament
G1 E1371.3323 F50 X22.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1371.4686 F50 X21.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1371.5436 F50 X20 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1371.5936 F50 X20 Y89 Z0.5 ; extrude filament
G1 E1371.73 F50 X19 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1371.8663 F50 X20 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1371.9413 F50 X21.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1371.9913 F50 X21.5 Y90.5 Z0.5 ; extrude filament
G1 E1372.1277 F50 X22.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1372.264 F50 X21.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1372.339 F50 X20 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1372.389 F50 X20 Y92 Z0.5 ; extrude filament
G1 E1372.5254 F50 X19 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1372.6617 F50 X20 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1372.7367 F50 X21.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1372.7867 F50 X21.5 Y93.5 Z0.5 ; extrude filament
G1 E1372.9231 F50 X22.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1373.0594 F50 X21.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1373.1344 F50 X20 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1373.1844 F50 X20 Y95 Z0.5 ; extrude filament
G1 E1373.3208 F50 X19 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1373.4572 F50 X20 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1373.5321 F50 X21.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1373.5822 F50 X21.5 Y96.5 Z0.5 ; extrude filament
G1 E1373.7185 F50 X22.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1373.8549 F50 X21.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1373.9298 F50 X20 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1373.9799 F50 X20 Y98 Z0.5 ; extrude filament
G1 E1374.1162 F50 X19 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1374.2526 F50 X20 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1374.3275 F50 X21.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1374.3776 F50 X21.5 Y99.5 Z0.5 ; extrude filament
G1 E1374.5139 F50 X22.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1374.6503 F50 X21.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1374.7252 F50 X20 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1374.7753 F50 X20 Y101 Z0.5 ; extrude filament
G1 E1374.9116 F50 X19 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1375.048 F50 X20 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1375.1229 F50 X21.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1375.173 F50 X21.5 Y102.5 Z0.5 ; extrude filament
G1 E1375.3093 F50 X22.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1375.4457 F50 X21.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1375.5206 F50 X20 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1375.5707 F50 X20 Y104 Z0.5 ; extrude filament
G1 E1375.707 F50 X19 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1375.8434 F50 X20 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1375.9183 F50 X21.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1375.9684 F50 X21.5 Y105.5 Z0.5 ; extrude filament
G1 E1376.1047 F50 X22.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1376.2411 F50 X21.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1376.316 F50 X20 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1376.3661 F50 X20 Y107 Z0.5 ; extrude filament
G1 E1376.5024 F50 X19 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1376.6388 F50 X20 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1376.7137 F50 X21.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1376.7638 F50 X21.5 Y108.5 Z0.5 ; extrude filament
G1 E1376.9001 F50 X22.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1377.0365 F50 X21.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1377.1115 F50 X20 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1377.1615 F50 X20 Y110 Z0.5 ; extrude filament
G1 E1377.2979 F50 X19 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1377.4342 F50 X20 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1377.5092 F50 X21.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1377.5592 F50 X21.5 Y111.5 Z0.5 ; extrude filament
G1 E1377.6956 F50 X22.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1377.8319 F50 X21.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1377.9069 F50 X20 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1377.9569 F50 X20 Y113 Z0.5 ; extrude filament
G1 E1378.0933 F50 X19 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1378.2296 F50 X20 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1378.3046 F50 X21.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1378.3546 F50 X21.5 Y114.5 Z0.5 ; extrude filament
G1 E1378.491 F50 X22.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1378.6273 F50 X21.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1378.7023 F50 X20 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1378.7523 F50 X20 Y116 Z0.5 ; extrude filament
G1 E1378.8887 F50 X19 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1379.025 F50 X20 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1379.1 F50 X21.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1379.15 F50 X21.5 Y117.5 Z0.5 ; extrude filament
G1 E1379.2864 F50 X22.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1379.4227 F50 X21.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1379.4977 F50 X20 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1379.5477 F50 X20 Y119 Z0.5 ; extrude filament
G1 E1376.5477 F5000 X20 Y119 Z0.5 ; retract filament
G0 E1376.5477 F5000 X20 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1376.5477 F5000 X26 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1376.5477 F5000 X26 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1379.5477 F50 X26 Y20 Z0.5 ; extrude filament
G1 E1379.6841 F50 X25 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1379.8204 F50 X26 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1379.8954 F50 X27.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1379.9454 F50 X27.5 Y21.5 Z0.5 ; extrude filament
G1 E1380.0818 F50 X28.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1380.2181 F50 X27.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1380.2931 F50 X26 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1380.3431 F50 X26 Y23 Z0.5 ; extrude filament
G1 E1380.4795 F50 X25 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1380.6158 F50 X26 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1380.6908 F50 X27.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1380.7408 F50 X27.5 Y24.5 Z0.5 ; extrude filament
G1 E1380.8772 F50 X28.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1381.0135 F50 X27.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1381.0885 F50 X26 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1381.1385 F50 X26 Y26 Z0.5 ; extrude filament
G1 E1381.2749 F50 X25 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1381.4113 F50 X26 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1381.4862 F50 X27.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1381.5363 F50 X27.5 Y27.5 Z0.5 ; extrude filament
G1 E1381.6726 F50 X28.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1381.809 F50 X27.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1381.8839 F50 X26 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1381.934 F50 X26 Y29 Z0.5 ; extrude filament
G1 E1382.0703 F50 X25 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1382.2067 F50 X26 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1382.2816 F50 X27.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1382.3317 F50 X27.5 Y30.5 Z0.5 ; extrude filament
G1 E1382.468 F50 X28.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1382.6044 F50 X27.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1382.6793 F50 X26 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1382.7294 F50 X26 Y32 Z0.5 ; extrude filament
G1 E1382.8657 F50 X25 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1383.0021 F50 X26 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1383.077 F50 X27.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1383.1271 F50 X27.5 Y33.5 Z0.5 ; extrude filament
G1 E1383.2634 F50 X28.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1383.3998 F50 X27.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1383.4747 F50 X26 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1383.5248 F50 X26 Y35 Z0.5 ; extrude filament
G1 E1383.6611 F50 X25 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1383.7975 F50 X26 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1383.8724 F50 X27.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1383.9225 F50 X27.5 Y36.5 Z0.5 ; extrude filament
G1 E1384.0588 F50 X28.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1384.1952 F50 X27.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1384.2701 F50 X26 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1384.3202 F50 X26 Y38 Z0.5 ; extrude filament
G1 E1384.4565 F50 X25 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1384.5929 F50 X26 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1384.6678 F50 X27.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1384.7179 F50 X27.5 Y39.5 Z0.5 ; extrude filament
G1 E1384.8542 F50 X28.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1384.9906 F50 X27.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1385.0656 F50 X26 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1385.1156 F50 X26 Y41 Z0.5 ; extrude filament
G1 E1385.252 F50 X25 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1385.3883 F50 X26 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1385.4633 F50 X27.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1385.5133 F50 X27.5 Y42.5 Z0.5 ; extrude filament
G1 E1385.6497 F50 X28.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1385.786 F50 X27.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1385.861 F50 X26 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1385.911 F50 X26 Y44 Z0.5 ; extrude filament
G1 E1386.0474 F50 X25 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1386.1837 F50 X26 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1386.2587 F50 X27.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1386.3087 F50 X27.5 Y45.5 Z0.5 ; extrude filament
G1 E1386.4451 F50 X28.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1386.5814 F50 X27.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1386.6564 F50 X26 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1386.7064 F50 X26 Y47 Z0.5 ; extrude filament
G1 E1386.8428 F50 X25 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1386.9791 F50 X26 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1387.0541 F50 X27.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1387.1041 F50 X27.5 Y48.5 Z0.5 ; extrude filament
G1 E1387.2405 F50 X28.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1387.3768 F50 X27.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1387.4518 F50 X26 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1387.5018 F50 X26 Y50 Z0.5 ; extrude filament
G1 E1387.6382 F50 X25 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1387.7745 F50 X26 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1387.8495 F50 X27.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1387.8995 F50 X27.5 Y51.5 Z0.5 ; extrude filament
G1 E1388.0359 F50 X28.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1388.1722 F50 X27.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1388.2472 F50 X26 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1388.2972 F50 X26 Y53 Z0.5 ; extrude filament
G1 E1388.4336 F50 X25 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1388.57 F50 X26 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1388.6449 F50 X27.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1388.695 F50 X27.5 Y54.5 Z0.5 ; extrude filament
G1 E1388.8313 F50 X28.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1388.9677 F50 X27.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1389.0426 F50 X26 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1389.0927 F50 X26 Y56 Z0.5 ; extrude filament
G1 E1389.229 F50 X25 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1389.3654 F50 X26 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1389.4403 F50 X27.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1389.4904 F50 X27.5 Y57.5 Z0.5 ; extrude filament
G1 E1389.6267 F50 X28.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1389.7631 F50 X27.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1389.838 F50 X26 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1389.8881 F50 X26 Y59 Z0.5 ; extrude filament
G1 E1390.0244 F50 X25 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1390.1608 F50 X26 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1390.2357 F50 X27.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1390.2858 F50 X27.5 Y60.5 Z0.5 ; extrude filament
G1 E1390.4221 F50 X28.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1390.5585 F50 X27.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1390.6334 F50 X26 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1390.6835 F50 X26 Y62 Z0.5 ; extrude filament
G1 E1390.8198 F50 X25 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1390.9562 F50 X26 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1391.0311 F50 X27.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1391.0812 F50 X27.5 Y63.5 Z0.5 ; extrude filament
G1 E1391.2175 F50 X28.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1391.3539 F50 X27.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1391.4288 F50 X26 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1391.4789 F50 X26 Y65 Z0.5 ; extrude filament
G1 E1391.6152 F50 X25 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1391.7516 F50 X26 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1391.8265 F50 X27.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1391.8766 F50 X27.5 Y66.5 Z0.5 ; extrude filament
G1 E1392.013 F50 X28.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1392.1493 F50 X27.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1392.2242 F50 X26 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1392.2743 F50 X26 Y68 Z0.5 ; extrude filament
G1 E1392.4106 F50 X25 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1392.547 F50 X26 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1392.622 F50 X27.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1392.672 F50 X27.5 Y69.5 Z0.5 ; extrude filament
G1 E1392.8083 F50 X28.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1392.9447 F50 X27.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1393.0197 F50 X26 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1393.0697 F50 X26 Y71 Z0.5 ; extrude filament
G1 E1393.206 F50 X25 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1393.3424 F50 X26 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1393.4174 F50 X27.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1393.4674 F50 X27.5 Y72.5 Z0.5 ; extrude filament
G1 E1393.6038 F50 X28.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1393.7401 F50 X27.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1393.8151 F50 X26 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1393.8651 F50 X26 Y74 Z0.5 ; extrude filament
G1 E1394.0015 F50 X25 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1394.1378 F50 X26 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1394.2128 F50 X27.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1394.2628 F50 X27.5 Y75.5 Z0.5 ; extrude filament
G1 E1394.3992 F50 X28.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1394.5355 F50 X27.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1394.6105 F50 X26 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1394.6605 F50 X26 Y77 Z0.5 ; extrude filament
G1 E1394.7969 F50 X25 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1394.9332 F50 X26 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1395.0082 F50 X27.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1395.0582 F50 X27.5 Y78.5 Z0.5 ; extrude filament
G1 E1395.1946 F50 X28.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1395.3309 F50 X27.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1395.4059 F50 X26 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1395.4559 F50 X26 Y80 Z0.5 ; extrude filament
G1 E1395.5923 F50 X25 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1395.7286 F50 X26 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1395.8036 F50 X27.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1395.8536 F50 X27.5 Y81.5 Z0.5 ; extrude filament
G1 E1395.99 F50 X28.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1396.1263 F50 X27.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1396.2013 F50 X26 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1396.2513 F50 X26 Y83 Z0.5 ; extrude filament
G1 E1396.3877 F50 X25 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1396.524 F50 X26 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1396.599 F50 X27.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1396.649 F50 X27.5 Y84.5 Z0.5 ; extrude filament
G1 E1396.7854 F50 X28.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1396.9218 F50 X27.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1396.9967 F50 X26 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1397.0468 F50 X26 Y86 Z0.5 ; extrude filament
G1 E1397.1831 F50 X25 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1397.3195 F50 X26 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1397.3944 F50 X27.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1397.4445 F50 X27.5 Y87.5 Z0.5 ; extrude filament
G1 E1397.5808 F50 X28.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1397.7172 F50 X27.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1397.7921 F50 X26 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1397.8422 F50 X26 Y89 Z0.5 ; extrude filament
G1 E1397.9785 F50 X25 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1398.1149 F50 X26 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1398.1898 F50 X27.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1398.2399 F50 X27.5 Y90.5 Z0.5 ; extrude filament
G1 E1398.3762 F50 X28.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1398.5126 F50 X27.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1398.5875 F50 X26 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1398.6376 F50 X26 Y92 Z0.5 ; extrude filament
G1 E1398.7739 F50 X25 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1398.9103 F50 X26 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1398.9852 F50 X27.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1399.0353 F50 X27.5 Y93.5 Z0.5 ; extrude filament
G1 E1399.1716 F50 X28.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1399.308 F50 X27.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1399.3829 F50 X26 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1399.433 F50 X26 Y95 Z0.5 ; extrude filament
G1 E1399.5693 F50 X25 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1399.7057 F50 X26 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1399.7806 F50 X27.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1399.8307 F50 X27.5 Y96.5 Z0.5 ; extrude filament
G1 E1399.967 F50 X28.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1400.1034 F50 X27.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1400.1783 F50 X26 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1400.2284 F50 X26 Y98 Z0.5 ; extrude filament
G1 E1400.3647 F50 X25 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1400.5011 F50 X26 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1400.576 F50 X27.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1400.6261 F50 X27.5 Y99.5 Z0.5 ; extrude filament
G1 E1400.7625 F50 X28.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1400.8988 F50 X27.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1400.9738 F50 X26 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1401.0238 F50 X26 Y101 Z0.5 ; extrude filament
G1 E1401.1602 F50 X25 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1401.2965 F50 X26 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1401.3715 F50 X27.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1401.4215 F50 X27.5 Y102.5 Z0.5 ; extrude filament
G1 E1401.5579 F50 X28.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1401.6942 F50 X27.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1401.7692 F50 X26 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1401.8192 F50 X26 Y104 Z0.5 ; extrude filament
G1 E1401.9556 F50 X25 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1402.0919 F50 X26 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1402.1669 F50 X27.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1402.2169 F50 X27.5 Y105.5 Z0.5 ; extrude filament
G1 E1402.3533 F50 X28.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1402.4896 F50 X27.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1402.5646 F50 X26 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1402.6146 F50 X26 Y107 Z0.5 ; extrude filament
G1 E1402.751 F50 X25 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1402.8873 F50 X26 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1402.9623 F50 X27.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1403.0123 F50 X27.5 Y108.5 Z0.5 ; extrude filament
G1 E1403.1487 F50 X28.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1403.285 F50 X27.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1403.36 F50 X26 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1403.41 F50 X26 Y110 Z0.5 ; extrude filament
G1 E1403.5464 F50 X25 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1403.6827 F50 X26 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1403.7577 F50 X27.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1403.8077 F50 X27.5 Y111.5 Z0.5 ; extrude filament
G1 E1403.9441 F50 X28.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1404.0804 F50 X27.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1404.1554 F50 X26 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1404.2054 F50 X26 Y113 Z0.5 ; extrude filament
G1 E1404.3418 F50 X25 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1404.4781 F50 X26 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1404.5531 F50 X27.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1404.6031 F50 X27.5 Y114.5 Z0.5 ; extrude filament
G1 E1404.7395 F50 X28.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1404.8759 F50 X27.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1404.9508 F50 X26 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1405.0009 F50 X26 Y116 Z0.5 ; extrude filament
G1 E1405.1372 F50 X25 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1405.2736 F50 X26 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1405.3485 F50 X27.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1405.3986 F50 X27.5 Y117.5 Z0.5 ; extrude filament
G1 E1405.5349 F50 X28.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1405.6713 F50 X27.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1405.7462 F50 X26 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1405.7963 F50 X26 Y119 Z0.5 ; extrude filament
G1 E1402.7963 F5000 X26 Y119 Z0.5 ; retract filament
G0 E1402.7963 F5000 X26 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1402.7963 F5000 X32 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1402.7963 F5000 X32 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1405.7963 F50 X32 Y20 Z0.5 ; extrude filament
G1 E1405.9326 F50 X31 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1406.069 F50 X32 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1406.1439 F50 X33.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1406.194 F50 X33.5 Y21.5 Z0.5 ; extrude filament
G1 E1406.3303 F50 X34.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1406.4667 F50 X33.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1406.5416 F50 X32 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1406.5917 F50 X32 Y23 Z0.5 ; extrude filament
G1 E1406.728 F50 X31 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1406.8644 F50 X32 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1406.9393 F50 X33.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1406.9894 F50 X33.5 Y24.5 Z0.5 ; extrude filament
G1 E1407.1257 F50 X34.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1407.2621 F50 X33.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1407.337 F50 X32 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1407.3871 F50 X32 Y26 Z0.5 ; extrude filament
G1 E1407.5234 F50 X31 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1407.6598 F50 X32 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1407.7347 F50 X33.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1407.7848 F50 X33.5 Y27.5 Z0.5 ; extrude filament
G1 E1407.9211 F50 X34.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1408.0575 F50 X33.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1408.1324 F50 X32 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1408.1825 F50 X32 Y29 Z0.5 ; extrude filament
G1 E1408.3188 F50 X31 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1408.4552 F50 X32 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1408.5302 F50 X33.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1408.5802 F50 X33.5 Y30.5 Z0.5 ; extrude filament
G1 E1408.7166 F50 X34.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1408.8529 F50 X33.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1408.9279 F50 X32 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1408.9779 F50 X32 Y32 Z0.5 ; extrude filament
G1 E1409.1143 F50 X31 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1409.2506 F50 X32 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1409.3256 F50 X33.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1409.3756 F50 X33.5 Y33.5 Z0.5 ; extrude filament
G1 E1409.512 F50 X34.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1409.6483 F50 X33.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1409.7233 F50 X32 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1409.7733 F50 X32 Y35 Z0.5 ; extrude filament
G1 E1409.9097 F50 X31 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1410.046 F50 X32 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1410.121 F50 X33.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1410.171 F50 X33.5 Y36.5 Z0.5 ; extrude filament
G1 E1410.3074 F50 X34.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1410.4437 F50 X33.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1410.5187 F50 X32 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1410.5687 F50 X32 Y38 Z0.5 ; extrude filament
G1 E1410.7051 F50 X31 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1410.8414 F50 X32 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1410.9164 F50 X33.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1410.9664 F50 X33.5 Y39.5 Z0.5 ; extrude filament
G1 E1411.1028 F50 X34.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1411.2391 F50 X33.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1411.3141 F50 X32 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1411.3641 F50 X32 Y41 Z0.5 ; extrude filament
G1 E1411.5005 F50 X31 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1411.6368 F50 X32 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1411.7118 F50 X33.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1411.7618 F50 X33.5 Y42.5 Z0.5 ; extrude filament
G1 E1411.8982 F50 X34.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1412.0345 F50 X33.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1412.1095 F50 X32 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1412.1595 F50 X32 Y44 Z0.5 ; extrude filament
G1 E1412.2959 F50 X31 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1412.4323 F50 X32 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1412.5072 F50 X33.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1412.5573 F50 X33.5 Y45.5 Z0.5 ; extrude filament
G1 E1412.6936 F50 X34.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1412.83 F50 X33.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1412.9049 F50 X32 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1412.955 F50 X32 Y47 Z0.5 ; extrude filament
G1 E1413.0913 F50 X31 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1413.2277 F50 X32 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1413.3026 F50 X33.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1413.3527 F50 X33.5 Y48.5 Z0.5 ; extrude filament
G1 E1413.489 F50 X34.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1413.6254 F50 X33.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1413.7003 F50 X32 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1413.7504 F50 X32 Y50 Z0.5 ; extrude filament
G1 E1413.8867 F50 X31 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1414.0231 F50 X32 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1414.098 F50 X33.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1414.1481 F50 X33.5 Y51.5 Z0.5 ; extrude filament
G1 E1414.2844 F50 X34.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1414.4208 F50 X33.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1414.4957 F50 X32 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1414.5458 F50 X32 Y53 Z0.5 ; extrude filament
G1 E1414.6821 F50 X31 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1414.8185 F50 X32 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1414.8934 F50 X33.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1414.9435 F50 X33.5 Y54.5 Z0.5 ; extrude filament
G1 E1415.0798 F50 X34.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1415.2162 F50 X33.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1415.2911 F50 X32 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1415.3412 F50 X32 Y56 Z0.5 ; extrude filament
G1 E1415.4775 F50 X31 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1415.6139 F50 X32 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1415.6888 F50 X33.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1415.7389 F50 X33.5 Y57.5 Z0.5 ; extrude filament
G1 E1415.8752 F50 X34.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1416.0116 F50 X33.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1416.0865 F50 X32 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1416.1366 F50 X32 Y59 Z0.5 ; extrude filament
G1 E1416.273 F50 X31 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1416.4093 F50 X32 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1416.4843 F50 X33.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1416.5343 F50 X33.5 Y60.5 Z0.5 ; extrude filament
G1 E1416.6707 F50 X34.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1416.807 F50 X33.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1416.882 F50 X32 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1416.932 F50 X32 Y62 Z0.5 ; extrude filament
G1 E1417.0684 F50 X31 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1417.2047 F50 X32 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1417.2797 F50 X33.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1417.3297 F50 X33.5 Y63.5 Z0.5 ; extrude filament
G1 E1417.4661 F50 X34.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1417.6024 F50 X33.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1417.6774 F50 X32 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1417.7274 F50 X32 Y65 Z0.5 ; extrude filament
G1 E1417.8638 F50 X31 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1418.0001 F50 X32 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1418.0751 F50 X33.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1418.1251 F50 X33.5 Y66.5 Z0.5 ; extrude filament
G1 E1418.2615 F50 X34.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1418.3978 F50 X33.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1418.4728 F50 X32 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1418.5228 F50 X32 Y68 Z0.5 ; extrude filament
G1 E1418.6592 F50 X31 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1418.7955 F50 X32 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1418.8705 F50 X33.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1418.9205 F50 X33.5 Y69.5 Z0.5 ; extrude filament
G1 E1419.0569 F50 X34.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1419.1932 F50 X33.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1419.2682 F50 X32 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1419.3182 F50 X32 Y71 Z0.5 ; extrude filament
G1 E1419.4546 F50 X31 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1419.591 F50 X32 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1419.6659 F50 X33.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1419.716 F50 X33.5 Y72.5 Z0.5 ; extrude filament
G1 E1419.8523 F50 X34.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1419.9886 F50 X33.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1420.0636 F50 X32 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1420.1136 F50 X32 Y74 Z0.5 ; extrude filament
G1 E1420.25 F50 X31 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1420.3864 F50 X32 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1420.4613 F50 X33.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1420.5114 F50 X33.5 Y75.5 Z0.5 ; extrude filament
G1 E1420.6477 F50 X34.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1420.784 F50 X33.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1420.859 F50 X32 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1420.909 F50 X32 Y77 Z0.5 ; extrude filament
G1 E1421.0454 F50 X31 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1421.1818 F50 X32 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1421.2567 F50 X33.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1421.3068 F50 X33.5 Y78.5 Z0.5 ; extrude filament
G1 E1421.4431 F50 X34.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1421.5795 F50 X33.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1421.6544 F50 X32 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1421.7045 F50 X32 Y80 Z0.5 ; extrude filament
G1 E1421.8408 F50 X31 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1421.9772 F50 X32 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1422.0521 F50 X33.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1422.1022 F50 X33.5 Y81.5 Z0.5 ; extrude filament
G1 E1422.2385 F50 X34.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1422.3749 F50 X33.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1422.4498 F50 X32 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1422.4999 F50 X32 Y83 Z0.5 ; extrude filament
G1 E1422.6362 F50 X31 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1422.7726 F50 X32 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1422.8475 F50 X33.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1422.8976 F50 X33.5 Y84.5 Z0.5 ; extrude filament
G1 E1423.0339 F50 X34.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1423.1703 F50 X33.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1423.2452 F50 X32 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1423.2953 F50 X32 Y86 Z0.5 ; extrude filament
G1 E1423.4316 F50 X31 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1423.568 F50 X32 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1423.643 F50 X33.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1423.693 F50 X33.5 Y87.5 Z0.5 ; extrude filament
G1 E1423.8293 F50 X34.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1423.9657 F50 X33.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1424.0406 F50 X32 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1424.0907 F50 X32 Y89 Z0.5 ; extrude filament
G1 E1424.227 F50 X31 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1424.3634 F50 X32 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1424.4384 F50 X33.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1424.4884 F50 X33.5 Y90.5 Z0.5 ; extrude filament
G1 E1424.6248 F50 X34.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1424.7611 F50 X33.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1424.836 F50 X32 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1424.8861 F50 X32 Y92 Z0.5 ; extrude filament
G1 E1425.0225 F50 X31 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1425.1588 F50 X32 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1425.2338 F50 X33.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1425.2838 F50 X33.5 Y93.5 Z0.5 ; extrude filament
G1 E1425.4202 F50 X34.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1425.5565 F50 X33.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1425.6315 F50 X32 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1425.6815 F50 X32 Y95 Z0.5 ; extrude filament
G1 E1425.8179 F50 X31 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1425.9542 F50 X32 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1426.0292 F50 X33.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1426.0792 F50 X33.5 Y96.5 Z0.5 ; extrude filament
G1 E1426.2156 F50 X34.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1426.3519 F50 X33.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1426.4269 F50 X32 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1426.4769 F50 X32 Y98 Z0.5 ; extrude filament
G1 E1426.6133 F50 X31 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1426.7496 F50 X32 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1426.8246 F50 X33.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1426.8746 F50 X33.5 Y99.5 Z0.5 ; extrude filament
G1 E1427.011 F50 X34.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1427.1473 F50 X33.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1427.2223 F50 X32 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1427.2723 F50 X32 Y101 Z0.5 ; extrude filament
G1 E1427.4087 F50 X31 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1427.545 F50 X32 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1427.62 F50 X33.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1427.67 F50 X33.5 Y102.5 Z0.5 ; extrude filament
G1 E1427.8064 F50 X34.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1427.9427 F50 X33.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1428.0177 F50 X32 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1428.0677 F50 X32 Y104 Z0.5 ; extrude filament
G1 E1428.2041 F50 X31 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1428.3405 F50 X32 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1428.4154 F50 X33.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1428.4655 F50 X33.5 Y105.5 Z0.5 ; extrude filament
G1 E1428.6018 F50 X34.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1428.7382 F50 X33.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1428.8131 F50 X32 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1428.8632 F50 X32 Y107 Z0.5 ; extrude filament
G1 E1428.9995 F50 X31 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1429.1359 F50 X32 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1429.2108 F50 X33.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1429.2609 F50 X33.5 Y108.5 Z0.5 ; extrude filament
G1 E1429.3972 F50 X34.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1429.5336 F50 X33.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1429.6085 F50 X32 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1429.6586 F50 X32 Y110 Z0.5 ; extrude filament
G1 E1429.7949 F50 X31 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1429.9313 F50 X32 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1430.0062 F50 X33.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1430.0563 F50 X33.5 Y111.5 Z0.5 ; extrude filament
G1 E1430.1926 F50 X34.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1430.329 F50 X33.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1430.4039 F50 X32 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1430.454 F50 X32 Y113 Z0.5 ; extrude filament
G1 E1430.5903 F50 X31 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1430.7267 F50 X32 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1430.8016 F50 X33.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1430.8517 F50 X33.5 Y114.5 Z0.5 ; extrude filament
G1 E1430.988 F50 X34.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1431.1244 F50 X33.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1431.1993 F50 X32 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1431.2494 F50 X32 Y116 Z0.5 ; extrude filament
G1 E1431.3857 F50 X31 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1431.5221 F50 X32 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1431.597 F50 X33.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1431.6471 F50 X33.5 Y117.5 Z0.5 ; extrude filament
G1 E1431.7834 F50 X34.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1431.9198 F50 X33.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1431.9948 F50 X32 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1432.0448 F50 X32 Y119 Z0.5 ; extrude filament
G1 E1429.0448 F5000 X32 Y119 Z0.5 ; retract filament
G0 E1429.0448 F5000 X32 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1429.0448 F5000 X38 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1429.0448 F5000 X38 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1432.0448 F50 X38 Y20 Z0.5 ; extrude filament
G1 E1432.1812 F50 X37 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1432.3175 F50 X38 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1432.3925 F50 X39.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1432.4425 F50 X39.5 Y21.5 Z0.5 ; extrude filament
G1 E1432.5789 F50 X40.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1432.7152 F50 X39.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1432.7902 F50 X38 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1432.8402 F50 X38 Y23 Z0.5 ; extrude filament
G1 E1432.9766 F50 X37 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1433.1129 F50 X38 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1433.1879 F50 X39.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1433.2379 F50 X39.5 Y24.5 Z0.5 ; extrude filament
G1 E1433.3743 F50 X40.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1433.5106 F50 X39.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1433.5856 F50 X38 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1433.6356 F50 X38 Y26 Z0.5 ; extrude filament
G1 E1433.772 F50 X37 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1433.9083 F50 X38 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1433.9833 F50 X39.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1434.0333 F50 X39.5 Y27.5 Z0.5 ; extrude filament
G1 E1434.1697 F50 X40.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1434.306 F50 X39.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1434.381 F50 X38 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1434.431 F50 X38 Y29 Z0.5 ; extrude filament
G1 E1434.5674 F50 X37 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1434.7037 F50 X38 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1434.7787 F50 X39.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1434.8287 F50 X39.5 Y30.5 Z0.5 ; extrude filament
G1 E1434.9651 F50 X40.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1435.1014 F50 X39.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1435.1764 F50 X38 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1435.2264 F50 X38 Y32 Z0.5 ; extrude filament
G1 E1435.3628 F50 X37 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1435.4991 F50 X38 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1435.5741 F50 X39.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1435.6241 F50 X39.5 Y33.5 Z0.5 ; extrude filament
G1 E1435.7605 F50 X40.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1435.8969 F50 X39.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1435.9718 F50 X38 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1436.0219 F50 X38 Y35 Z0.5 ; extrude filament
G1 E1436.1582 F50 X37 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1436.2946 F50 X38 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1436.3695 F50 X39.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1436.4196 F50 X39.5 Y36.5 Z0.5 ; extrude filament
G1 E1436.5559 F50 X40.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1436.6923 F50 X39.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1436.7672 F50 X38 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1436.8173 F50 X38 Y38 Z0.5 ; extrude filament
G1 E1436.9536 F50 X37 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1437.09 F50 X38 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1437.1649 F50 X39.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1437.215 F50 X39.5 Y39.5 Z0.5 ; extrude filament
G1 E1437.3513 F50 X40.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1437.4877 F50 X39.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1437.5626 F50 X38 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1437.6127 F50 X38 Y41 Z0.5 ; extrude filament
G1 E1437.749 F50 X37 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1437.8854 F50 X38 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1437.9603 F50 X39.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1438.0104 F50 X39.5 Y42.5 Z0.5 ; extrude filament
G1 E1438.1467 F50 X40.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1438.2831 F50 X39.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1438.358 F50 X38 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1438.4081 F50 X38 Y44 Z0.5 ; extrude filament
G1 E1438.5444 F50 X37 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1438.6808 F50 X38 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1438.7557 F50 X39.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1438.8058 F50 X39.5 Y45.5 Z0.5 ; extrude filament
G1 E1438.9421 F50 X40.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1439.0785 F50 X39.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1439.1534 F50 X38 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1439.2035 F50 X38 Y47 Z0.5 ; extrude filament
G1 E1439.3398 F50 X37 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1439.4762 F50 X38 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1439.5511 F50 X39.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1439.6012 F50 X39.5 Y48.5 Z0.5 ; extrude filament
G1 E1439.7375 F50 X40.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1439.8739 F50 X39.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1439.9489 F50 X38 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1439.9989 F50 X38 Y50 Z0.5 ; extrude filament
G1 E1440.1353 F50 X37 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1440.2716 F50 X38 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1440.3466 F50 X39.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1440.3966 F50 X39.5 Y51.5 Z0.5 ; extrude filament
G1 E1440.533 F50 X40.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1440.6693 F50 X39.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1440.7443 F50 X38 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1440.7943 F50 X38 Y53 Z0.5 ; extrude filament
G1 E1440.9307 F50 X37 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1441.067 F50 X38 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1441.142 F50 X39.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1441.192 F50 X39.5 Y54.5 Z0.5 ; extrude filament
G1 E1441.3284 F50 X40.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1441.4647 F50 X39.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1441.5397 F50 X38 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1441.5897 F50 X38 Y56 Z0.5 ; extrude filament
G1 E1441.7261 F50 X37 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1441.8624 F50 X38 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1441.9374 F50 X39.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1441.9874 F50 X39.5 Y57.5 Z0.5 ; extrude filament
G1 E1442.1238 F50 X40.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1442.2601 F50 X39.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1442.3351 F50 X38 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1442.3851 F50 X38 Y59 Z0.5 ; extrude filament
G1 E1442.5215 F50 X37 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1442.6578 F50 X38 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1442.7328 F50 X39.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1442.7828 F50 X39.5 Y60.5 Z0.5 ; extrude filament
G1 E1442.9192 F50 X40.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1443.0555 F50 X39.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1443.1305 F50 X38 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1443.1805 F50 X38 Y62 Z0.5 ; extrude filament
G1 E1443.3169 F50 X37 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1443.4532 F50 X38 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1443.5282 F50 X39.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1443.5782 F50 X39.5 Y63.5 Z0.5 ; extrude filament
G1 E1443.7146 F50 X40.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1443.851 F50 X39.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1443.9259 F50 X38 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1443.976 F50 X38 Y65 Z0.5 ; extrude filament
G1 E1444.1123 F50 X37 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1444.2487 F50 X38 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1444.3236 F50 X39.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1444.3737 F50 X39.5 Y66.5 Z0.5 ; extrude filament
G1 E1444.51 F50 X40.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1444.6464 F50 X39.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1444.7213 F50 X38 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1444.7714 F50 X38 Y68 Z0.5 ; extrude filament
G1 E1444.9077 F50 X37 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1445.0441 F50 X38 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1445.119 F50 X39.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1445.1691 F50 X39.5 Y69.5 Z0.5 ; extrude filament
G1 E1445.3054 F50 X40.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1445.4418 F50 X39.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1445.5167 F50 X38 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1445.5668 F50 X38 Y71 Z0.5 ; extrude filament
G1 E1445.7031 F50 X37 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1445.8395 F50 X38 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1445.9144 F50 X39.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1445.9645 F50 X39.5 Y72.5 Z0.5 ; extrude filament
G1 E1446.1008 F50 X40.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1446.2372 F50 X39.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1446.3121 F50 X38 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1446.3622 F50 X38 Y74 Z0.5 ; extrude filament
G1 E1446.4985 F50 X37 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1446.6349 F50 X38 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1446.7098 F50 X39.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1446.7599 F50 X39.5 Y75.5 Z0.5 ; extrude filament
G1 E1446.8962 F50 X40.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1447.0326 F50 X39.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1447.1075 F50 X38 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1447.1576 F50 X38 Y77 Z0.5 ; extrude filament
G1 E1447.294 F50 X37 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1447.4303 F50 X38 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1447.5052 F50 X39.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1447.5553 F50 X39.5 Y78.5 Z0.5 ; extrude filament
G1 E1447.6917 F50 X40.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1447.828 F50 X39.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1447.903 F50 X38 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1447.953 F50 X38 Y80 Z0.5 ; extrude filament
G1 E1448.0894 F50 X37 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1448.2257 F50 X38 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1448.3007 F50 X39.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1448.3507 F50 X39.5 Y81.5 Z0.5 ; extrude filament
G1 E1448.487 F50 X40.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1448.6234 F50 X39.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1448.6984 F50 X38 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1448.7484 F50 X38 Y83 Z0.5 ; extrude filament
G1 E1448.8848 F50 X37 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1449.0211 F50 X38 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1449.0961 F50 X39.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1449.1461 F50 X39.5 Y84.5 Z0.5 ; extrude filament
G1 E1449.2825 F50 X40.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1449.4188 F50 X39.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1449.4938 F50 X38 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1449.5438 F50 X38 Y86 Z0.5 ; extrude filament
G1 E1449.6802 F50 X37 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1449.8165 F50 X38 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1449.8915 F50 X39.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1449.9415 F50 X39.5 Y87.5 Z0.5 ; extrude filament
G1 E1450.0779 F50 X40.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1450.2142 F50 X39.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1450.2892 F50 X38 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1450.3392 F50 X38 Y89 Z0.5 ; extrude filament
G1 E1450.4756 F50 X37 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1450.6119 F50 X38 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1450.6869 F50 X39.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1450.7369 F50 X39.5 Y90.5 Z0.5 ; extrude filament
G1 E1450.8733 F50 X40.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1451.0096 F50 X39.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1451.0846 F50 X38 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1451.1346 F50 X38 Y92 Z0.5 ; extrude filament
G1 E1451.271 F50 X37 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1451.4073 F50 X38 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1451.4823 F50 X39.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1451.5323 F50 X39.5 Y93.5 Z0.5 ; extrude filament
G1 E1451.6687 F50 X40.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1451.805 F50 X39.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1451.88 F50 X38 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1451.93 F50 X38 Y95 Z0.5 ; extrude filament
G1 E1452.0664 F50 X37 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1452.2028 F50 X38 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1452.2777 F50 X39.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1452.3278 F50 X39.5 Y96.5 Z0.5 ; extrude filament
G1 E1452.4641 F50 X40.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1452.6005 F50 X39.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1452.6754 F50 X38 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1452.7255 F50 X38 Y98 Z0.5 ; extrude filament
G1 E1452.8618 F50 X37 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1452.9982 F50 X38 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1453.0731 F50 X39.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1453.1232 F50 X39.5 Y99.5 Z0.5 ; extrude filament
G1 E1453.2595 F50 X40.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1453.3959 F50 X39.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1453.4708 F50 X38 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1453.5209 F50 X38 Y101 Z0.5 ; extrude filament
G1 E1453.6572 F50 X37 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1453.7936 F50 X38 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1453.8685 F50 X39.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1453.9186 F50 X39.5 Y102.5 Z0.5 ; extrude filament
G1 E1454.0549 F50 X40.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1454.1913 F50 X39.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1454.2662 F50 X38 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1454.3163 F50 X38 Y104 Z0.5 ; extrude filament
G1 E1454.4526 F50 X37 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1454.589 F50 X38 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1454.664 F50 X39.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1454.714 F50 X39.5 Y105.5 Z0.5 ; extrude filament
G1 E1454.8503 F50 X40.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1454.9867 F50 X39.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1455.0616 F50 X38 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1455.1117 F50 X38 Y107 Z0.5 ; extrude filament
G1 E1455.248 F50 X37 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1455.3844 F50 X38 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1455.4594 F50 X39.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1455.5094 F50 X39.5 Y108.5 Z0.5 ; extrude filament
G1 E1455.6458 F50 X40.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1455.7821 F50 X39.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1455.857 F50 X38 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1455.9071 F50 X38 Y110 Z0.5 ; extrude filament
G1 E1456.0435 F50 X37 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1456.1798 F50 X38 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1456.2548 F50 X39.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1456.3048 F50 X39.5 Y111.5 Z0.5 ; extrude filament
G1 E1456.4412 F50 X40.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1456.5775 F50 X39.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1456.6525 F50 X38 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1456.7025 F50 X38 Y113 Z0.5 ; extrude filament
G1 E1456.8389 F50 X37 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1456.9752 F50 X38 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1457.0502 F50 X39.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1457.1002 F50 X39.5 Y114.5 Z0.5 ; extrude filament
G1 E1457.2366 F50 X40.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1457.3729 F50 X39.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1457.4479 F50 X38 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1457.4979 F50 X38 Y116 Z0.5 ; extrude filament
G1 E1457.6343 F50 X37 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1457.7706 F50 X38 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1457.8456 F50 X39.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1457.8956 F50 X39.5 Y117.5 Z0.5 ; extrude filament
G1 E1458.032 F50 X40.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1458.1683 F50 X39.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1458.2433 F50 X38 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1458.2933 F50 X38 Y119 Z0.5 ; extrude filament
G1 E1455.2933 F5000 X38 Y119 Z0.5 ; retract filament
G0 E1455.2933 F5000 X38 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1455.2933 F5000 X44 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1455.2933 F5000 X44 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1458.2933 F50 X44 Y20 Z0.5 ; extrude filament
G1 E1458.4297 F50 X43 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1458.566 F50 X44 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1458.641 F50 X45.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1458.691 F50 X45.5 Y21.5 Z0.5 ; extrude filament
G1 E1458.8274 F50 X46.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1458.9637 F50 X45.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1459.0387 F50 X44 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1459.0887 F50 X44 Y23 Z0.5 ; extrude filament
G1 E1459.2251 F50 X43 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1459.3615 F50 X44 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1459.4364 F50 X45.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1459.4865 F50 X45.5 Y24.5 Z0.5 ; extrude filament
G1 E1459.6228 F50 X46.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1459.7592 F50 X45.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1459.8341 F50 X44 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1459.8842 F50 X44 Y26 Z0.5 ; extrude filament
G1 E1460.0205 F50 X43 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1460.1569 F50 X44 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1460.2318 F50 X45.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1460.2819 F50 X45.5 Y27.5 Z0.5 ; extrude filament
G1 E1460.4182 F50 X46.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1460.5546 F50 X45.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1460.6295 F50 X44 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1460.6796 F50 X44 Y29 Z0.5 ; extrude filament
G1 E1460.8159 F50 X43 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1460.9523 F50 X44 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1461.0272 F50 X45.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1461.0773 F50 X45.5 Y30.5 Z0.5 ; extrude filament
G1 E1461.2136 F50 X46.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1461.35 F50 X45.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1461.4249 F50 X44 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1461.475 F50 X44 Y32 Z0.5 ; extrude filament
G1 E1461.6113 F50 X43 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1461.7477 F50 X44 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1461.8226 F50 X45.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1461.8727 F50 X45.5 Y33.5 Z0.5 ; extrude filament
G1 E1462.009 F50 X46.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1462.1454 F50 X45.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1462.2203 F50 X44 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1462.2704 F50 X44 Y35 Z0.5 ; extrude filament
G1 E1462.4067 F50 X43 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1462.5431 F50 X44 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1462.618 F50 X45.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1462.6681 F50 X45.5 Y36.5 Z0.5 ; extrude filament
G1 E1462.8044 F50 X46.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1462.9408 F50 X45.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1463.0157 F50 X44 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1463.0658 F50 X44 Y38 Z0.5 ; extrude filament
G1 E1463.2021 F50 X43 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1463.3385 F50 X44 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1463.4135 F50 X45.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1463.4635 F50 X45.5 Y39.5 Z0.5 ; extrude filament
G1 E1463.5999 F50 X46.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1463.7362 F50 X45.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1463.8112 F50 X44 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1463.8612 F50 X44 Y41 Z0.5 ; extrude filament
G1 E1463.9976 F50 X43 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1464.1339 F50 X44 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1464.2089 F50 X45.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1464.2589 F50 X45.5 Y42.5 Z0.5 ; extrude filament
G1 E1464.3953 F50 X46.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1464.5316 F50 X45.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1464.6066 F50 X44 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1464.6566 F50 X44 Y44 Z0.5 ; extrude filament
G1 E1464.793 F50 X43 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1464.9293 F50 X44 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1465.0043 F50 X45.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1465.0543 F50 X45.5 Y45.5 Z0.5 ; extrude filament
G1 E1465.1907 F50 X46.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1465.327 F50 X45.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1465.402 F50 X44 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1465.452 F50 X44 Y47 Z0.5 ; extrude filament
G1 E1465.5884 F50 X43 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1465.7247 F50 X44 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1465.7997 F50 X45.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1465.8497 F50 X45.5 Y48.5 Z0.5 ; extrude filament
G1 E1465.9861 F50 X46.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1466.1224 F50 X45.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1466.1974 F50 X44 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1466.2474 F50 X44 Y50 Z0.5 ; extrude filament
G1 E1466.3838 F50 X43 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1466.5201 F50 X44 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1466.5951 F50 X45.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1466.6451 F50 X45.5 Y51.5 Z0.5 ; extrude filament
G1 E1466.7815 F50 X46.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1466.9178 F50 X45.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1466.9928 F50 X44 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1467.0428 F50 X44 Y53 Z0.5 ; extrude filament
G1 E1467.1792 F50 X43 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1467.3156 F50 X44 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1467.3905 F50 X45.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1467.4406 F50 X45.5 Y54.5 Z0.5 ; extrude filament
G1 E1467.5769 F50 X46.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1467.7133 F50 X45.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1467.7882 F50 X44 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1467.8383 F50 X44 Y56 Z0.5 ; extrude filament
G1 E1467.9746 F50 X43 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1468.111 F50 X44 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1468.1859 F50 X45.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1468.236 F50 X45.5 Y57.5 Z0.5 ; extrude filament
G1 E1468.3723 F50 X46.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1468.5087 F50 X45.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1468.5836 F50 X44 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1468.6337 F50 X44 Y59 Z0.5 ; extrude filament
G1 E1468.77 F50 X43 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1468.9064 F50 X44 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1468.9813 F50 X45.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1469.0314 F50 X45.5 Y60.5 Z0.5 ; extrude filament
G1 E1469.1677 F50 X46.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1469.3041 F50 X45.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1469.379 F50 X44 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1469.4291 F50 X44 Y62 Z0.5 ; extrude filament
G1 E1469.5654 F50 X43 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1469.7018 F50 X44 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1469.7767 F50 X45.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1469.8268 F50 X45.5 Y63.5 Z0.5 ; extrude filament
G1 E1469.9631 F50 X46.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1470.0995 F50 X45.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1470.1744 F50 X44 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1470.2245 F50 X44 Y65 Z0.5 ; extrude filament
G1 E1470.3608 F50 X43 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1470.4972 F50 X44 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1470.5721 F50 X45.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1470.6222 F50 X45.5 Y66.5 Z0.5 ; extrude filament
G1 E1470.7585 F50 X46.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1470.8949 F50 X45.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1470.9698 F50 X44 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1471.0199 F50 X44 Y68 Z0.5 ; extrude filament
G1 E1471.1563 F50 X43 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1471.2926 F50 X44 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1471.3676 F50 X45.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1471.4176 F50 X45.5 Y69.5 Z0.5 ; extrude filament
G1 E1471.554 F50 X46.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1471.6903 F50 X45.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1471.7653 F50 X44 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1471.8153 F50 X44 Y71 Z0.5 ; extrude filament
G1 E1471.9517 F50 X43 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1472.088 F50 X44 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1472.163 F50 X45.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1472.213 F50 X45.5 Y72.5 Z0.5 ; extrude filament
G1 E1472.3494 F50 X46.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1472.4857 F50 X45.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1472.5607 F50 X44 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1472.6107 F50 X44 Y74 Z0.5 ; extrude filament
G1 E1472.7471 F50 X43 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1472.8834 F50 X44 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1472.9584 F50 X45.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1473.0084 F50 X45.5 Y75.5 Z0.5 ; extrude filament
G1 E1473.1448 F50 X46.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1473.2811 F50 X45.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1473.3561 F50 X44 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1473.4061 F50 X44 Y77 Z0.5 ; extrude filament
G1 E1473.5425 F50 X43 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1473.6788 F50 X44 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1473.7538 F50 X45.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1473.8038 F50 X45.5 Y78.5 Z0.5 ; extrude filament
G1 E1473.9402 F50 X46.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1474.0765 F50 X45.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1474.1515 F50 X44 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1474.2015 F50 X44 Y80 Z0.5 ; extrude filament
G1 E1474.3379 F50 X43 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1474.4742 F50 X44 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1474.5492 F50 X45.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1474.5992 F50 X45.5 Y81.5 Z0.5 ; extrude filament
G1 E1474.7356 F50 X46.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1474.872 F50 X45.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1474.9469 F50 X44 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1474.997 F50 X44 Y83 Z0.5 ; extrude filament
G1 E1475.1333 F50 X43 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1475.2697 F50 X44 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1475.3446 F50 X45.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1475.3947 F50 X45.5 Y84.5 Z0.5 ; extrude filament
G1 E1475.531 F50 X46.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1475.6674 F50 X45.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1475.7423 F50 X44 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1475.7924 F50 X44 Y86 Z0.5 ; extrude filament
G1 E1475.9287 F50 X43 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1476.0651 F50 X44 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1476.14 F50 X45.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1476.1901 F50 X45.5 Y87.5 Z0.5 ; extrude filament
G1 E1476.3264 F50 X46.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1476.4628 F50 X45.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1476.5377 F50 X44 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1476.5878 F50 X44 Y89 Z0.5 ; extrude filament
G1 E1476.7241 F50 X43 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1476.8605 F50 X44 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1476.9354 F50 X45.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1476.9855 F50 X45.5 Y90.5 Z0.5 ; extrude filament
G1 E1477.1218 F50 X46.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1477.2582 F50 X45.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1477.3331 F50 X44 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1477.3832 F50 X44 Y92 Z0.5 ; extrude filament
G1 E1477.5195 F50 X43 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1477.6559 F50 X44 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1477.7308 F50 X45.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1477.7809 F50 X45.5 Y93.5 Z0.5 ; extrude filament
G1 E1477.9172 F50 X46.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1478.0536 F50 X45.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1478.1285 F50 X44 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1478.1786 F50 X44 Y95 Z0.5 ; extrude filament
G1 E1478.315 F50 X43 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1478.4513 F50 X44 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1478.5262 F50 X45.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1478.5763 F50 X45.5 Y96.5 Z0.5 ; extrude filament
G1 E1478.7126 F50 X46.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1478.849 F50 X45.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1478.924 F50 X44 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1478.974 F50 X44 Y98 Z0.5 ; extrude filament
G1 E1479.1104 F50 X43 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1479.2467 F50 X44 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1479.3217 F50 X45.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1479.3717 F50 X45.5 Y99.5 Z0.5 ; extrude filament
G1 E1479.508 F50 X46.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1479.6444 F50 X45.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1479.7194 F50 X44 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1479.7694 F50 X44 Y101 Z0.5 ; extrude filament
G1 E1479.9058 F50 X43 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1480.0421 F50 X44 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1480.1171 F50 X45.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1480.1671 F50 X45.5 Y102.5 Z0.5 ; extrude filament
G1 E1480.3035 F50 X46.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1480.4398 F50 X45.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1480.5148 F50 X44 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1480.5648 F50 X44 Y104 Z0.5 ; extrude filament
G1 E1480.7012 F50 X43 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1480.8375 F50 X44 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1480.9125 F50 X45.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1480.9625 F50 X45.5 Y105.5 Z0.5 ; extrude filament
G1 E1481.0989 F50 X46.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1481.2352 F50 X45.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1481.3102 F50 X44 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1481.3602 F50 X44 Y107 Z0.5 ; extrude filament
G1 E1481.4966 F50 X43 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1481.6329 F50 X44 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1481.7079 F50 X45.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1481.7579 F50 X45.5 Y108.5 Z0.5 ; extrude filament
G1 E1481.8943 F50 X46.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1482.0306 F50 X45.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1482.1056 F50 X44 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1482.1556 F50 X44 Y110 Z0.5 ; extrude filament
G1 E1482.292 F50 X43 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1482.4283 F50 X44 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1482.5033 F50 X45.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1482.5533 F50 X45.5 Y111.5 Z0.5 ; extrude filament
G1 E1482.6897 F50 X46.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1482.826 F50 X45.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1482.901 F50 X44 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1482.951 F50 X44 Y113 Z0.5 ; extrude filament
G1 E1483.0874 F50 X43 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1483.2238 F50 X44 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1483.2987 F50 X45.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1483.3488 F50 X45.5 Y114.5 Z0.5 ; extrude filament
G1 E1483.4851 F50 X46.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1483.6215 F50 X45.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1483.6964 F50 X44 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1483.7465 F50 X44 Y116 Z0.5 ; extrude filament
G1 E1483.8828 F50 X43 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1484.0192 F50 X44 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1484.0941 F50 X45.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1484.1442 F50 X45.5 Y117.5 Z0.5 ; extrude filament
G1 E1484.2805 F50 X46.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1484.4169 F50 X45.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1484.4918 F50 X44 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1484.5419 F50 X44 Y119 Z0.5 ; extrude filament
G1 E1481.5419 F5000 X44 Y119 Z0.5 ; retract filament
G0 E1481.5419 F5000 X44 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1481.5419 F5000 X50 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1481.5419 F5000 X50 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1484.5419 F50 X50 Y20 Z0.5 ; extrude filament
G1 E1484.6782 F50 X49 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1484.8146 F50 X50 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1484.8895 F50 X51.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1484.9396 F50 X51.5 Y21.5 Z0.5 ; extrude filament
G1 E1485.0759 F50 X52.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1485.2123 F50 X51.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1485.2872 F50 X50 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1485.3373 F50 X50 Y23 Z0.5 ; extrude filament
G1 E1485.4736 F50 X49 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1485.61 F50 X50 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1485.6849 F50 X51.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1485.735 F50 X51.5 Y24.5 Z0.5 ; extrude filament
G1 E1485.8713 F50 X52.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1486.0077 F50 X51.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1486.0826 F50 X50 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1486.1327 F50 X50 Y26 Z0.5 ; extrude filament
G1 E1486.269 F50 X49 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1486.4054 F50 X50 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1486.4803 F50 X51.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1486.5304 F50 X51.5 Y27.5 Z0.5 ; extrude filament
G1 E1486.6667 F50 X52.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1486.8031 F50 X51.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1486.878 F50 X50 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1486.9281 F50 X50 Y29 Z0.5 ; extrude filament
G1 E1487.0645 F50 X49 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1487.2008 F50 X50 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1487.2758 F50 X51.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1487.3258 F50 X51.5 Y30.5 Z0.5 ; extrude filament
G1 E1487.4622 F50 X52.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1487.5985 F50 X51.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1487.6735 F50 X50 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1487.7235 F50 X50 Y32 Z0.5 ; extrude filament
G1 E1487.8599 F50 X49 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1487.9962 F50 X50 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1488.0712 F50 X51.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1488.1212 F50 X51.5 Y33.5 Z0.5 ; extrude filament
G1 E1488.2576 F50 X52.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1488.3939 F50 X51.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1488.4689 F50 X50 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1488.5189 F50 X50 Y35 Z0.5 ; extrude filament
G1 E1488.6553 F50 X49 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1488.7916 F50 X50 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1488.8666 F50 X51.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1488.9166 F50 X51.5 Y36.5 Z0.5 ; extrude filament
G1 E1489.053 F50 X52.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1489.1893 F50 X51.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1489.2643 F50 X50 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1489.3143 F50 X50 Y38 Z0.5 ; extrude filament
G1 E1489.4507 F50 X49 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1489.587 F50 X50 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1489.662 F50 X51.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1489.712 F50 X51.5 Y39.5 Z0.5 ; extrude filament
G1 E1489.8484 F50 X52.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1489.9847 F50 X51.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1490.0597 F50 X50 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1490.1097 F50 X50 Y41 Z0.5 ; extrude filament
G1 E1490.2461 F50 X49 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1490.3824 F50 X50 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1490.4574 F50 X51.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1490.5074 F50 X51.5 Y42.5 Z0.5 ; extrude filament
G1 E1490.6438 F50 X52.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1490.7802 F50 X51.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1490.8551 F50 X50 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1490.9052 F50 X50 Y44 Z0.5 ; extrude filament
G1 E1491.0415 F50 X49 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1491.1779 F50 X50 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1491.2528 F50 X51.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1491.3029 F50 X51.5 Y45.5 Z0.5 ; extrude filament
G1 E1491.4392 F50 X52.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1491.5756 F50 X51.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1491.6505 F50 X50 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1491.7006 F50 X50 Y47 Z0.5 ; extrude filament
G1 E1491.8369 F50 X49 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1491.9733 F50 X50 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1492.0482 F50 X51.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1492.0983 F50 X51.5 Y48.5 Z0.5 ; extrude filament
G1 E1492.2346 F50 X52.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1492.371 F50 X51.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1492.4459 F50 X50 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1492.496 F50 X50 Y50 Z0.5 ; extrude filament
G1 E1492.6323 F50 X49 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1492.7687 F50 X50 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1492.8436 F50 X51.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1492.8937 F50 X51.5 Y51.5 Z0.5 ; extrude filament
G1 E1493.03 F50 X52.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1493.1664 F50 X51.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1493.2413 F50 X50 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1493.2914 F50 X50 Y53 Z0.5 ; extrude filament
G1 E1493.4277 F50 X49 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1493.5641 F50 X50 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1493.639 F50 X51.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1493.6891 F50 X51.5 Y54.5 Z0.5 ; extrude filament
G1 E1493.8254 F50 X52.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1493.9618 F50 X51.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1494.0367 F50 X50 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1494.0868 F50 X50 Y56 Z0.5 ; extrude filament
G1 E1494.2231 F50 X49 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1494.3595 F50 X50 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1494.4344 F50 X51.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1494.4845 F50 X51.5 Y57.5 Z0.5 ; extrude filament
G1 E1494.6208 F50 X52.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1494.7572 F50 X51.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1494.8322 F50 X50 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1494.8822 F50 X50 Y59 Z0.5 ; extrude filament
G1 E1495.0186 F50 X49 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1495.1549 F50 X50 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1495.2299 F50 X51.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1495.2799 F50 X51.5 Y60.5 Z0.5 ; extrude filament
G1 E1495.4163 F50 X52.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1495.5526 F50 X51.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1495.6276 F50 X50 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1495.6776 F50 X50 Y62 Z0.5 ; extrude filament
G1 E1495.814 F50 X49 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1495.9503 F50 X50 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1496.0253 F50 X51.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1496.0753 F50 X51.5 Y63.5 Z0.5 ; extrude filament
G1 E1496.2117 F50 X52.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1496.348 F50 X51.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1496.423 F50 X50 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1496.473 F50 X50 Y65 Z0.5 ; extrude filament
G1 E1496.6094 F50 X49 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1496.7457 F50 X50 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1496.8207 F50 X51.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1496.8707 F50 X51.5 Y66.5 Z0.5 ; extrude filament
G1 E1497.0071 F50 X52.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1497.1434 F50 X51.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1497.2184 F50 X50 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1497.2684 F50 X50 Y68 Z0.5 ; extrude filament
G1 E1497.4048 F50 X49 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1497.5411 F50 X50 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1497.6161 F50 X51.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1497.6661 F50 X51.5 Y69.5 Z0.5 ; extrude filament
G1 E1497.8025 F50 X52.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1497.9388 F50 X51.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1498.0138 F50 X50 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1498.0638 F50 X50 Y71 Z0.5 ; extrude filament
G1 E1498.2002 F50 X49 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1498.3365 F50 X50 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1498.4115 F50 X51.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1498.4615 F50 X51.5 Y72.5 Z0.5 ; extrude filament
G1 E1498.5979 F50 X52.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1498.7343 F50 X51.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1498.8092 F50 X50 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1498.8593 F50 X50 Y74 Z0.5 ; extrude filament
G1 E1498.9956 F50 X49 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1499.132 F50 X50 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1499.2069 F50 X51.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1499.257 F50 X51.5 Y75.5 Z0.5 ; extrude filament
G1 E1499.3933 F50 X52.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1499.5297 F50 X51.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1499.6046 F50 X50 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1499.6547 F50 X50 Y77 Z0.5 ; extrude filament
G1 E1499.791 F50 X49 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1499.9274 F50 X50 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1500.0023 F50 X51.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1500.0524 F50 X51.5 Y78.5 Z0.5 ; extrude filament
G1 E1500.1887 F50 X52.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1500.3251 F50 X51.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1500.4 F50 X50 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1500.4501 F50 X50 Y80 Z0.5 ; extrude filament
G1 E1500.5864 F50 X49 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1500.7228 F50 X50 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1500.7977 F50 X51.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1500.8478 F50 X51.5 Y81.5 Z0.5 ; extrude filament
G1 E1500.9841 F50 X52.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1501.1205 F50 X51.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1501.1954 F50 X50 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1501.2455 F50 X50 Y83 Z0.5 ; extrude filament
G1 E1501.3818 F50 X49 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1501.5182 F50 X50 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1501.5931 F50 X51.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1501.6432 F50 X51.5 Y84.5 Z0.5 ; extrude filament
G1 E1501.7795 F50 X52.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1501.9159 F50 X51.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1501.9908 F50 X50 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1502.0409 F50 X50 Y86 Z0.5 ; extrude filament
G1 E1502.1772 F50 X49 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1502.3136 F50 X50 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1502.3885 F50 X51.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1502.4386 F50 X51.5 Y87.5 Z0.5 ; extrude filament
G1 E1502.575 F50 X52.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1502.7113 F50 X51.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1502.7863 F50 X50 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1502.8363 F50 X50 Y89 Z0.5 ; extrude filament
G1 E1502.9727 F50 X49 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1503.109 F50 X50 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1503.184 F50 X51.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1503.234 F50 X51.5 Y90.5 Z0.5 ; extrude filament
G1 E1503.3704 F50 X52.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1503.5067 F50 X51.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1503.5817 F50 X50 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1503.6317 F50 X50 Y92 Z0.5 ; extrude filament
G1 E1503.7681 F50 X49 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1503.9044 F50 X50 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1503.9794 F50 X51.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1504.0294 F50 X51.5 Y93.5 Z0.5 ; extrude filament
G1 E1504.1658 F50 X52.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1504.3021 F50 X51.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1504.3771 F50 X50 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1504.4271 F50 X50 Y95 Z0.5 ; extrude filament
G1 E1504.5635 F50 X49 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1504.6998 F50 X50 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1504.7748 F50 X51.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1504.8248 F50 X51.5 Y96.5 Z0.5 ; extrude filament
G1 E1504.9612 F50 X52.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1505.0975 F50 X51.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1505.1725 F50 X50 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1505.2225 F50 X50 Y98 Z0.5 ; extrude filament
G1 E1505.3589 F50 X49 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1505.4952 F50 X50 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1505.5702 F50 X51.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1505.6202 F50 X51.5 Y99.5 Z0.5 ; extrude filament
G1 E1505.7566 F50 X52.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1505.893 F50 X51.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1505.9679 F50 X50 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1506.018 F50 X50 Y101 Z0.5 ; extrude filament
G1 E1506.1543 F50 X49 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1506.2906 F50 X50 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1506.3656 F50 X51.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1506.4156 F50 X51.5 Y102.5 Z0.5 ; extrude filament
G1 E1506.552 F50 X52.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1506.6884 F50 X51.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1506.7633 F50 X50 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1506.8134 F50 X50 Y104 Z0.5 ; extrude filament
G1 E1506.9497 F50 X49 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1507.086 F50 X50 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1507.161 F50 X51.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1507.211 F50 X51.5 Y105.5 Z0.5 ; extrude filament
G1 E1507.3474 F50 X52.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1507.4838 F50 X51.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1507.5587 F50 X50 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1507.6088 F50 X50 Y107 Z0.5 ; extrude filament
G1 E1507.7451 F50 X49 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1507.8815 F50 X50 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1507.9564 F50 X51.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1508.0065 F50 X51.5 Y108.5 Z0.5 ; extrude filament
G1 E1508.1428 F50 X52.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1508.2792 F50 X51.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1508.3541 F50 X50 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1508.4042 F50 X50 Y110 Z0.5 ; extrude filament
G1 E1508.5405 F50 X49 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1508.6769 F50 X50 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1508.7518 F50 X51.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1508.8019 F50 X51.5 Y111.5 Z0.5 ; extrude filament
G1 E1508.9382 F50 X52.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1509.0746 F50 X51.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1509.1495 F50 X50 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1509.1996 F50 X50 Y113 Z0.5 ; extrude filament
G1 E1509.3359 F50 X49 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1509.4723 F50 X50 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1509.5472 F50 X51.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1509.5973 F50 X51.5 Y114.5 Z0.5 ; extrude filament
G1 E1509.7336 F50 X52.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1509.87 F50 X51.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1509.945 F50 X50 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1509.995 F50 X50 Y116 Z0.5 ; extrude filament
G1 E1510.1313 F50 X49 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1510.2677 F50 X50 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1510.3427 F50 X51.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1510.3927 F50 X51.5 Y117.5 Z0.5 ; extrude filament
G1 E1510.529 F50 X52.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1510.6654 F50 X51.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1510.7404 F50 X50 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1510.7904 F50 X50 Y119 Z0.5 ; extrude filament
G1 E1507.7904 F5000 X50 Y119 Z0.5 ; retract filament
G0 E1507.7904 F5000 X50 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1507.7904 F5000 X56 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1507.7904 F5000 X56 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1510.7904 F50 X56 Y20 Z0.5 ; extrude filament
G1 E1510.9268 F50 X55 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1511.0631 F50 X56 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1511.1381 F50 X57.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1511.1881 F50 X57.5 Y21.5 Z0.5 ; extrude filament
G1 E1511.3245 F50 X58.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1511.4608 F50 X57.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1511.5358 F50 X56 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1511.5858 F50 X56 Y23 Z0.5 ; extrude filament
G1 E1511.7222 F50 X55 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1511.8585 F50 X56 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1511.9335 F50 X57.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1511.9835 F50 X57.5 Y24.5 Z0.5 ; extrude filament
G1 E1512.1199 F50 X58.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1512.2562 F50 X57.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1512.3312 F50 X56 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1512.3812 F50 X56 Y26 Z0.5 ; extrude filament
G1 E1512.5176 F50 X55 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1512.6539 F50 X56 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1512.7289 F50 X57.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1512.7789 F50 X57.5 Y27.5 Z0.5 ; extrude filament
G1 E1512.9153 F50 X58.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1513.0516 F50 X57.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1513.1266 F50 X56 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1513.1766 F50 X56 Y29 Z0.5 ; extrude filament
G1 E1513.313 F50 X55 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1513.4493 F50 X56 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1513.5243 F50 X57.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1513.5743 F50 X57.5 Y30.5 Z0.5 ; extrude filament
G1 E1513.7107 F50 X58.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1513.847 F50 X57.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1513.922 F50 X56 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1513.972 F50 X56 Y32 Z0.5 ; extrude filament
G1 E1514.1084 F50 X55 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1514.2448 F50 X56 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1514.3197 F50 X57.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1514.3698 F50 X57.5 Y33.5 Z0.5 ; extrude filament
G1 E1514.5061 F50 X58.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1514.6425 F50 X57.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1514.7174 F50 X56 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1514.7675 F50 X56 Y35 Z0.5 ; extrude filament
G1 E1514.9038 F50 X55 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1515.0402 F50 X56 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1515.1151 F50 X57.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1515.1652 F50 X57.5 Y36.5 Z0.5 ; extrude filament
G1 E1515.3015 F50 X58.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1515.4379 F50 X57.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1515.5128 F50 X56 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1515.5629 F50 X56 Y38 Z0.5 ; extrude filament
G1 E1515.6992 F50 X55 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1515.8356 F50 X56 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1515.9105 F50 X57.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1515.9606 F50 X57.5 Y39.5 Z0.5 ; extrude filament
G1 E1516.0969 F50 X58.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1516.2333 F50 X57.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1516.3082 F50 X56 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1516.3583 F50 X56 Y41 Z0.5 ; extrude filament
G1 E1516.4946 F50 X55 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1516.631 F50 X56 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1516.7059 F50 X57.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1516.756 F50 X57.5 Y42.5 Z0.5 ; extrude filament
G1 E1516.8923 F50 X58.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1517.0287 F50 X57.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1517.1036 F50 X56 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1517.1537 F50 X56 Y44 Z0.5 ; extrude filament
G1 E1517.29 F50 X55 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1517.4264 F50 X56 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1517.5013 F50 X57.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1517.5514 F50 X57.5 Y45.5 Z0.5 ; extrude filament
G1 E1517.6877 F50 X58.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1517.8241 F50 X57.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1517.899 F50 X56 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1517.9491 F50 X56 Y47 Z0.5 ; extrude filament
G1 E1518.0854 F50 X55 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1518.2218 F50 X56 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1518.2968 F50 X57.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1518.3468 F50 X57.5 Y48.5 Z0.5 ; extrude filament
G1 E1518.4832 F50 X58.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1518.6195 F50 X57.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1518.6945 F50 X56 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1518.7445 F50 X56 Y50 Z0.5 ; extrude filament
G1 E1518.8809 F50 X55 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1519.0172 F50 X56 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1519.0922 F50 X57.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1519.1422 F50 X57.5 Y51.5 Z0.5 ; extrude filament
G1 E1519.2786 F50 X58.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1519.4149 F50 X57.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1519.4899 F50 X56 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1519.5399 F50 X56 Y53 Z0.5 ; extrude filament
G1 E1519.6763 F50 X55 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1519.8126 F50 X56 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1519.8876 F50 X57.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1519.9376 F50 X57.5 Y54.5 Z0.5 ; extrude filament
G1 E1520.074 F50 X58.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1520.2103 F50 X57.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1520.2853 F50 X56 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1520.3353 F50 X56 Y56 Z0.5 ; extrude filament
G1 E1520.4717 F50 X55 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1520.608 F50 X56 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1520.683 F50 X57.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1520.733 F50 X57.5 Y57.5 Z0.5 ; extrude filament
G1 E1520.8694 F50 X58.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1521.0057 F50 X57.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1521.0807 F50 X56 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1521.1307 F50 X56 Y59 Z0.5 ; extrude filament
G1 E1521.2671 F50 X55 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1521.4034 F50 X56 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1521.4784 F50 X57.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1521.5284 F50 X57.5 Y60.5 Z0.5 ; extrude filament
G1 E1521.6648 F50 X58.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1521.8011 F50 X57.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1521.8761 F50 X56 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1521.9261 F50 X56 Y62 Z0.5 ; extrude filament
G1 E1522.0625 F50 X55 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1522.1989 F50 X56 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1522.2738 F50 X57.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1522.3239 F50 X57.5 Y63.5 Z0.5 ; extrude filament
G1 E1522.4602 F50 X58.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1522.5966 F50 X57.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1522.6715 F50 X56 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1522.7216 F50 X56 Y65 Z0.5 ; extrude filament
G1 E1522.8579 F50 X55 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1522.9943 F50 X56 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1523.0692 F50 X57.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1523.1193 F50 X57.5 Y66.5 Z0.5 ; extrude filament
G1 E1523.2556 F50 X58.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1523.392 F50 X57.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1523.4669 F50 X56 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1523.517 F50 X56 Y68 Z0.5 ; extrude filament
G1 E1523.6533 F50 X55 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1523.7897 F50 X56 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1523.8646 F50 X57.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1523.9147 F50 X57.5 Y69.5 Z0.5 ; extrude filament
G1 E1524.051 F50 X58.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1524.1874 F50 X57.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1524.2623 F50 X56 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1524.3124 F50 X56 Y71 Z0.5 ; extrude filament
G1 E1524.4487 F50 X55 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1524.5851 F50 X56 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1524.66 F50 X57.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1524.7101 F50 X57.5 Y72.5 Z0.5 ; extrude filament
G1 E1524.8464 F50 X58.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1524.9828 F50 X57.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1525.0577 F50 X56 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1525.1078 F50 X56 Y74 Z0.5 ; extrude filament
G1 E1525.2441 F50 X55 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1525.3805 F50 X56 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1525.4554 F50 X57.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1525.5055 F50 X57.5 Y75.5 Z0.5 ; extrude filament
G1 E1525.6418 F50 X58.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1525.7782 F50 X57.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1525.8531 F50 X56 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1525.9032 F50 X56 Y77 Z0.5 ; extrude filament
G1 E1526.0396 F50 X55 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1526.1759 F50 X56 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1526.2509 F50 X57.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1526.3009 F50 X57.5 Y78.5 Z0.5 ; extrude filament
G1 E1526.4373 F50 X58.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1526.5736 F50 X57.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1526.6486 F50 X56 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1526.6986 F50 X56 Y80 Z0.5 ; extrude filament
G1 E1526.835 F50 X55 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1526.9713 F50 X56 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1527.0463 F50 X57.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1527.0963 F50 X57.5 Y81.5 Z0.5 ; extrude filament
G1 E1527.2327 F50 X58.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1527.369 F50 X57.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1527.444 F50 X56 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1527.494 F50 X56 Y83 Z0.5 ; extrude filament
G1 E1527.6304 F50 X55 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1527.7667 F50 X56 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1527.8417 F50 X57.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1527.8917 F50 X57.5 Y84.5 Z0.5 ; extrude filament
G1 E1528.0281 F50 X58.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1528.1644 F50 X57.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1528.2394 F50 X56 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1528.2894 F50 X56 Y86 Z0.5 ; extrude filament
G1 E1528.4258 F50 X55 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1528.5621 F50 X56 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1528.6371 F50 X57.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1528.6871 F50 X57.5 Y87.5 Z0.5 ; extrude filament
G1 E1528.8235 F50 X58.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1528.9598 F50 X57.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1529.0348 F50 X56 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1529.0848 F50 X56 Y89 Z0.5 ; extrude filament
G1 E1529.2212 F50 X55 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1529.3575 F50 X56 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1529.4325 F50 X57.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1529.4825 F50 X57.5 Y90.5 Z0.5 ; extrude filament
G1 E1529.6189 F50 X58.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1529.7552 F50 X57.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1529.8302 F50 X56 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1529.8802 F50 X56 Y92 Z0.5 ; extrude filament
G1 E1530.0166 F50 X55 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1530.153 F50 X56 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1530.2279 F50 X57.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1530.278 F50 X57.5 Y93.5 Z0.5 ; extrude filament
G1 E1530.4143 F50 X58.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1530.5507 F50 X57.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1530.6256 F50 X56 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1530.6757 F50 X56 Y95 Z0.5 ; extrude filament
G1 E1530.812 F50 X55 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1530.9484 F50 X56 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1531.0233 F50 X57.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1531.0734 F50 X57.5 Y96.5 Z0.5 ; extrude filament
G1 E1531.2097 F50 X58.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1531.3461 F50 X57.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1531.421 F50 X56 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1531.4711 F50 X56 Y98 Z0.5 ; extrude filament
G1 E1531.6074 F50 X55 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1531.7438 F50 X56 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1531.8187 F50 X57.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1531.8688 F50 X57.5 Y99.5 Z0.5 ; extrude filament
G1 E1532.0051 F50 X58.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1532.1415 F50 X57.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1532.2164 F50 X56 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1532.2665 F50 X56 Y101 Z0.5 ; extrude filament
G1 E1532.4028 F50 X55 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1532.5392 F50 X56 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1532.6141 F50 X57.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1532.6642 F50 X57.5 Y102.5 Z0.5 ; extrude filament
G1 E1532.8005 F50 X58.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1532.9369 F50 X57.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1533.0118 F50 X56 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1533.0619 F50 X56 Y104 Z0.5 ; extrude filament
G1 E1533.1982 F50 X55 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1533.3346 F50 X56 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1533.4095 F50 X57.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1533.4596 F50 X57.5 Y105.5 Z0.5 ; extrude filament
G1 E1533.596 F50 X58.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1533.7323 F50 X57.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1533.8073 F50 X56 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1533.8573 F50 X56 Y107 Z0.5 ; extrude filament
G1 E1533.9937 F50 X55 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1534.13 F50 X56 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1534.205 F50 X57.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1534.255 F50 X57.5 Y108.5 Z0.5 ; extrude filament
G1 E1534.3914 F50 X58.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1534.5277 F50 X57.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1534.6027 F50 X56 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1534.6527 F50 X56 Y110 Z0.5 ; extrude filament
G1 E1534.7891 F50 X55 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1534.9254 F50 X56 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1535.0004 F50 X57.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1535.0504 F50 X57.5 Y111.5 Z0.5 ; extrude filament
G1 E1535.1868 F50 X58.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1535.3231 F50 X57.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1535.3981 F50 X56 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1535.4481 F50 X56 Y113 Z0.5 ; extrude filament
G1 E1535.5845 F50 X55 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1535.7208 F50 X56 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1535.7958 F50 X57.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1535.8458 F50 X57.5 Y114.5 Z0.5 ; extrude filament
G1 E1535.9822 F50 X58.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1536.1185 F50 X57.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1536.1935 F50 X56 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1536.2435 F50 X56 Y116 Z0.5 ; extrude filament
G1 E1536.3799 F50 X55 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1536.5162 F50 X56 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1536.5912 F50 X57.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1536.6412 F50 X57.5 Y117.5 Z0.5 ; extrude filament
G1 E1536.7776 F50 X58.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1536.914 F50 X57.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1536.9889 F50 X56 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1537.039 F50 X56 Y119 Z0.5 ; extrude filament
G1 E1534.039 F5000 X56 Y119 Z0.5 ; retract filament
G0 E1534.039 F5000 X56 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1534.039 F5000 X62 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1534.039 F5000 X62 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1537.039 F50 X62 Y20 Z0.5 ; extrude filament
G1 E1537.1753 F50 X61 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1537.3116 F50 X62 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1537.3866 F50 X63.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1537.4366 F50 X63.5 Y21.5 Z0.5 ; extrude filament
G1 E1537.573 F50 X64.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1537.7094 F50 X63.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1537.7843 F50 X62 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1537.8344 F50 X62 Y23 Z0.5 ; extrude filament
G1 E1537.9707 F50 X61 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1538.107 F50 X62 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1538.182 F50 X63.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1538.232 F50 X63.5 Y24.5 Z0.5 ; extrude filament
G1 E1538.3684 F50 X64.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1538.5048 F50 X63.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1538.5797 F50 X62 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1538.6298 F50 X62 Y26 Z0.5 ; extrude filament
G1 E1538.7661 F50 X61 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1538.9025 F50 X62 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1538.9774 F50 X63.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1539.0275 F50 X63.5 Y27.5 Z0.5 ; extrude filament
G1 E1539.1638 F50 X64.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1539.3002 F50 X63.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1539.3751 F50 X62 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1539.4252 F50 X62 Y29 Z0.5 ; extrude filament
G1 E1539.5615 F50 X61 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1539.6979 F50 X62 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1539.7728 F50 X63.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1539.8229 F50 X63.5 Y30.5 Z0.5 ; extrude filament
G1 E1539.9592 F50 X64.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1540.0956 F50 X63.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1540.1705 F50 X62 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1540.2206 F50 X62 Y32 Z0.5 ; extrude filament
G1 E1540.3569 F50 X61 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1540.4933 F50 X62 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1540.5682 F50 X63.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1540.6183 F50 X63.5 Y33.5 Z0.5 ; extrude filament
G1 E1540.7546 F50 X64.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1540.891 F50 X63.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1540.966 F50 X62 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1541.016 F50 X62 Y35 Z0.5 ; extrude filament
G1 E1541.1523 F50 X61 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1541.2887 F50 X62 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1541.3636 F50 X63.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1541.4137 F50 X63.5 Y36.5 Z0.5 ; extrude filament
G1 E1541.55 F50 X64.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1541.6864 F50 X63.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1541.7614 F50 X62 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1541.8114 F50 X62 Y38 Z0.5 ; extrude filament
G1 E1541.9478 F50 X61 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1542.0841 F50 X62 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1542.159 F50 X63.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1542.2091 F50 X63.5 Y39.5 Z0.5 ; extrude filament
G1 E1542.3455 F50 X64.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1542.4818 F50 X63.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1542.5568 F50 X62 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1542.6068 F50 X62 Y41 Z0.5 ; extrude filament
G1 E1542.7432 F50 X61 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1542.8795 F50 X62 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1542.9545 F50 X63.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1543.0045 F50 X63.5 Y42.5 Z0.5 ; extrude filament
G1 E1543.1409 F50 X64.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1543.2772 F50 X63.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1543.3522 F50 X62 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1543.4022 F50 X62 Y44 Z0.5 ; extrude filament
G1 E1543.5386 F50 X61 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1543.6749 F50 X62 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1543.7499 F50 X63.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1543.7999 F50 X63.5 Y45.5 Z0.5 ; extrude filament
G1 E1543.9363 F50 X64.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1544.0726 F50 X63.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1544.1476 F50 X62 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1544.1976 F50 X62 Y47 Z0.5 ; extrude filament
G1 E1544.334 F50 X61 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1544.4703 F50 X62 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1544.5453 F50 X63.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1544.5953 F50 X63.5 Y48.5 Z0.5 ; extrude filament
G1 E1544.7317 F50 X64.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1544.868 F50 X63.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1544.943 F50 X62 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1544.993 F50 X62 Y50 Z0.5 ; extrude filament
G1 E1545.1294 F50 X61 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1545.2657 F50 X62 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1545.3407 F50 X63.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1545.3907 F50 X63.5 Y51.5 Z0.5 ; extrude filament
G1 E1545.5271 F50 X64.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1545.6635 F50 X63.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1545.7384 F50 X62 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1545.7885 F50 X62 Y53 Z0.5 ; extrude filament
G1 E1545.9248 F50 X61 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1546.0612 F50 X62 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1546.1361 F50 X63.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1546.1862 F50 X63.5 Y54.5 Z0.5 ; extrude filament
G1 E1546.3225 F50 X64.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1546.4589 F50 X63.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1546.5338 F50 X62 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1546.5839 F50 X62 Y56 Z0.5 ; extrude filament
G1 E1546.7202 F50 X61 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1546.8566 F50 X62 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1546.9315 F50 X63.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1546.9816 F50 X63.5 Y57.5 Z0.5 ; extrude filament
G1 E1547.1179 F50 X64.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1547.2543 F50 X63.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1547.3292 F50 X62 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1547.3793 F50 X62 Y59 Z0.5 ; extrude filament
G1 E1547.5156 F50 X61 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1547.652 F50 X62 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1547.7269 F50 X63.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1547.777 F50 X63.5 Y60.5 Z0.5 ; extrude filament
G1 E1547.9133 F50 X64.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1548.0497 F50 X63.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1548.1246 F50 X62 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1548.1747 F50 X62 Y62 Z0.5 ; extrude filament
G1 E1548.311 F50 X61 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1548.4474 F50 X62 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1548.5223 F50 X63.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1548.5724 F50 X63.5 Y63.5 Z0.5 ; extrude filament
G1 E1548.7087 F50 X64.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1548.8451 F50 X63.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1548.92 F50 X62 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1548.9701 F50 X62 Y65 Z0.5 ; extrude filament
G1 E1549.1064 F50 X61 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1549.2428 F50 X62 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1549.3177 F50 X63.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1549.3678 F50 X63.5 Y66.5 Z0.5 ; extrude filament
G1 E1549.5042 F50 X64.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1549.6405 F50 X63.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1549.7155 F50 X62 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1549.7655 F50 X62 Y68 Z0.5 ; extrude filament
G1 E1549.9019 F50 X61 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1550.0382 F50 X62 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1550.1132 F50 X63.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1550.1632 F50 X63.5 Y69.5 Z0.5 ; extrude filament
G1 E1550.2996 F50 X64.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1550.4359 F50 X63.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1550.5109 F50 X62 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1550.5609 F50 X62 Y71 Z0.5 ; extrude filament
G1 E1550.6973 F50 X61 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1550.8336 F50 X62 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1550.9086 F50 X63.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1550.9586 F50 X63.5 Y72.5 Z0.5 ; extrude filament
G1 E1551.095 F50 X64.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1551.2313 F50 X63.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1551.3063 F50 X62 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1551.3563 F50 X62 Y74 Z0.5 ; extrude filament
G1 E1551.4927 F50 X61 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1551.629 F50 X62 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1551.704 F50 X63.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1551.754 F50 X63.5 Y75.5 Z0.5 ; extrude filament
G1 E1551.8904 F50 X64.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1552.0267 F50 X63.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1552.1017 F50 X62 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1552.1517 F50 X62 Y77 Z0.5 ; extrude filament
G1 E1552.2881 F50 X61 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1552.4244 F50 X62 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1552.4994 F50 X63.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1552.5494 F50 X63.5 Y78.5 Z0.5 ; extrude filament
G1 E1552.6858 F50 X64.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1552.8221 F50 X63.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1552.8971 F50 X62 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1552.9471 F50 X62 Y80 Z0.5 ; extrude filament
G1 E1553.0835 F50 X61 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1553.2198 F50 X62 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1553.2948 F50 X63.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1553.3448 F50 X63.5 Y81.5 Z0.5 ; extrude filament
G1 E1553.4812 F50 X64.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1553.6176 F50 X63.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1553.6925 F50 X62 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1553.7426 F50 X62 Y83 Z0.5 ; extrude filament
G1 E1553.8789 F50 X61 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1554.0153 F50 X62 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1554.0902 F50 X63.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1554.1403 F50 X63.5 Y84.5 Z0.5 ; extrude filament
G1 E1554.2766 F50 X64.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1554.413 F50 X63.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1554.4879 F50 X62 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1554.538 F50 X62 Y86 Z0.5 ; extrude filament
G1 E1554.6743 F50 X61 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1554.8107 F50 X62 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1554.8856 F50 X63.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1554.9357 F50 X63.5 Y87.5 Z0.5 ; extrude filament
G1 E1555.072 F50 X64.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1555.2084 F50 X63.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1555.2833 F50 X62 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1555.3334 F50 X62 Y89 Z0.5 ; extrude filament
G1 E1555.4697 F50 X61 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1555.6061 F50 X62 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1555.681 F50 X63.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1555.7311 F50 X63.5 Y90.5 Z0.5 ; extrude filament
G1 E1555.8674 F50 X64.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1556.0038 F50 X63.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1556.0787 F50 X62 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1556.1288 F50 X62 Y92 Z0.5 ; extrude filament
G1 E1556.2651 F50 X61 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1556.4015 F50 X62 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1556.4764 F50 X63.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1556.5265 F50 X63.5 Y93.5 Z0.5 ; extrude filament
G1 E1556.6628 F50 X64.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1556.7992 F50 X63.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1556.8741 F50 X62 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1556.9242 F50 X62 Y95 Z0.5 ; extrude filament
G1 E1557.0605 F50 X61 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1557.1969 F50 X62 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1557.2719 F50 X63.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1557.3219 F50 X63.5 Y96.5 Z0.5 ; extrude filament
G1 E1557.4583 F50 X64.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1557.5946 F50 X63.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1557.6696 F50 X62 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1557.7196 F50 X62 Y98 Z0.5 ; extrude filament
G1 E1557.856 F50 X61 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1557.9923 F50 X62 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1558.0673 F50 X63.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1558.1173 F50 X63.5 Y99.5 Z0.5 ; extrude filament
G1 E1558.2537 F50 X64.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1558.39 F50 X63.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1558.465 F50 X62 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1558.515 F50 X62 Y101 Z0.5 ; extrude filament
G1 E1558.6514 F50 X61 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1558.7877 F50 X62 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1558.8627 F50 X63.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1558.9127 F50 X63.5 Y102.5 Z0.5 ; extrude filament
G1 E1559.0491 F50 X64.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1559.1854 F50 X63.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1559.2604 F50 X62 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1559.3104 F50 X62 Y104 Z0.5 ; extrude filament
G1 E1559.4468 F50 X61 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1559.5831 F50 X62 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1559.6581 F50 X63.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1559.7081 F50 X63.5 Y105.5 Z0.5 ; extrude filament
G1 E1559.8445 F50 X64.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1559.9808 F50 X63.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1560.0558 F50 X62 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1560.1058 F50 X62 Y107 Z0.5 ; extrude filament
G1 E1560.2422 F50 X61 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1560.3785 F50 X62 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1560.4535 F50 X63.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1560.5035 F50 X63.5 Y108.5 Z0.5 ; extrude filament
G1 E1560.6399 F50 X64.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1560.7762 F50 X63.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1560.8512 F50 X62 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1560.9012 F50 X62 Y110 Z0.5 ; extrude filament
G1 E1561.0376 F50 X61 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1561.174 F50 X62 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1561.2489 F50 X63.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1561.299 F50 X63.5 Y111.5 Z0.5 ; extrude filament
G1 E1561.4353 F50 X64.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1561.5717 F50 X63.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1561.6466 F50 X62 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1561.6967 F50 X62 Y113 Z0.5 ; extrude filament
G1 E1561.833 F50 X61 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1561.9694 F50 X62 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1562.0443 F50 X63.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1562.0944 F50 X63.5 Y114.5 Z0.5 ; extrude filament
G1 E1562.2307 F50 X64.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1562.3671 F50 X63.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1562.442 F50 X62 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1562.4921 F50 X62 Y116 Z0.5 ; extrude filament
G1 E1562.6284 F50 X61 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1562.7648 F50 X62 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1562.8397 F50 X63.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1562.8898 F50 X63.5 Y117.5 Z0.5 ; extrude filament
G1 E1563.0261 F50 X64.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1563.1625 F50 X63.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1563.2374 F50 X62 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1563.2875 F50 X62 Y119 Z0.5 ; extrude filament
G1 E1560.2875 F5000 X62 Y119 Z0.5 ; retract filament
G0 E1560.2875 F5000 X62 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1560.2875 F5000 X68 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1560.2875 F5000 X68 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1563.2875 F50 X68 Y20 Z0.5 ; extrude filament
G1 E1563.4238 F50 X67 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1563.5602 F50 X68 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1563.6351 F50 X69.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1563.6852 F50 X69.5 Y21.5 Z0.5 ; extrude filament
G1 E1563.8215 F50 X70.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1563.9579 F50 X69.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1564.0328 F50 X68 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1564.0829 F50 X68 Y23 Z0.5 ; extrude filament
G1 E1564.2192 F50 X67 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1564.3556 F50 X68 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1564.4305 F50 X69.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1564.4806 F50 X69.5 Y24.5 Z0.5 ; extrude filament
G1 E1564.617 F50 X70.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1564.7533 F50 X69.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1564.8282 F50 X68 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1564.8783 F50 X68 Y26 Z0.5 ; extrude filament
G1 E1565.0146 F50 X67 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1565.151 F50 X68 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1565.226 F50 X69.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1565.276 F50 X69.5 Y27.5 Z0.5 ; extrude filament
G1 E1565.4124 F50 X70.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1565.5487 F50 X69.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1565.6237 F50 X68 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1565.6737 F50 X68 Y29 Z0.5 ; extrude filament
G1 E1565.81 F50 X67 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1565.9464 F50 X68 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1566.0214 F50 X69.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1566.0714 F50 X69.5 Y30.5 Z0.5 ; extrude filament
G1 E1566.2078 F50 X70.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1566.3441 F50 X69.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1566.4191 F50 X68 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1566.4691 F50 X68 Y32 Z0.5 ; extrude filament
G1 E1566.6055 F50 X67 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1566.7418 F50 X68 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1566.8168 F50 X69.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1566.8668 F50 X69.5 Y33.5 Z0.5 ; extrude filament
G1 E1567.0032 F50 X70.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1567.1395 F50 X69.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1567.2145 F50 X68 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1567.2645 F50 X68 Y35 Z0.5 ; extrude filament
G1 E1567.4009 F50 X67 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1567.5372 F50 X68 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1567.6122 F50 X69.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1567.6622 F50 X69.5 Y36.5 Z0.5 ; extrude filament
G1 E1567.7986 F50 X70.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1567.9349 F50 X69.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1568.0099 F50 X68 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1568.0599 F50 X68 Y38 Z0.5 ; extrude filament
G1 E1568.1963 F50 X67 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1568.3326 F50 X68 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1568.4076 F50 X69.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1568.4576 F50 X69.5 Y39.5 Z0.5 ; extrude filament
G1 E1568.594 F50 X70.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1568.7303 F50 X69.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1568.8053 F50 X68 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1568.8553 F50 X68 Y41 Z0.5 ; extrude filament
G1 E1568.9917 F50 X67 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1569.128 F50 X68 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1569.203 F50 X69.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1569.253 F50 X69.5 Y42.5 Z0.5 ; extrude filament
G1 E1569.3894 F50 X70.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1569.5258 F50 X69.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1569.6007 F50 X68 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1569.6508 F50 X68 Y44 Z0.5 ; extrude filament
G1 E1569.7871 F50 X67 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1569.9235 F50 X68 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1569.9984 F50 X69.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1570.0485 F50 X69.5 Y45.5 Z0.5 ; extrude filament
G1 E1570.1848 F50 X70.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1570.3212 F50 X69.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1570.3961 F50 X68 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1570.4462 F50 X68 Y47 Z0.5 ; extrude filament
G1 E1570.5825 F50 X67 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1570.7189 F50 X68 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1570.7938 F50 X69.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1570.8439 F50 X69.5 Y48.5 Z0.5 ; extrude filament
G1 E1570.9802 F50 X70.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1571.1166 F50 X69.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1571.1915 F50 X68 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1571.2416 F50 X68 Y50 Z0.5 ; extrude filament
G1 E1571.3779 F50 X67 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1571.5143 F50 X68 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1571.5892 F50 X69.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1571.6393 F50 X69.5 Y51.5 Z0.5 ; extrude filament
G1 E1571.7756 F50 X70.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1571.912 F50 X69.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1571.9869 F50 X68 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1572.037 F50 X68 Y53 Z0.5 ; extrude filament
G1 E1572.1733 F50 X67 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1572.3097 F50 X68 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1572.3846 F50 X69.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1572.4347 F50 X69.5 Y54.5 Z0.5 ; extrude filament
G1 E1572.571 F50 X70.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1572.7074 F50 X69.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1572.7823 F50 X68 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1572.8324 F50 X68 Y56 Z0.5 ; extrude filament
G1 E1572.9688 F50 X67 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1573.1051 F50 X68 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1573.18 F50 X69.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1573.2301 F50 X69.5 Y57.5 Z0.5 ; extrude filament
G1 E1573.3665 F50 X70.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1573.5028 F50 X69.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1573.5778 F50 X68 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1573.6278 F50 X68 Y59 Z0.5 ; extrude filament
G1 E1573.7642 F50 X67 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1573.9005 F50 X68 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1573.9755 F50 X69.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1574.0255 F50 X69.5 Y60.5 Z0.5 ; extrude filament
G1 E1574.1619 F50 X70.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1574.2982 F50 X69.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1574.3732 F50 X68 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1574.4232 F50 X68 Y62 Z0.5 ; extrude filament
G1 E1574.5596 F50 X67 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1574.6959 F50 X68 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1574.7709 F50 X69.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1574.8209 F50 X69.5 Y63.5 Z0.5 ; extrude filament
G1 E1574.9573 F50 X70.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1575.0936 F50 X69.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1575.1686 F50 X68 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1575.2186 F50 X68 Y65 Z0.5 ; extrude filament
G1 E1575.355 F50 X67 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1575.4913 F50 X68 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1575.5663 F50 X69.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1575.6163 F50 X69.5 Y66.5 Z0.5 ; extrude filament
G1 E1575.7527 F50 X70.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1575.889 F50 X69.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1575.964 F50 X68 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1576.014 F50 X68 Y68 Z0.5 ; extrude filament
G1 E1576.1504 F50 X67 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1576.2867 F50 X68 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1576.3617 F50 X69.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1576.4117 F50 X69.5 Y69.5 Z0.5 ; extrude filament
G1 E1576.5481 F50 X70.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1576.6844 F50 X69.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1576.7594 F50 X68 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1576.8094 F50 X68 Y71 Z0.5 ; extrude filament
G1 E1576.9458 F50 X67 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1577.0822 F50 X68 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1577.1571 F50 X69.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1577.2072 F50 X69.5 Y72.5 Z0.5 ; extrude filament
G1 E1577.3435 F50 X70.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1577.4799 F50 X69.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1577.5548 F50 X68 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1577.6049 F50 X68 Y74 Z0.5 ; extrude filament
G1 E1577.7412 F50 X67 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1577.8776 F50 X68 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1577.9525 F50 X69.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1578.0026 F50 X69.5 Y75.5 Z0.5 ; extrude filament
G1 E1578.1389 F50 X70.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1578.2753 F50 X69.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1578.3502 F50 X68 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1578.4003 F50 X68 Y77 Z0.5 ; extrude filament
G1 E1578.5366 F50 X67 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1578.673 F50 X68 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1578.7479 F50 X69.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1578.798 F50 X69.5 Y78.5 Z0.5 ; extrude filament
G1 E1578.9343 F50 X70.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1579.0707 F50 X69.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1579.1456 F50 X68 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1579.1957 F50 X68 Y80 Z0.5 ; extrude filament
G1 E1579.332 F50 X67 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1579.4684 F50 X68 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1579.5433 F50 X69.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1579.5934 F50 X69.5 Y81.5 Z0.5 ; extrude filament
G1 E1579.7297 F50 X70.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1579.8661 F50 X69.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1579.941 F50 X68 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1579.9911 F50 X68 Y83 Z0.5 ; extrude filament
G1 E1580.1274 F50 X67 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1580.2638 F50 X68 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1580.3387 F50 X69.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1580.3888 F50 X69.5 Y84.5 Z0.5 ; extrude filament
G1 E1580.5251 F50 X70.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1580.6615 F50 X69.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1580.7365 F50 X68 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1580.7865 F50 X68 Y86 Z0.5 ; extrude filament
G1 E1580.9229 F50 X67 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1581.0592 F50 X68 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1581.1342 F50 X69.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1581.1842 F50 X69.5 Y87.5 Z0.5 ; extrude filament
G1 E1581.3206 F50 X70.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1581.4569 F50 X69.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1581.5319 F50 X68 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1581.5819 F50 X68 Y89 Z0.5 ; extrude filament
G1 E1581.7183 F50 X67 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1581.8546 F50 X68 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1581.9296 F50 X69.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1581.9796 F50 X69.5 Y90.5 Z0.5 ; extrude filament
G1 E1582.116 F50 X70.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1582.2523 F50 X69.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1582.3273 F50 X68 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1582.3773 F50 X68 Y92 Z0.5 ; extrude filament
G1 E1582.5137 F50 X67 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1582.65 F50 X68 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1582.725 F50 X69.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1582.775 F50 X69.5 Y93.5 Z0.5 ; extrude filament
G1 E1582.9114 F50 X70.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1583.0477 F50 X69.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1583.1227 F50 X68 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1583.1727 F50 X68 Y95 Z0.5 ; extrude filament
G1 E1583.3091 F50 X67 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1583.4454 F50 X68 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1583.5204 F50 X69.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1583.5704 F50 X69.5 Y96.5 Z0.5 ; extrude filament
G1 E1583.7068 F50 X70.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1583.8431 F50 X69.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1583.9181 F50 X68 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1583.9681 F50 X68 Y98 Z0.5 ; extrude filament
G1 E1584.1045 F50 X67 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1584.2408 F50 X68 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1584.3158 F50 X69.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1584.3658 F50 X69.5 Y99.5 Z0.5 ; extrude filament
G1 E1584.5022 F50 X70.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1584.6385 F50 X69.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1584.7135 F50 X68 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1584.7635 F50 X68 Y101 Z0.5 ; extrude filament
G1 E1584.8999 F50 X67 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1585.0363 F50 X68 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1585.1112 F50 X69.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1585.1613 F50 X69.5 Y102.5 Z0.5 ; extrude filament
G1 E1585.2976 F50 X70.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1585.434 F50 X69.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1585.5089 F50 X68 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1585.559 F50 X68 Y104 Z0.5 ; extrude filament
G1 E1585.6953 F50 X67 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1585.8317 F50 X68 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1585.9066 F50 X69.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1585.9567 F50 X69.5 Y105.5 Z0.5 ; extrude filament
G1 E1586.093 F50 X70.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1586.2294 F50 X69.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1586.3043 F50 X68 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1586.3544 F50 X68 Y107 Z0.5 ; extrude filament
G1 E1586.4907 F50 X67 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1586.6271 F50 X68 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1586.702 F50 X69.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1586.7521 F50 X69.5 Y108.5 Z0.5 ; extrude filament
G1 E1586.8884 F50 X70.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1587.0248 F50 X69.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1587.0997 F50 X68 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1587.1498 F50 X68 Y110 Z0.5 ; extrude filament
G1 E1587.2861 F50 X67 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1587.4225 F50 X68 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1587.4974 F50 X69.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1587.5475 F50 X69.5 Y111.5 Z0.5 ; extrude filament
G1 E1587.6838 F50 X70.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1587.8202 F50 X69.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1587.8951 F50 X68 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1587.9452 F50 X68 Y113 Z0.5 ; extrude filament
G1 E1588.0815 F50 X67 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1588.2179 F50 X68 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1588.2928 F50 X69.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1588.3429 F50 X69.5 Y114.5 Z0.5 ; extrude filament
G1 E1588.4792 F50 X70.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1588.6156 F50 X69.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1588.6906 F50 X68 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1588.7406 F50 X68 Y116 Z0.5 ; extrude filament
G1 E1588.877 F50 X67 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1589.0133 F50 X68 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1589.0883 F50 X69.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1589.1383 F50 X69.5 Y117.5 Z0.5 ; extrude filament
G1 E1589.2747 F50 X70.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1589.411 F50 X69.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1589.486 F50 X68 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1589.536 F50 X68 Y119 Z0.5 ; extrude filament
G1 E1586.536 F5000 X68 Y119 Z0.5 ; retract filament
G0 E1586.536 F5000 X68 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1586.536 F5000 X74 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1586.536 F5000 X74 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1589.536 F50 X74 Y20 Z0.5 ; extrude filament
G1 E1589.6724 F50 X73 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1589.8087 F50 X74 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1589.8837 F50 X75.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1589.9337 F50 X75.5 Y21.5 Z0.5 ; extrude filament
G1 E1590.0701 F50 X76.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1590.2064 F50 X75.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1590.2814 F50 X74 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1590.3314 F50 X74 Y23 Z0.5 ; extrude filament
G1 E1590.4678 F50 X73 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1590.6041 F50 X74 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1590.6791 F50 X75.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1590.7291 F50 X75.5 Y24.5 Z0.5 ; extrude filament
G1 E1590.8655 F50 X76.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1591.0018 F50 X75.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1591.0768 F50 X74 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1591.1268 F50 X74 Y26 Z0.5 ; extrude filament
G1 E1591.2632 F50 X73 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1591.3995 F50 X74 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1591.4745 F50 X75.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1591.5245 F50 X75.5 Y27.5 Z0.5 ; extrude filament
G1 E1591.6609 F50 X76.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1591.7972 F50 X75.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1591.8722 F50 X74 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1591.9222 F50 X74 Y29 Z0.5 ; extrude filament
G1 E1592.0586 F50 X73 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1592.195 F50 X74 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1592.2699 F50 X75.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1592.32 F50 X75.5 Y30.5 Z0.5 ; extrude filament
G1 E1592.4563 F50 X76.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1592.5927 F50 X75.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1592.6676 F50 X74 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1592.7177 F50 X74 Y32 Z0.5 ; extrude filament
G1 E1592.854 F50 X73 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1592.9904 F50 X74 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1593.0653 F50 X75.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1593.1154 F50 X75.5 Y33.5 Z0.5 ; extrude filament
G1 E1593.2517 F50 X76.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1593.3881 F50 X75.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1593.463 F50 X74 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1593.5131 F50 X74 Y35 Z0.5 ; extrude filament
G1 E1593.6494 F50 X73 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1593.7858 F50 X74 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1593.8607 F50 X75.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1593.9108 F50 X75.5 Y36.5 Z0.5 ; extrude filament
G1 E1594.0471 F50 X76.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1594.1835 F50 X75.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1594.2584 F50 X74 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1594.3085 F50 X74 Y38 Z0.5 ; extrude filament
G1 E1594.4448 F50 X73 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1594.5812 F50 X74 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1594.6561 F50 X75.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1594.7062 F50 X75.5 Y39.5 Z0.5 ; extrude filament
G1 E1594.8425 F50 X76.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1594.9789 F50 X75.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1595.0538 F50 X74 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1595.1039 F50 X74 Y41 Z0.5 ; extrude filament
G1 E1595.2402 F50 X73 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1595.3766 F50 X74 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1595.4515 F50 X75.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1595.5016 F50 X75.5 Y42.5 Z0.5 ; extrude filament
G1 E1595.638 F50 X76.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1595.7743 F50 X75.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1595.8492 F50 X74 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1595.8993 F50 X74 Y44 Z0.5 ; extrude filament
G1 E1596.0356 F50 X73 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1596.172 F50 X74 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1596.247 F50 X75.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1596.297 F50 X75.5 Y45.5 Z0.5 ; extrude filament
G1 E1596.4333 F50 X76.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1596.5697 F50 X75.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1596.6447 F50 X74 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1596.6947 F50 X74 Y47 Z0.5 ; extrude filament
G1 E1596.831 F50 X73 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1596.9674 F50 X74 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1597.0424 F50 X75.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1597.0924 F50 X75.5 Y48.5 Z0.5 ; extrude filament
G1 E1597.2288 F50 X76.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1597.3651 F50 X75.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1597.4401 F50 X74 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1597.4901 F50 X74 Y50 Z0.5 ; extrude filament
G1 E1597.6265 F50 X73 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1597.7628 F50 X74 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1597.8378 F50 X75.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1597.8878 F50 X75.5 Y51.5 Z0.5 ; extrude filament
G1 E1598.0242 F50 X76.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1598.1605 F50 X75.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1598.2355 F50 X74 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1598.2855 F50 X74 Y53 Z0.5 ; extrude filament
G1 E1598.4219 F50 X73 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1598.5582 F50 X74 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1598.6332 F50 X75.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1598.6832 F50 X75.5 Y54.5 Z0.5 ; extrude filament
G1 E1598.8196 F50 X76.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1598.9559 F50 X75.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1599.0309 F50 X74 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1599.0809 F50 X74 Y56 Z0.5 ; extrude filament
G1 E1599.2173 F50 X73 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1599.3536 F50 X74 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1599.4286 F50 X75.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1599.4786 F50 X75.5 Y57.5 Z0.5 ; extrude filament
G1 E1599.615 F50 X76.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1599.7513 F50 X75.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1599.8263 F50 X74 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1599.8763 F50 X74 Y59 Z0.5 ; extrude filament
G1 E1600.0127 F50 X73 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1600.149 F50 X74 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1600.224 F50 X75.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1600.274 F50 X75.5 Y60.5 Z0.5 ; extrude filament
G1 E1600.4104 F50 X76.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1600.5468 F50 X75.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1600.6217 F50 X74 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1600.6718 F50 X74 Y62 Z0.5 ; extrude filament
G1 E1600.8081 F50 X73 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1600.9445 F50 X74 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1601.0194 F50 X75.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1601.0695 F50 X75.5 Y63.5 Z0.5 ; extrude filament
G1 E1601.2058 F50 X76.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1601.3422 F50 X75.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1601.4171 F50 X74 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1601.4672 F50 X74 Y65 Z0.5 ; extrude filament
G1 E1601.6035 F50 X73 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1601.7399 F50 X74 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1601.8148 F50 X75.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1601.8649 F50 X75.5 Y66.5 Z0.5 ; extrude filament
G1 E1602.0012 F50 X76.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1602.1376 F50 X75.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1602.2125 F50 X74 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1602.2626 F50 X74 Y68 Z0.5 ; extrude filament
G1 E1602.3989 F50 X73 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1602.5353 F50 X74 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1602.6102 F50 X75.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1602.6603 F50 X75.5 Y69.5 Z0.5 ; extrude filament
G1 E1602.7966 F50 X76.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1602.933 F50 X75.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1603.0079 F50 X74 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1603.058 F50 X74 Y71 Z0.5 ; extrude filament
G1 E1603.1943 F50 X73 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1603.3307 F50 X74 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1603.4056 F50 X75.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1603.4557 F50 X75.5 Y72.5 Z0.5 ; extrude filament
G1 E1603.592 F50 X76.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1603.7284 F50 X75.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1603.8033 F50 X74 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1603.8534 F50 X74 Y74 Z0.5 ; extrude filament
G1 E1603.9897 F50 X73 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1604.1261 F50 X74 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1604.201 F50 X75.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1604.2511 F50 X75.5 Y75.5 Z0.5 ; extrude filament
G1 E1604.3875 F50 X76.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1604.5238 F50 X75.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1604.5988 F50 X74 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1604.6488 F50 X74 Y77 Z0.5 ; extrude filament
G1 E1604.7852 F50 X73 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1604.9215 F50 X74 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1604.9965 F50 X75.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1605.0465 F50 X75.5 Y78.5 Z0.5 ; extrude filament
G1 E1605.1829 F50 X76.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1605.3192 F50 X75.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1605.3942 F50 X74 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1605.4442 F50 X74 Y80 Z0.5 ; extrude filament
G1 E1605.5806 F50 X73 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1605.7169 F50 X74 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1605.7919 F50 X75.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1605.8419 F50 X75.5 Y81.5 Z0.5 ; extrude filament
G1 E1605.9783 F50 X76.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1606.1146 F50 X75.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1606.1896 F50 X74 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1606.2396 F50 X74 Y83 Z0.5 ; extrude filament
G1 E1606.376 F50 X73 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1606.5123 F50 X74 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1606.5873 F50 X75.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1606.6373 F50 X75.5 Y84.5 Z0.5 ; extrude filament
G1 E1606.7737 F50 X76.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1606.91 F50 X75.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1606.985 F50 X74 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1607.035 F50 X74 Y86 Z0.5 ; extrude filament
G1 E1607.1714 F50 X73 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1607.3077 F50 X74 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1607.3827 F50 X75.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1607.4327 F50 X75.5 Y87.5 Z0.5 ; extrude filament
G1 E1607.5691 F50 X76.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1607.7054 F50 X75.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1607.7804 F50 X74 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1607.8304 F50 X74 Y89 Z0.5 ; extrude filament
G1 E1607.9668 F50 X73 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1608.1031 F50 X74 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1608.1781 F50 X75.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1608.2281 F50 X75.5 Y90.5 Z0.5 ; extrude filament
G1 E1608.3645 F50 X76.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1608.5009 F50 X75.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1608.5758 F50 X74 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1608.6259 F50 X74 Y92 Z0.5 ; extrude filament
G1 E1608.7622 F50 X73 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1608.8986 F50 X74 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1608.9735 F50 X75.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1609.0236 F50 X75.5 Y93.5 Z0.5 ; extrude filament
G1 E1609.1599 F50 X76.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1609.2963 F50 X75.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1609.3712 F50 X74 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1609.4213 F50 X74 Y95 Z0.5 ; extrude filament
G1 E1609.5576 F50 X73 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1609.694 F50 X74 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1609.7689 F50 X75.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1609.819 F50 X75.5 Y96.5 Z0.5 ; extrude filament
G1 E1609.9553 F50 X76.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1610.0917 F50 X75.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1610.1666 F50 X74 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1610.2167 F50 X74 Y98 Z0.5 ; extrude filament
G1 E1610.353 F50 X73 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1610.4894 F50 X74 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1610.5643 F50 X75.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1610.6144 F50 X75.5 Y99.5 Z0.5 ; extrude filament
G1 E1610.7507 F50 X76.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1610.8871 F50 X75.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1610.962 F50 X74 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1611.0121 F50 X74 Y101 Z0.5 ; extrude filament
G1 E1611.1484 F50 X73 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1611.2848 F50 X74 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1611.3597 F50 X75.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1611.4098 F50 X75.5 Y102.5 Z0.5 ; extrude filament
G1 E1611.5461 F50 X76.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1611.6825 F50 X75.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1611.7574 F50 X74 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1611.8075 F50 X74 Y104 Z0.5 ; extrude filament
G1 E1611.9438 F50 X73 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1612.0802 F50 X74 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1612.1552 F50 X75.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1612.2052 F50 X75.5 Y105.5 Z0.5 ; extrude filament
G1 E1612.3416 F50 X76.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1612.4779 F50 X75.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1612.5529 F50 X74 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1612.6029 F50 X74 Y107 Z0.5 ; extrude filament
G1 E1612.7393 F50 X73 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1612.8756 F50 X74 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1612.9506 F50 X75.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1613.0006 F50 X75.5 Y108.5 Z0.5 ; extrude filament
G1 E1613.137 F50 X76.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1613.2733 F50 X75.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1613.3483 F50 X74 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1613.3983 F50 X74 Y110 Z0.5 ; extrude filament
G1 E1613.5347 F50 X73 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1613.671 F50 X74 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1613.746 F50 X75.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1613.796 F50 X75.5 Y111.5 Z0.5 ; extrude filament
G1 E1613.9324 F50 X76.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1614.0687 F50 X75.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1614.1437 F50 X74 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1614.1937 F50 X74 Y113 Z0.5 ; extrude filament
G1 E1614.3301 F50 X73 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1614.4664 F50 X74 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1614.5414 F50 X75.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1614.5914 F50 X75.5 Y114.5 Z0.5 ; extrude filament
G1 E1614.7278 F50 X76.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1614.8641 F50 X75.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1614.9391 F50 X74 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1614.9891 F50 X74 Y116 Z0.5 ; extrude filament
G1 E1615.1255 F50 X73 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1615.2618 F50 X74 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1615.3368 F50 X75.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1615.3868 F50 X75.5 Y117.5 Z0.5 ; extrude filament
G1 E1615.5232 F50 X76.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1615.6595 F50 X75.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1615.7345 F50 X74 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1615.7845 F50 X74 Y119 Z0.5 ; extrude filament
G1 E1612.7845 F5000 X74 Y119 Z0.5 ; retract filament
G0 E1612.7845 F5000 X74 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1612.7845 F5000 X80 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1612.7845 F5000 X80 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1615.7845 F50 X80 Y20 Z0.5 ; extrude filament
G1 E1615.9209 F50 X79 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1616.0573 F50 X80 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1616.1322 F50 X81.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1616.1823 F50 X81.5 Y21.5 Z0.5 ; extrude filament
G1 E1616.3186 F50 X82.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1616.455 F50 X81.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1616.5299 F50 X80 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1616.58 F50 X80 Y23 Z0.5 ; extrude filament
G1 E1616.7163 F50 X79 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1616.8527 F50 X80 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1616.9276 F50 X81.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1616.9777 F50 X81.5 Y24.5 Z0.5 ; extrude filament
G1 E1617.114 F50 X82.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1617.2504 F50 X81.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1617.3253 F50 X80 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1617.3754 F50 X80 Y26 Z0.5 ; extrude filament
G1 E1617.5117 F50 X79 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1617.6481 F50 X80 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1617.723 F50 X81.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1617.7731 F50 X81.5 Y27.5 Z0.5 ; extrude filament
G1 E1617.9094 F50 X82.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1618.0458 F50 X81.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1618.1207 F50 X80 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1618.1708 F50 X80 Y29 Z0.5 ; extrude filament
G1 E1618.3071 F50 X79 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1618.4435 F50 X80 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1618.5184 F50 X81.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1618.5685 F50 X81.5 Y30.5 Z0.5 ; extrude filament
G1 E1618.7048 F50 X82.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1618.8412 F50 X81.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1618.9161 F50 X80 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1618.9662 F50 X80 Y32 Z0.5 ; extrude filament
G1 E1619.1025 F50 X79 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1619.2389 F50 X80 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1619.3138 F50 X81.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1619.3639 F50 X81.5 Y33.5 Z0.5 ; extrude filament
G1 E1619.5002 F50 X82.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1619.6366 F50 X81.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1619.7115 F50 X80 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1619.7616 F50 X80 Y35 Z0.5 ; extrude filament
G1 E1619.898 F50 X79 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1620.0343 F50 X80 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1620.1093 F50 X81.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1620.1593 F50 X81.5 Y36.5 Z0.5 ; extrude filament
G1 E1620.2957 F50 X82.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1620.432 F50 X81.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1620.507 F50 X80 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1620.557 F50 X80 Y38 Z0.5 ; extrude filament
G1 E1620.6934 F50 X79 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1620.8297 F50 X80 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1620.9047 F50 X81.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1620.9547 F50 X81.5 Y39.5 Z0.5 ; extrude filament
G1 E1621.0911 F50 X82.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1621.2274 F50 X81.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1621.3024 F50 X80 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1621.3524 F50 X80 Y41 Z0.5 ; extrude filament
G1 E1621.4888 F50 X79 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1621.6251 F50 X80 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1621.7001 F50 X81.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1621.7501 F50 X81.5 Y42.5 Z0.5 ; extrude filament
G1 E1621.8865 F50 X82.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1622.0228 F50 X81.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1622.0978 F50 X80 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1622.1478 F50 X80 Y44 Z0.5 ; extrude filament
G1 E1622.2842 F50 X79 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1622.4205 F50 X80 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1622.4955 F50 X81.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1622.5455 F50 X81.5 Y45.5 Z0.5 ; extrude filament
G1 E1622.6819 F50 X82.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1622.8182 F50 X81.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1622.8932 F50 X80 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1622.9432 F50 X80 Y47 Z0.5 ; extrude filament
G1 E1623.0796 F50 X79 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1623.216 F50 X80 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1623.2909 F50 X81.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1623.341 F50 X81.5 Y48.5 Z0.5 ; extrude filament
G1 E1623.4773 F50 X82.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1623.6136 F50 X81.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1623.6886 F50 X80 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1623.7386 F50 X80 Y50 Z0.5 ; extrude filament
G1 E1623.875 F50 X79 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1624.0114 F50 X80 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1624.0863 F50 X81.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1624.1364 F50 X81.5 Y51.5 Z0.5 ; extrude filament
G1 E1624.2727 F50 X82.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1624.409 F50 X81.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1624.484 F50 X80 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1624.534 F50 X80 Y53 Z0.5 ; extrude filament
G1 E1624.6704 F50 X79 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1624.8068 F50 X80 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1624.8817 F50 X81.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1624.9318 F50 X81.5 Y54.5 Z0.5 ; extrude filament
G1 E1625.0681 F50 X82.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1625.2045 F50 X81.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1625.2794 F50 X80 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1625.3295 F50 X80 Y56 Z0.5 ; extrude filament
G1 E1625.4658 F50 X79 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1625.6022 F50 X80 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1625.6771 F50 X81.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1625.7272 F50 X81.5 Y57.5 Z0.5 ; extrude filament
G1 E1625.8635 F50 X82.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1625.9999 F50 X81.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1626.0748 F50 X80 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1626.1249 F50 X80 Y59 Z0.5 ; extrude filament
G1 E1626.2612 F50 X79 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1626.3976 F50 X80 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1626.4725 F50 X81.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1626.5226 F50 X81.5 Y60.5 Z0.5 ; extrude filament
G1 E1626.6589 F50 X82.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1626.7953 F50 X81.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1626.8702 F50 X80 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1626.9203 F50 X80 Y62 Z0.5 ; extrude filament
G1 E1627.0566 F50 X79 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1627.193 F50 X80 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1627.268 F50 X81.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1627.318 F50 X81.5 Y63.5 Z0.5 ; extrude filament
G1 E1627.4543 F50 X82.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1627.5907 F50 X81.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1627.6656 F50 X80 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1627.7157 F50 X80 Y65 Z0.5 ; extrude filament
G1 E1627.852 F50 X79 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1627.9884 F50 X80 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1628.0634 F50 X81.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1628.1134 F50 X81.5 Y66.5 Z0.5 ; extrude filament
G1 E1628.2498 F50 X82.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1628.3861 F50 X81.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1628.461 F50 X80 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1628.5111 F50 X80 Y68 Z0.5 ; extrude filament
G1 E1628.6475 F50 X79 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1628.7838 F50 X80 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1628.8588 F50 X81.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1628.9088 F50 X81.5 Y69.5 Z0.5 ; extrude filament
G1 E1629.0452 F50 X82.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1629.1815 F50 X81.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1629.2565 F50 X80 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1629.3065 F50 X80 Y71 Z0.5 ; extrude filament
G1 E1629.4429 F50 X79 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1629.5792 F50 X80 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1629.6542 F50 X81.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1629.7042 F50 X81.5 Y72.5 Z0.5 ; extrude filament
G1 E1629.8406 F50 X82.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1629.9769 F50 X81.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1630.0519 F50 X80 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1630.1019 F50 X80 Y74 Z0.5 ; extrude filament
G1 E1630.2383 F50 X79 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1630.3746 F50 X80 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1630.4496 F50 X81.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1630.4996 F50 X81.5 Y75.5 Z0.5 ; extrude filament
G1 E1630.636 F50 X82.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1630.7723 F50 X81.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1630.8473 F50 X80 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1630.8973 F50 X80 Y77 Z0.5 ; extrude filament
G1 E1631.0337 F50 X79 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1631.17 F50 X80 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1631.245 F50 X81.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1631.295 F50 X81.5 Y78.5 Z0.5 ; extrude filament
G1 E1631.4314 F50 X82.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1631.5677 F50 X81.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1631.6427 F50 X80 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1631.6927 F50 X80 Y80 Z0.5 ; extrude filament
G1 E1631.8291 F50 X79 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1631.9655 F50 X80 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1632.0404 F50 X81.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1632.0905 F50 X81.5 Y81.5 Z0.5 ; extrude filament
G1 E1632.2268 F50 X82.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1632.3632 F50 X81.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1632.4381 F50 X80 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1632.4882 F50 X80 Y83 Z0.5 ; extrude filament
G1 E1632.6245 F50 X79 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1632.7609 F50 X80 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1632.8358 F50 X81.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1632.8859 F50 X81.5 Y84.5 Z0.5 ; extrude filament
G1 E1633.0222 F50 X82.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1633.1586 F50 X81.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1633.2335 F50 X80 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1633.2836 F50 X80 Y86 Z0.5 ; extrude filament
G1 E1633.4199 F50 X79 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1633.5563 F50 X80 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1633.6312 F50 X81.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1633.6813 F50 X81.5 Y87.5 Z0.5 ; extrude filament
G1 E1633.8176 F50 X82.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1633.954 F50 X81.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1634.0289 F50 X80 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1634.079 F50 X80 Y89 Z0.5 ; extrude filament
G1 E1634.2153 F50 X79 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1634.3517 F50 X80 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1634.4266 F50 X81.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1634.4767 F50 X81.5 Y90.5 Z0.5 ; extrude filament
G1 E1634.613 F50 X82.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1634.7494 F50 X81.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1634.8243 F50 X80 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1634.8744 F50 X80 Y92 Z0.5 ; extrude filament
G1 E1635.0107 F50 X79 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1635.1471 F50 X80 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1635.222 F50 X81.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1635.2721 F50 X81.5 Y93.5 Z0.5 ; extrude filament
G1 E1635.4084 F50 X82.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1635.5448 F50 X81.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1635.6198 F50 X80 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1635.6698 F50 X80 Y95 Z0.5 ; extrude filament
G1 E1635.8062 F50 X79 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1635.9425 F50 X80 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1636.0175 F50 X81.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1636.0675 F50 X81.5 Y96.5 Z0.5 ; extrude filament
G1 E1636.2039 F50 X82.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1636.3402 F50 X81.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1636.4152 F50 X80 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1636.4652 F50 X80 Y98 Z0.5 ; extrude filament
G1 E1636.6016 F50 X79 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1636.7379 F50 X80 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1636.8129 F50 X81.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1636.8629 F50 X81.5 Y99.5 Z0.5 ; extrude filament
G1 E1636.9993 F50 X82.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1637.1356 F50 X81.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1637.2106 F50 X80 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1637.2606 F50 X80 Y101 Z0.5 ; extrude filament
G1 E1637.397 F50 X79 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1637.5333 F50 X80 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1637.6083 F50 X81.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1637.6583 F50 X81.5 Y102.5 Z0.5 ; extrude filament
G1 E1637.7947 F50 X82.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1637.931 F50 X81.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1638.006 F50 X80 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1638.056 F50 X80 Y104 Z0.5 ; extrude filament
G1 E1638.1924 F50 X79 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1638.3287 F50 X80 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1638.4037 F50 X81.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1638.4537 F50 X81.5 Y105.5 Z0.5 ; extrude filament
G1 E1638.5901 F50 X82.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1638.7264 F50 X81.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1638.8014 F50 X80 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1638.8514 F50 X80 Y107 Z0.5 ; extrude filament
G1 E1638.9878 F50 X79 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1639.1241 F50 X80 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1639.1991 F50 X81.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1639.2491 F50 X81.5 Y108.5 Z0.5 ; extrude filament
G1 E1639.3855 F50 X82.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1639.5219 F50 X81.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1639.5968 F50 X80 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1639.6469 F50 X80 Y110 Z0.5 ; extrude filament
G1 E1639.7832 F50 X79 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1639.9196 F50 X80 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1639.9945 F50 X81.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1640.0446 F50 X81.5 Y111.5 Z0.5 ; extrude filament
G1 E1640.1809 F50 X82.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1640.3173 F50 X81.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1640.3922 F50 X80 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1640.4423 F50 X80 Y113 Z0.5 ; extrude filament
G1 E1640.5786 F50 X79 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1640.715 F50 X80 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1640.7899 F50 X81.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1640.84 F50 X81.5 Y114.5 Z0.5 ; extrude filament
G1 E1640.9763 F50 X82.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1641.1127 F50 X81.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1641.1876 F50 X80 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1641.2377 F50 X80 Y116 Z0.5 ; extrude filament
G1 E1641.374 F50 X79 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1641.5104 F50 X80 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1641.5853 F50 X81.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1641.6354 F50 X81.5 Y117.5 Z0.5 ; extrude filament
G1 E1641.7717 F50 X82.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1641.9081 F50 X81.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1641.983 F50 X80 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1642.0331 F50 X80 Y119 Z0.5 ; extrude filament
G1 E1639.0331 F5000 X80 Y119 Z0.5 ; retract filament
G0 E1639.0331 F5000 X80 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1639.0331 F5000 X86 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1639.0331 F5000 X86 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1642.0331 F50 X86 Y20 Z0.5 ; extrude filament
G1 E1642.1694 F50 X85 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1642.3058 F50 X86 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1642.3807 F50 X87.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1642.4308 F50 X87.5 Y21.5 Z0.5 ; extrude filament
G1 E1642.5671 F50 X88.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1642.7035 F50 X87.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1642.7784 F50 X86 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1642.8285 F50 X86 Y23 Z0.5 ; extrude filament
G1 E1642.9648 F50 X85 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1643.1012 F50 X86 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1643.1761 F50 X87.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1643.2262 F50 X87.5 Y24.5 Z0.5 ; extrude filament
G1 E1643.3625 F50 X88.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1643.4989 F50 X87.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1643.5739 F50 X86 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1643.6239 F50 X86 Y26 Z0.5 ; extrude filament
G1 E1643.7603 F50 X85 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1643.8966 F50 X86 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1643.9716 F50 X87.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1644.0216 F50 X87.5 Y27.5 Z0.5 ; extrude filament
G1 E1644.158 F50 X88.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1644.2943 F50 X87.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1644.3693 F50 X86 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1644.4193 F50 X86 Y29 Z0.5 ; extrude filament
G1 E1644.5557 F50 X85 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1644.692 F50 X86 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1644.767 F50 X87.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1644.817 F50 X87.5 Y30.5 Z0.5 ; extrude filament
G1 E1644.9534 F50 X88.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1645.0897 F50 X87.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1645.1647 F50 X86 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1645.2147 F50 X86 Y32 Z0.5 ; extrude filament
G1 E1645.3511 F50 X85 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1645.4874 F50 X86 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1645.5624 F50 X87.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1645.6124 F50 X87.5 Y33.5 Z0.5 ; extrude filament
G1 E1645.7488 F50 X88.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1645.8851 F50 X87.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1645.9601 F50 X86 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1646.0101 F50 X86 Y35 Z0.5 ; extrude filament
G1 E1646.1465 F50 X85 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1646.2828 F50 X86 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1646.3578 F50 X87.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1646.4078 F50 X87.5 Y36.5 Z0.5 ; extrude filament
G1 E1646.5442 F50 X88.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1646.6805 F50 X87.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1646.7555 F50 X86 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1646.8055 F50 X86 Y38 Z0.5 ; extrude filament
G1 E1646.9419 F50 X85 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1647.0782 F50 X86 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1647.1532 F50 X87.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1647.2032 F50 X87.5 Y39.5 Z0.5 ; extrude filament
G1 E1647.3396 F50 X88.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1647.476 F50 X87.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1647.5509 F50 X86 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1647.601 F50 X86 Y41 Z0.5 ; extrude filament
G1 E1647.7373 F50 X85 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1647.8737 F50 X86 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1647.9486 F50 X87.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1647.9987 F50 X87.5 Y42.5 Z0.5 ; extrude filament
G1 E1648.135 F50 X88.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1648.2714 F50 X87.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1648.3463 F50 X86 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1648.3964 F50 X86 Y44 Z0.5 ; extrude filament
G1 E1648.5327 F50 X85 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1648.6691 F50 X86 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1648.744 F50 X87.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1648.7941 F50 X87.5 Y45.5 Z0.5 ; extrude filament
G1 E1648.9304 F50 X88.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1649.0668 F50 X87.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1649.1417 F50 X86 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1649.1918 F50 X86 Y47 Z0.5 ; extrude filament
G1 E1649.3281 F50 X85 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1649.4645 F50 X86 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1649.5394 F50 X87.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1649.5895 F50 X87.5 Y48.5 Z0.5 ; extrude filament
G1 E1649.7258 F50 X88.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1649.8622 F50 X87.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1649.9371 F50 X86 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1649.9872 F50 X86 Y50 Z0.5 ; extrude filament
G1 E1650.1235 F50 X85 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1650.2599 F50 X86 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1650.3348 F50 X87.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1650.3849 F50 X87.5 Y51.5 Z0.5 ; extrude filament
G1 E1650.5212 F50 X88.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1650.6576 F50 X87.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1650.7325 F50 X86 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1650.7826 F50 X86 Y53 Z0.5 ; extrude filament
G1 E1650.919 F50 X85 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1651.0553 F50 X86 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1651.1302 F50 X87.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1651.1803 F50 X87.5 Y54.5 Z0.5 ; extrude filament
G1 E1651.3167 F50 X88.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1651.453 F50 X87.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1651.528 F50 X86 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1651.578 F50 X86 Y56 Z0.5 ; extrude filament
G1 E1651.7144 F50 X85 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1651.8507 F50 X86 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1651.9257 F50 X87.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1651.9757 F50 X87.5 Y57.5 Z0.5 ; extrude filament
G1 E1652.112 F50 X88.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1652.2484 F50 X87.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1652.3234 F50 X86 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1652.3734 F50 X86 Y59 Z0.5 ; extrude filament
G1 E1652.5098 F50 X85 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1652.6461 F50 X86 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1652.7211 F50 X87.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1652.7711 F50 X87.5 Y60.5 Z0.5 ; extrude filament
G1 E1652.9075 F50 X88.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1653.0438 F50 X87.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1653.1188 F50 X86 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1653.1688 F50 X86 Y62 Z0.5 ; extrude filament
G1 E1653.3052 F50 X85 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1653.4415 F50 X86 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1653.5165 F50 X87.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1653.5665 F50 X87.5 Y63.5 Z0.5 ; extrude filament
G1 E1653.7029 F50 X88.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1653.8392 F50 X87.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1653.9142 F50 X86 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1653.9642 F50 X86 Y65 Z0.5 ; extrude filament
G1 E1654.1006 F50 X85 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1654.2369 F50 X86 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1654.3119 F50 X87.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1654.3619 F50 X87.5 Y66.5 Z0.5 ; extrude filament
G1 E1654.4983 F50 X88.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1654.6346 F50 X87.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1654.7096 F50 X86 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1654.7596 F50 X86 Y68 Z0.5 ; extrude filament
G1 E1654.896 F50 X85 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1655.0323 F50 X86 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1655.1073 F50 X87.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1655.1573 F50 X87.5 Y69.5 Z0.5 ; extrude filament
G1 E1655.2937 F50 X88.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1655.43 F50 X87.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1655.505 F50 X86 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1655.555 F50 X86 Y71 Z0.5 ; extrude filament
G1 E1655.6914 F50 X85 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1655.8278 F50 X86 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1655.9027 F50 X87.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1655.9528 F50 X87.5 Y72.5 Z0.5 ; extrude filament
G1 E1656.0891 F50 X88.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1656.2255 F50 X87.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1656.3004 F50 X86 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1656.3505 F50 X86 Y74 Z0.5 ; extrude filament
G1 E1656.4868 F50 X85 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1656.6232 F50 X86 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1656.6981 F50 X87.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1656.7482 F50 X87.5 Y75.5 Z0.5 ; extrude filament
G1 E1656.8845 F50 X88.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1657.0209 F50 X87.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1657.0958 F50 X86 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1657.1459 F50 X86 Y77 Z0.5 ; extrude filament
G1 E1657.2822 F50 X85 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1657.4186 F50 X86 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1657.4935 F50 X87.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1657.5436 F50 X87.5 Y78.5 Z0.5 ; extrude filament
G1 E1657.6799 F50 X88.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1657.8163 F50 X87.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1657.8912 F50 X86 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1657.9413 F50 X86 Y80 Z0.5 ; extrude filament
G1 E1658.0776 F50 X85 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1658.214 F50 X86 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1658.289 F50 X87.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1658.339 F50 X87.5 Y81.5 Z0.5 ; extrude filament
G1 E1658.4753 F50 X88.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1658.6117 F50 X87.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1658.6866 F50 X86 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1658.7367 F50 X86 Y83 Z0.5 ; extrude filament
G1 E1658.873 F50 X85 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1659.0094 F50 X86 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1659.0844 F50 X87.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1659.1344 F50 X87.5 Y84.5 Z0.5 ; extrude filament
G1 E1659.2708 F50 X88.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1659.4071 F50 X87.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1659.482 F50 X86 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1659.5321 F50 X86 Y86 Z0.5 ; extrude filament
G1 E1659.6685 F50 X85 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1659.8048 F50 X86 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1659.8798 F50 X87.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1659.9298 F50 X87.5 Y87.5 Z0.5 ; extrude filament
G1 E1660.0662 F50 X88.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1660.2025 F50 X87.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1660.2775 F50 X86 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1660.3275 F50 X86 Y89 Z0.5 ; extrude filament
G1 E1660.4639 F50 X85 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1660.6002 F50 X86 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1660.6752 F50 X87.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1660.7252 F50 X87.5 Y90.5 Z0.5 ; extrude filament
G1 E1660.8616 F50 X88.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1660.9979 F50 X87.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1661.0729 F50 X86 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1661.1229 F50 X86 Y92 Z0.5 ; extrude filament
G1 E1661.2593 F50 X85 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1661.3956 F50 X86 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1661.4706 F50 X87.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1661.5206 F50 X87.5 Y93.5 Z0.5 ; extrude filament
G1 E1661.657 F50 X88.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1661.7933 F50 X87.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1661.8683 F50 X86 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1661.9183 F50 X86 Y95 Z0.5 ; extrude filament
G1 E1662.0547 F50 X85 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1662.191 F50 X86 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1662.266 F50 X87.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1662.316 F50 X87.5 Y96.5 Z0.5 ; extrude filament
G1 E1662.4524 F50 X88.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1662.5887 F50 X87.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1662.6637 F50 X86 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1662.7137 F50 X86 Y98 Z0.5 ; extrude filament
G1 E1662.8501 F50 X85 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1662.9865 F50 X86 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1663.0614 F50 X87.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1663.1115 F50 X87.5 Y99.5 Z0.5 ; extrude filament
G1 E1663.2478 F50 X88.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1663.3842 F50 X87.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1663.4591 F50 X86 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1663.5092 F50 X86 Y101 Z0.5 ; extrude filament
G1 E1663.6455 F50 X85 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1663.7819 F50 X86 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1663.8568 F50 X87.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1663.9069 F50 X87.5 Y102.5 Z0.5 ; extrude filament
G1 E1664.0432 F50 X88.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1664.1796 F50 X87.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1664.2545 F50 X86 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1664.3046 F50 X86 Y104 Z0.5 ; extrude filament
G1 E1664.4409 F50 X85 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1664.5773 F50 X86 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1664.6522 F50 X87.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1664.7023 F50 X87.5 Y105.5 Z0.5 ; extrude filament
G1 E1664.8386 F50 X88.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1664.975 F50 X87.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1665.0499 F50 X86 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1665.1 F50 X86 Y107 Z0.5 ; extrude filament
G1 E1665.2363 F50 X85 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1665.3727 F50 X86 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1665.4476 F50 X87.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1665.4977 F50 X87.5 Y108.5 Z0.5 ; extrude filament
G1 E1665.634 F50 X88.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1665.7704 F50 X87.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1665.8453 F50 X86 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1665.8954 F50 X86 Y110 Z0.5 ; extrude filament
G1 E1666.0317 F50 X85 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1666.1681 F50 X86 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1666.243 F50 X87.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1666.2931 F50 X87.5 Y111.5 Z0.5 ; extrude filament
G1 E1666.4294 F50 X88.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1666.5658 F50 X87.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1666.6407 F50 X86 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1666.6908 F50 X86 Y113 Z0.5 ; extrude filament
G1 E1666.8271 F50 X85 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1666.9635 F50 X86 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1667.0385 F50 X87.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1667.0885 F50 X87.5 Y114.5 Z0.5 ; extrude filament
G1 E1667.2249 F50 X88.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1667.3612 F50 X87.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1667.4362 F50 X86 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1667.4862 F50 X86 Y116 Z0.5 ; extrude filament
G1 E1667.6226 F50 X85 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1667.7589 F50 X86 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1667.8339 F50 X87.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1667.8839 F50 X87.5 Y117.5 Z0.5 ; extrude filament
G1 E1668.0203 F50 X88.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1668.1566 F50 X87.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1668.2316 F50 X86 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1668.2816 F50 X86 Y119 Z0.5 ; extrude filament
G1 E1665.2816 F5000 X86 Y119 Z0.5 ; retract filament
G0 E1665.2816 F5000 X86 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1665.2816 F5000 X92 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1665.2816 F5000 X92 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1668.2816 F50 X92 Y20 Z0.5 ; extrude filament
G1 E1668.418 F50 X91 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1668.5543 F50 X92 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1668.6293 F50 X93.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1668.6793 F50 X93.5 Y21.5 Z0.5 ; extrude filament
G1 E1668.8157 F50 X94.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1668.952 F50 X93.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1669.027 F50 X92 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1669.077 F50 X92 Y23 Z0.5 ; extrude filament
G1 E1669.2134 F50 X91 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1669.3497 F50 X92 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1669.4247 F50 X93.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1669.4747 F50 X93.5 Y24.5 Z0.5 ; extrude filament
G1 E1669.6111 F50 X94.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1669.7474 F50 X93.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1669.8224 F50 X92 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1669.8724 F50 X92 Y26 Z0.5 ; extrude filament
G1 E1670.0088 F50 X91 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1670.1451 F50 X92 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1670.2201 F50 X93.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1670.2701 F50 X93.5 Y27.5 Z0.5 ; extrude filament
G1 E1670.4065 F50 X94.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1670.5428 F50 X93.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1670.6178 F50 X92 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1670.6678 F50 X92 Y29 Z0.5 ; extrude filament
G1 E1670.8042 F50 X91 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1670.9406 F50 X92 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1671.0155 F50 X93.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1671.0656 F50 X93.5 Y30.5 Z0.5 ; extrude filament
G1 E1671.2019 F50 X94.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1671.3383 F50 X93.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1671.4132 F50 X92 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1671.4633 F50 X92 Y32 Z0.5 ; extrude filament
G1 E1671.5996 F50 X91 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1671.736 F50 X92 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1671.8109 F50 X93.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1671.861 F50 X93.5 Y33.5 Z0.5 ; extrude filament
G1 E1671.9973 F50 X94.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1672.1337 F50 X93.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1672.2086 F50 X92 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1672.2587 F50 X92 Y35 Z0.5 ; extrude filament
G1 E1672.395 F50 X91 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1672.5314 F50 X92 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1672.6063 F50 X93.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1672.6564 F50 X93.5 Y36.5 Z0.5 ; extrude filament
G1 E1672.7927 F50 X94.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1672.9291 F50 X93.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1673.004 F50 X92 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1673.0541 F50 X92 Y38 Z0.5 ; extrude filament
G1 E1673.1904 F50 X91 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1673.3268 F50 X92 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1673.4017 F50 X93.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1673.4518 F50 X93.5 Y39.5 Z0.5 ; extrude filament
G1 E1673.5881 F50 X94.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1673.7245 F50 X93.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1673.7994 F50 X92 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1673.8495 F50 X92 Y41 Z0.5 ; extrude filament
G1 E1673.9858 F50 X91 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1674.1222 F50 X92 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1674.1971 F50 X93.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1674.2472 F50 X93.5 Y42.5 Z0.5 ; extrude filament
G1 E1674.3835 F50 X94.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1674.5199 F50 X93.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1674.5948 F50 X92 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1674.6449 F50 X92 Y44 Z0.5 ; extrude filament
G1 E1674.7813 F50 X91 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1674.9176 F50 X92 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1674.9926 F50 X93.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1675.0426 F50 X93.5 Y45.5 Z0.5 ; extrude filament
G1 E1675.179 F50 X94.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1675.3153 F50 X93.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1675.3903 F50 X92 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1675.4403 F50 X92 Y47 Z0.5 ; extrude filament
G1 E1675.5767 F50 X91 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1675.713 F50 X92 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1675.788 F50 X93.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1675.838 F50 X93.5 Y48.5 Z0.5 ; extrude filament
G1 E1675.9744 F50 X94.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1676.1107 F50 X93.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1676.1857 F50 X92 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1676.2357 F50 X92 Y50 Z0.5 ; extrude filament
G1 E1676.3721 F50 X91 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1676.5084 F50 X92 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1676.5834 F50 X93.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1676.6334 F50 X93.5 Y51.5 Z0.5 ; extrude filament
G1 E1676.7698 F50 X94.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1676.9061 F50 X93.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1676.9811 F50 X92 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1677.0311 F50 X92 Y53 Z0.5 ; extrude filament
G1 E1677.1675 F50 X91 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1677.3038 F50 X92 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1677.3788 F50 X93.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1677.4288 F50 X93.5 Y54.5 Z0.5 ; extrude filament
G1 E1677.5652 F50 X94.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1677.7015 F50 X93.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1677.7765 F50 X92 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1677.8265 F50 X92 Y56 Z0.5 ; extrude filament
G1 E1677.9629 F50 X91 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1678.0992 F50 X92 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1678.1742 F50 X93.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1678.2242 F50 X93.5 Y57.5 Z0.5 ; extrude filament
G1 E1678.3606 F50 X94.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1678.497 F50 X93.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1678.5719 F50 X92 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1678.622 F50 X92 Y59 Z0.5 ; extrude filament
G1 E1678.7583 F50 X91 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1678.8947 F50 X92 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1678.9696 F50 X93.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1679.0197 F50 X93.5 Y60.5 Z0.5 ; extrude filament
G1 E1679.156 F50 X94.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1679.2924 F50 X93.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1679.3673 F50 X92 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1679.4174 F50 X92 Y62 Z0.5 ; extrude filament
G1 E1679.5537 F50 X91 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1679.6901 F50 X92 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1679.765 F50 X93.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1679.8151 F50 X93.5 Y63.5 Z0.5 ; extrude filament
G1 E1679.9514 F50 X94.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1680.0878 F50 X93.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1680.1627 F50 X92 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1680.2128 F50 X92 Y65 Z0.5 ; extrude filament
G1 E1680.3491 F50 X91 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1680.4855 F50 X92 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1680.5604 F50 X93.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1680.6105 F50 X93.5 Y66.5 Z0.5 ; extrude filament
G1 E1680.7468 F50 X94.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1680.8832 F50 X93.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1680.9581 F50 X92 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1681.0082 F50 X92 Y68 Z0.5 ; extrude filament
G1 E1681.1445 F50 X91 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1681.2809 F50 X92 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1681.3558 F50 X93.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1681.4059 F50 X93.5 Y69.5 Z0.5 ; extrude filament
G1 E1681.5422 F50 X94.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1681.6786 F50 X93.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1681.7535 F50 X92 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1681.8036 F50 X92 Y71 Z0.5 ; extrude filament
G1 E1681.94 F50 X91 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1682.0763 F50 X92 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1682.1512 F50 X93.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1682.2013 F50 X93.5 Y72.5 Z0.5 ; extrude filament
G1 E1682.3376 F50 X94.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1682.474 F50 X93.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1682.549 F50 X92 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1682.599 F50 X92 Y74 Z0.5 ; extrude filament
G1 E1682.7354 F50 X91 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1682.8717 F50 X92 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1682.9467 F50 X93.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1682.9967 F50 X93.5 Y75.5 Z0.5 ; extrude filament
G1 E1683.133 F50 X94.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1683.2694 F50 X93.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1683.3444 F50 X92 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1683.3944 F50 X92 Y77 Z0.5 ; extrude filament
G1 E1683.5308 F50 X91 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1683.6671 F50 X92 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1683.7421 F50 X93.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1683.7921 F50 X93.5 Y78.5 Z0.5 ; extrude filament
G1 E1683.9285 F50 X94.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1684.0648 F50 X93.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1684.1398 F50 X92 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1684.1898 F50 X92 Y80 Z0.5 ; extrude filament
G1 E1684.3262 F50 X91 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1684.4625 F50 X92 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1684.5375 F50 X93.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1684.5875 F50 X93.5 Y81.5 Z0.5 ; extrude filament
G1 E1684.7239 F50 X94.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1684.8602 F50 X93.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1684.9352 F50 X92 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1684.9852 F50 X92 Y83 Z0.5 ; extrude filament
G1 E1685.1216 F50 X91 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1685.2579 F50 X92 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1685.3329 F50 X93.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1685.3829 F50 X93.5 Y84.5 Z0.5 ; extrude filament
G1 E1685.5193 F50 X94.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1685.6556 F50 X93.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1685.7306 F50 X92 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1685.7806 F50 X92 Y86 Z0.5 ; extrude filament
G1 E1685.917 F50 X91 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1686.0533 F50 X92 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1686.1283 F50 X93.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1686.1783 F50 X93.5 Y87.5 Z0.5 ; extrude filament
G1 E1686.3147 F50 X94.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1686.451 F50 X93.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1686.526 F50 X92 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1686.576 F50 X92 Y89 Z0.5 ; extrude filament
G1 E1686.7124 F50 X91 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1686.8488 F50 X92 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1686.9237 F50 X93.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1686.9738 F50 X93.5 Y90.5 Z0.5 ; extrude filament
G1 E1687.1101 F50 X94.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1687.2465 F50 X93.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1687.3214 F50 X92 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1687.3715 F50 X92 Y92 Z0.5 ; extrude filament
G1 E1687.5078 F50 X91 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1687.6442 F50 X92 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1687.7191 F50 X93.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1687.7692 F50 X93.5 Y93.5 Z0.5 ; extrude filament
G1 E1687.9055 F50 X94.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1688.0419 F50 X93.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1688.1168 F50 X92 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1688.1669 F50 X92 Y95 Z0.5 ; extrude filament
G1 E1688.3032 F50 X91 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1688.4396 F50 X92 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1688.5145 F50 X93.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1688.5646 F50 X93.5 Y96.5 Z0.5 ; extrude filament
G1 E1688.7009 F50 X94.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1688.8373 F50 X93.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1688.9122 F50 X92 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1688.9623 F50 X92 Y98 Z0.5 ; extrude filament
G1 E1689.0986 F50 X91 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1689.235 F50 X92 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1689.3099 F50 X93.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1689.36 F50 X93.5 Y99.5 Z0.5 ; extrude filament
G1 E1689.4963 F50 X94.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1689.6327 F50 X93.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1689.7076 F50 X92 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1689.7577 F50 X92 Y101 Z0.5 ; extrude filament
G1 E1689.894 F50 X91 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1690.0304 F50 X92 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1690.1053 F50 X93.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1690.1554 F50 X93.5 Y102.5 Z0.5 ; extrude filament
G1 E1690.2917 F50 X94.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1690.4281 F50 X93.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1690.503 F50 X92 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1690.5531 F50 X92 Y104 Z0.5 ; extrude filament
G1 E1690.6895 F50 X91 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1690.8258 F50 X92 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1690.9008 F50 X93.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1690.9508 F50 X93.5 Y105.5 Z0.5 ; extrude filament
G1 E1691.0872 F50 X94.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1691.2235 F50 X93.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1691.2985 F50 X92 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1691.3485 F50 X92 Y107 Z0.5 ; extrude filament
G1 E1691.4849 F50 X91 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1691.6212 F50 X92 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1691.6962 F50 X93.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1691.7462 F50 X93.5 Y108.5 Z0.5 ; extrude filament
G1 E1691.8826 F50 X94.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1692.0189 F50 X93.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1692.0939 F50 X92 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1692.1439 F50 X92 Y110 Z0.5 ; extrude filament
G1 E1692.2803 F50 X91 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1692.4166 F50 X92 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1692.4916 F50 X93.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1692.5416 F50 X93.5 Y111.5 Z0.5 ; extrude filament
G1 E1692.678 F50 X94.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1692.8143 F50 X93.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1692.8893 F50 X92 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1692.9393 F50 X92 Y113 Z0.5 ; extrude filament
G1 E1693.0757 F50 X91 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1693.212 F50 X92 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1693.287 F50 X93.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1693.337 F50 X93.5 Y114.5 Z0.5 ; extrude filament
G1 E1693.4734 F50 X94.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1693.6097 F50 X93.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1693.6847 F50 X92 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1693.7347 F50 X92 Y116 Z0.5 ; extrude filament
G1 E1693.8711 F50 X91 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1694.0074 F50 X92 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1694.0824 F50 X93.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1694.1324 F50 X93.5 Y117.5 Z0.5 ; extrude filament
G1 E1694.2688 F50 X94.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1694.4052 F50 X93.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1694.4801 F50 X92 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1694.5302 F50 X92 Y119 Z0.5 ; extrude filament
G1 E1691.5302 F5000 X92 Y119 Z0.5 ; retract filament
G0 E1691.5302 F5000 X92 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1691.5302 F5000 X98 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1691.5302 F5000 X98 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1694.5302 F50 X98 Y20 Z0.5 ; extrude filament
G1 E1694.6665 F50 X97 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1694.8029 F50 X98 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1694.8778 F50 X99.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1694.9279 F50 X99.5 Y21.5 Z0.5 ; extrude filament
G1 E1695.0642 F50 X100.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1695.2006 F50 X99.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1695.2755 F50 X98 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1695.3256 F50 X98 Y23 Z0.5 ; extrude filament
G1 E1695.4619 F50 X97 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1695.5983 F50 X98 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1695.6732 F50 X99.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1695.7233 F50 X99.5 Y24.5 Z0.5 ; extrude filament
G1 E1695.8596 F50 X100.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1695.996 F50 X99.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1696.0709 F50 X98 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1696.121 F50 X98 Y26 Z0.5 ; extrude filament
G1 E1696.2573 F50 X97 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1696.3937 F50 X98 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1696.4686 F50 X99.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1696.5187 F50 X99.5 Y27.5 Z0.5 ; extrude filament
G1 E1696.655 F50 X100.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1696.7914 F50 X99.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1696.8663 F50 X98 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1696.9164 F50 X98 Y29 Z0.5 ; extrude filament
G1 E1697.0527 F50 X97 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1697.1891 F50 X98 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1697.264 F50 X99.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1697.3141 F50 X99.5 Y30.5 Z0.5 ; extrude filament
G1 E1697.4504 F50 X100.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1697.5868 F50 X99.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1697.6617 F50 X98 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1697.7118 F50 X98 Y32 Z0.5 ; extrude filament
G1 E1697.8481 F50 X97 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1697.9845 F50 X98 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1698.0594 F50 X99.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1698.1095 F50 X99.5 Y33.5 Z0.5 ; extrude filament
G1 E1698.2458 F50 X100.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1698.3822 F50 X99.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1698.4572 F50 X98 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1698.5072 F50 X98 Y35 Z0.5 ; extrude filament
G1 E1698.6436 F50 X97 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1698.7799 F50 X98 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1698.8549 F50 X99.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1698.9049 F50 X99.5 Y36.5 Z0.5 ; extrude filament
G1 E1699.0413 F50 X100.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1699.1776 F50 X99.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1699.2526 F50 X98 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1699.3026 F50 X98 Y38 Z0.5 ; extrude filament
G1 E1699.439 F50 X97 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1699.5753 F50 X98 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1699.6503 F50 X99.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1699.7003 F50 X99.5 Y39.5 Z0.5 ; extrude filament
G1 E1699.8367 F50 X100.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1699.973 F50 X99.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1700.048 F50 X98 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1700.098 F50 X98 Y41 Z0.5 ; extrude filament
G1 E1700.2344 F50 X97 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1700.3707 F50 X98 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1700.4457 F50 X99.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1700.4957 F50 X99.5 Y42.5 Z0.5 ; extrude filament
G1 E1700.6321 F50 X100.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1700.7684 F50 X99.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1700.8434 F50 X98 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1700.8934 F50 X98 Y44 Z0.5 ; extrude filament
G1 E1701.0298 F50 X97 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1701.1661 F50 X98 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1701.2411 F50 X99.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1701.2911 F50 X99.5 Y45.5 Z0.5 ; extrude filament
G1 E1701.4275 F50 X100.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1701.5638 F50 X99.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1701.6388 F50 X98 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1701.6888 F50 X98 Y47 Z0.5 ; extrude filament
G1 E1701.8252 F50 X97 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1701.9615 F50 X98 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1702.0365 F50 X99.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1702.0865 F50 X99.5 Y48.5 Z0.5 ; extrude filament
G1 E1702.2229 F50 X100.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1702.3593 F50 X99.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1702.4342 F50 X98 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1702.4843 F50 X98 Y50 Z0.5 ; extrude filament
G1 E1702.6206 F50 X97 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1702.757 F50 X98 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1702.8319 F50 X99.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1702.882 F50 X99.5 Y51.5 Z0.5 ; extrude filament
G1 E1703.0183 F50 X100.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1703.1547 F50 X99.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1703.2296 F50 X98 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1703.2797 F50 X98 Y53 Z0.5 ; extrude filament
G1 E1703.416 F50 X97 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1703.5524 F50 X98 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1703.6273 F50 X99.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1703.6774 F50 X99.5 Y54.5 Z0.5 ; extrude filament
G1 E1703.8137 F50 X100.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1703.9501 F50 X99.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1704.025 F50 X98 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1704.0751 F50 X98 Y56 Z0.5 ; extrude filament
G1 E1704.2114 F50 X97 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1704.3478 F50 X98 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1704.4227 F50 X99.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1704.4728 F50 X99.5 Y57.5 Z0.5 ; extrude filament
G1 E1704.6091 F50 X100.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1704.7455 F50 X99.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1704.8204 F50 X98 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1704.8705 F50 X98 Y59 Z0.5 ; extrude filament
G1 E1705.0068 F50 X97 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1705.1432 F50 X98 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1705.2181 F50 X99.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1705.2682 F50 X99.5 Y60.5 Z0.5 ; extrude filament
G1 E1705.4045 F50 X100.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1705.5409 F50 X99.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1705.6158 F50 X98 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1705.6659 F50 X98 Y62 Z0.5 ; extrude filament
G1 E1705.8022 F50 X97 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1705.9386 F50 X98 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1706.0135 F50 X99.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1706.0636 F50 X99.5 Y63.5 Z0.5 ; extrude filament
G1 E1706.2 F50 X100.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1706.3363 F50 X99.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1706.4113 F50 X98 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1706.4613 F50 X98 Y65 Z0.5 ; extrude filament
G1 E1706.5977 F50 X97 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1706.734 F50 X98 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1706.809 F50 X99.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1706.859 F50 X99.5 Y66.5 Z0.5 ; extrude filament
G1 E1706.9954 F50 X100.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1707.1317 F50 X99.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1707.2067 F50 X98 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1707.2567 F50 X98 Y68 Z0.5 ; extrude filament
G1 E1707.3931 F50 X97 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1707.5294 F50 X98 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1707.6044 F50 X99.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1707.6544 F50 X99.5 Y69.5 Z0.5 ; extrude filament
G1 E1707.7908 F50 X100.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1707.9271 F50 X99.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1708.0021 F50 X98 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1708.0521 F50 X98 Y71 Z0.5 ; extrude filament
G1 E1708.1885 F50 X97 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1708.3248 F50 X98 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1708.3998 F50 X99.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1708.4498 F50 X99.5 Y72.5 Z0.5 ; extrude filament
G1 E1708.5862 F50 X100.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1708.7225 F50 X99.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1708.7975 F50 X98 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1708.8475 F50 X98 Y74 Z0.5 ; extrude filament
G1 E1708.9839 F50 X97 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1709.1202 F50 X98 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1709.1952 F50 X99.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1709.2452 F50 X99.5 Y75.5 Z0.5 ; extrude filament
G1 E1709.3816 F50 X100.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1709.518 F50 X99.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1709.5929 F50 X98 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1709.643 F50 X98 Y77 Z0.5 ; extrude filament
G1 E1709.7793 F50 X97 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1709.9156 F50 X98 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1709.9906 F50 X99.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1710.0406 F50 X99.5 Y78.5 Z0.5 ; extrude filament
G1 E1710.177 F50 X100.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1710.3134 F50 X99.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1710.3883 F50 X98 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1710.4384 F50 X98 Y80 Z0.5 ; extrude filament
G1 E1710.5747 F50 X97 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1710.711 F50 X98 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1710.786 F50 X99.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1710.836 F50 X99.5 Y81.5 Z0.5 ; extrude filament
G1 E1710.9724 F50 X100.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1711.1088 F50 X99.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1711.1837 F50 X98 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1711.2338 F50 X98 Y83 Z0.5 ; extrude filament
G1 E1711.3701 F50 X97 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1711.5065 F50 X98 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1711.5814 F50 X99.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1711.6315 F50 X99.5 Y84.5 Z0.5 ; extrude filament
G1 E1711.7678 F50 X100.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1711.9042 F50 X99.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1711.9791 F50 X98 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1712.0292 F50 X98 Y86 Z0.5 ; extrude filament
G1 E1712.1655 F50 X97 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1712.3019 F50 X98 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1712.3768 F50 X99.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1712.4269 F50 X99.5 Y87.5 Z0.5 ; extrude filament
G1 E1712.5632 F50 X100.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1712.6996 F50 X99.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1712.7745 F50 X98 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1712.8246 F50 X98 Y89 Z0.5 ; extrude filament
G1 E1712.9609 F50 X97 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1713.0973 F50 X98 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1713.1722 F50 X99.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1713.2223 F50 X99.5 Y90.5 Z0.5 ; extrude filament
G1 E1713.3586 F50 X100.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1713.495 F50 X99.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1713.57 F50 X98 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1713.62 F50 X98 Y92 Z0.5 ; extrude filament
G1 E1713.7563 F50 X97 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1713.8927 F50 X98 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1713.9677 F50 X99.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1714.0177 F50 X99.5 Y93.5 Z0.5 ; extrude filament
G1 E1714.154 F50 X100.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1714.2904 F50 X99.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1714.3654 F50 X98 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1714.4154 F50 X98 Y95 Z0.5 ; extrude filament
G1 E1714.5518 F50 X97 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1714.6881 F50 X98 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1714.7631 F50 X99.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1714.8131 F50 X99.5 Y96.5 Z0.5 ; extrude filament
G1 E1714.9495 F50 X100.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1715.0858 F50 X99.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1715.1608 F50 X98 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1715.2108 F50 X98 Y98 Z0.5 ; extrude filament
G1 E1715.3472 F50 X97 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1715.4835 F50 X98 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1715.5585 F50 X99.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1715.6085 F50 X99.5 Y99.5 Z0.5 ; extrude filament
G1 E1715.7449 F50 X100.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1715.8812 F50 X99.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1715.9562 F50 X98 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1716.0062 F50 X98 Y101 Z0.5 ; extrude filament
G1 E1716.1426 F50 X97 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1716.2789 F50 X98 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1716.3539 F50 X99.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1716.4039 F50 X99.5 Y102.5 Z0.5 ; extrude filament
G1 E1716.5403 F50 X100.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1716.6766 F50 X99.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1716.7516 F50 X98 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1716.8016 F50 X98 Y104 Z0.5 ; extrude filament
G1 E1716.938 F50 X97 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1717.0743 F50 X98 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1717.1493 F50 X99.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1717.1993 F50 X99.5 Y105.5 Z0.5 ; extrude filament
G1 E1717.3357 F50 X100.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1717.472 F50 X99.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1717.547 F50 X98 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1717.597 F50 X98 Y107 Z0.5 ; extrude filament
G1 E1717.7334 F50 X97 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1717.8698 F50 X98 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1717.9447 F50 X99.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1717.9948 F50 X99.5 Y108.5 Z0.5 ; extrude filament
G1 E1718.1311 F50 X100.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1718.2675 F50 X99.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1718.3424 F50 X98 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1718.3925 F50 X98 Y110 Z0.5 ; extrude filament
G1 E1718.5288 F50 X97 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1718.6652 F50 X98 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1718.7401 F50 X99.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1718.7902 F50 X99.5 Y111.5 Z0.5 ; extrude filament
G1 E1718.9265 F50 X100.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1719.0629 F50 X99.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1719.1378 F50 X98 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1719.1879 F50 X98 Y113 Z0.5 ; extrude filament
G1 E1719.3242 F50 X97 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1719.4606 F50 X98 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1719.5355 F50 X99.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1719.5856 F50 X99.5 Y114.5 Z0.5 ; extrude filament
G1 E1719.7219 F50 X100.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1719.8583 F50 X99.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1719.9332 F50 X98 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1719.9833 F50 X98 Y116 Z0.5 ; extrude filament
G1 E1720.1196 F50 X97 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1720.256 F50 X98 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1720.3309 F50 X99.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1720.381 F50 X99.5 Y117.5 Z0.5 ; extrude filament
G1 E1720.5173 F50 X100.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1720.6537 F50 X99.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1720.7286 F50 X98 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1720.7787 F50 X98 Y119 Z0.5 ; extrude filament
G1 E1717.7787 F5000 X98 Y119 Z0.5 ; retract filament
G0 E1717.7787 F5000 X98 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1717.7787 F5000 X104 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1717.7787 F5000 X104 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1720.7787 F50 X104 Y20 Z0.5 ; extrude filament
G1 E1720.915 F50 X103 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1721.0514 F50 X104 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1721.1263 F50 X105.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1721.1764 F50 X105.5 Y21.5 Z0.5 ; extrude filament
G1 E1721.3127 F50 X106.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1721.4491 F50 X105.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1721.524 F50 X104 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1721.5741 F50 X104 Y23 Z0.5 ; extrude filament
G1 E1721.7104 F50 X103 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1721.8468 F50 X104 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1721.9218 F50 X105.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1721.9718 F50 X105.5 Y24.5 Z0.5 ; extrude filament
G1 E1722.1082 F50 X106.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1722.2445 F50 X105.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1722.3195 F50 X104 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1722.3695 F50 X104 Y26 Z0.5 ; extrude filament
G1 E1722.5059 F50 X103 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1722.6422 F50 X104 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1722.7172 F50 X105.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1722.7672 F50 X105.5 Y27.5 Z0.5 ; extrude filament
G1 E1722.9036 F50 X106.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1723.0399 F50 X105.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1723.1149 F50 X104 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1723.1649 F50 X104 Y29 Z0.5 ; extrude filament
G1 E1723.3013 F50 X103 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1723.4376 F50 X104 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1723.5126 F50 X105.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1723.5626 F50 X105.5 Y30.5 Z0.5 ; extrude filament
G1 E1723.699 F50 X106.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1723.8353 F50 X105.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1723.9103 F50 X104 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1723.9603 F50 X104 Y32 Z0.5 ; extrude filament
G1 E1724.0967 F50 X103 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1724.233 F50 X104 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1724.308 F50 X105.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1724.358 F50 X105.5 Y33.5 Z0.5 ; extrude filament
G1 E1724.4944 F50 X106.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1724.6307 F50 X105.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1724.7057 F50 X104 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1724.7557 F50 X104 Y35 Z0.5 ; extrude filament
G1 E1724.8921 F50 X103 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1725.0284 F50 X104 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1725.1034 F50 X105.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1725.1534 F50 X105.5 Y36.5 Z0.5 ; extrude filament
G1 E1725.2898 F50 X106.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1725.4261 F50 X105.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1725.5011 F50 X104 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1725.5511 F50 X104 Y38 Z0.5 ; extrude filament
G1 E1725.6875 F50 X103 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1725.8239 F50 X104 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1725.8988 F50 X105.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1725.9489 F50 X105.5 Y39.5 Z0.5 ; extrude filament
G1 E1726.0852 F50 X106.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1726.2216 F50 X105.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1726.2965 F50 X104 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1726.3466 F50 X104 Y41 Z0.5 ; extrude filament
G1 E1726.4829 F50 X103 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1726.6193 F50 X104 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1726.6942 F50 X105.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1726.7443 F50 X105.5 Y42.5 Z0.5 ; extrude filament
G1 E1726.8806 F50 X106.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1727.017 F50 X105.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1727.0919 F50 X104 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1727.142 F50 X104 Y44 Z0.5 ; extrude filament
G1 E1727.2783 F50 X103 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1727.4147 F50 X104 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1727.4896 F50 X105.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1727.5397 F50 X105.5 Y45.5 Z0.5 ; extrude filament
G1 E1727.676 F50 X106.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1727.8124 F50 X105.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1727.8873 F50 X104 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1727.9374 F50 X104 Y47 Z0.5 ; extrude filament
G1 E1728.0737 F50 X103 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1728.2101 F50 X104 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1728.285 F50 X105.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1728.3351 F50 X105.5 Y48.5 Z0.5 ; extrude filament
G1 E1728.4714 F50 X106.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1728.6078 F50 X105.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1728.6827 F50 X104 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1728.7328 F50 X104 Y50 Z0.5 ; extrude filament
G1 E1728.8691 F50 X103 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1729.0055 F50 X104 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1729.0804 F50 X105.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1729.1305 F50 X105.5 Y51.5 Z0.5 ; extrude filament
G1 E1729.2668 F50 X106.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1729.4032 F50 X105.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1729.4781 F50 X104 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1729.5282 F50 X104 Y53 Z0.5 ; extrude filament
G1 E1729.6646 F50 X103 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1729.8009 F50 X104 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1729.8759 F50 X105.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1729.9259 F50 X105.5 Y54.5 Z0.5 ; extrude filament
G1 E1730.0623 F50 X106.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1730.1986 F50 X105.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1730.2736 F50 X104 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1730.3236 F50 X104 Y56 Z0.5 ; extrude filament
G1 E1730.46 F50 X103 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1730.5963 F50 X104 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1730.6713 F50 X105.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1730.7213 F50 X105.5 Y57.5 Z0.5 ; extrude filament
G1 E1730.8577 F50 X106.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1730.994 F50 X105.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1731.069 F50 X104 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1731.119 F50 X104 Y59 Z0.5 ; extrude filament
G1 E1731.2554 F50 X103 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1731.3917 F50 X104 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1731.4667 F50 X105.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1731.5167 F50 X105.5 Y60.5 Z0.5 ; extrude filament
G1 E1731.6531 F50 X106.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1731.7894 F50 X105.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1731.8644 F50 X104 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1731.9144 F50 X104 Y62 Z0.5 ; extrude filament
G1 E1732.0508 F50 X103 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1732.1871 F50 X104 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1732.2621 F50 X105.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1732.3121 F50 X105.5 Y63.5 Z0.5 ; extrude filament
G1 E1732.4485 F50 X106.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1732.5848 F50 X105.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1732.6598 F50 X104 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1732.7098 F50 X104 Y65 Z0.5 ; extrude filament
G1 E1732.8462 F50 X103 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1732.9825 F50 X104 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1733.0575 F50 X105.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1733.1075 F50 X105.5 Y66.5 Z0.5 ; extrude filament
G1 E1733.2439 F50 X106.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1733.3802 F50 X105.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1733.4552 F50 X104 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1733.5052 F50 X104 Y68 Z0.5 ; extrude filament
G1 E1733.6416 F50 X103 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1733.778 F50 X104 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1733.8529 F50 X105.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1733.903 F50 X105.5 Y69.5 Z0.5 ; extrude filament
G1 E1734.0393 F50 X106.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1734.1757 F50 X105.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1734.2506 F50 X104 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1734.3007 F50 X104 Y71 Z0.5 ; extrude filament
G1 E1734.437 F50 X103 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1734.5734 F50 X104 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1734.6483 F50 X105.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1734.6984 F50 X105.5 Y72.5 Z0.5 ; extrude filament
G1 E1734.8347 F50 X106.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1734.9711 F50 X105.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1735.046 F50 X104 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1735.0961 F50 X104 Y74 Z0.5 ; extrude filament
G1 E1735.2324 F50 X103 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1735.3688 F50 X104 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1735.4437 F50 X105.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1735.4938 F50 X105.5 Y75.5 Z0.5 ; extrude filament
G1 E1735.6301 F50 X106.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1735.7665 F50 X105.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1735.8414 F50 X104 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1735.8915 F50 X104 Y77 Z0.5 ; extrude filament
G1 E1736.0278 F50 X103 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1736.1642 F50 X104 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1736.2391 F50 X105.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1736.2892 F50 X105.5 Y78.5 Z0.5 ; extrude filament
G1 E1736.4255 F50 X106.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1736.5619 F50 X105.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1736.6368 F50 X104 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1736.6869 F50 X104 Y80 Z0.5 ; extrude filament
G1 E1736.8232 F50 X103 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1736.9596 F50 X104 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1737.0345 F50 X105.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1737.0846 F50 X105.5 Y81.5 Z0.5 ; extrude filament
G1 E1737.221 F50 X106.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1737.3573 F50 X105.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1737.4323 F50 X104 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1737.4823 F50 X104 Y83 Z0.5 ; extrude filament
G1 E1737.6187 F50 X103 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1737.755 F50 X104 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1737.83 F50 X105.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1737.88 F50 X105.5 Y84.5 Z0.5 ; extrude filament
G1 E1738.0164 F50 X106.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1738.1527 F50 X105.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1738.2277 F50 X104 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1738.2777 F50 X104 Y86 Z0.5 ; extrude filament
G1 E1738.4141 F50 X103 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1738.5504 F50 X104 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1738.6254 F50 X105.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1738.6754 F50 X105.5 Y87.5 Z0.5 ; extrude filament
G1 E1738.8118 F50 X106.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1738.9481 F50 X105.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1739.0231 F50 X104 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1739.0731 F50 X104 Y89 Z0.5 ; extrude filament
G1 E1739.2095 F50 X103 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1739.3458 F50 X104 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1739.4208 F50 X105.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1739.4708 F50 X105.5 Y90.5 Z0.5 ; extrude filament
G1 E1739.6072 F50 X106.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1739.7435 F50 X105.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1739.8185 F50 X104 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1739.8685 F50 X104 Y92 Z0.5 ; extrude filament
G1 E1740.0049 F50 X103 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1740.1412 F50 X104 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1740.2162 F50 X105.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1740.2662 F50 X105.5 Y93.5 Z0.5 ; extrude filament
G1 E1740.4026 F50 X106.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1740.539 F50 X105.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1740.6139 F50 X104 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1740.664 F50 X104 Y95 Z0.5 ; extrude filament
G1 E1740.8003 F50 X103 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1740.9366 F50 X104 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1741.0116 F50 X105.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1741.0616 F50 X105.5 Y96.5 Z0.5 ; extrude filament
G1 E1741.198 F50 X106.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1741.3344 F50 X105.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1741.4093 F50 X104 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1741.4594 F50 X104 Y98 Z0.5 ; extrude filament
G1 E1741.5957 F50 X103 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1741.732 F50 X104 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1741.807 F50 X105.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1741.857 F50 X105.5 Y99.5 Z0.5 ; extrude filament
G1 E1741.9934 F50 X106.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1742.1298 F50 X105.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1742.2047 F50 X104 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1742.2548 F50 X104 Y101 Z0.5 ; extrude filament
G1 E1742.3911 F50 X103 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1742.5275 F50 X104 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1742.6024 F50 X105.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1742.6525 F50 X105.5 Y102.5 Z0.5 ; extrude filament
G1 E1742.7888 F50 X106.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1742.9252 F50 X105.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1743.0001 F50 X104 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1743.0502 F50 X104 Y104 Z0.5 ; extrude filament
G1 E1743.1865 F50 X103 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1743.3229 F50 X104 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1743.3978 F50 X105.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1743.4479 F50 X105.5 Y105.5 Z0.5 ; extrude filament
G1 E1743.5842 F50 X106.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1743.7206 F50 X105.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1743.7955 F50 X104 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1743.8456 F50 X104 Y107 Z0.5 ; extrude filament
G1 E1743.9819 F50 X103 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1744.1183 F50 X104 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1744.1932 F50 X105.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1744.2433 F50 X105.5 Y108.5 Z0.5 ; extrude filament
G1 E1744.3796 F50 X106.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1744.516 F50 X105.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1744.591 F50 X104 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1744.641 F50 X104 Y110 Z0.5 ; extrude filament
G1 E1744.7773 F50 X103 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1744.9137 F50 X104 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1744.9886 F50 X105.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1745.0387 F50 X105.5 Y111.5 Z0.5 ; extrude filament
G1 E1745.175 F50 X106.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1745.3114 F50 X105.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1745.3864 F50 X104 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1745.4364 F50 X104 Y113 Z0.5 ; extrude filament
G1 E1745.5728 F50 X103 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1745.7091 F50 X104 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1745.784 F50 X105.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1745.8341 F50 X105.5 Y114.5 Z0.5 ; extrude filament
G1 E1745.9705 F50 X106.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1746.1068 F50 X105.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1746.1818 F50 X104 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1746.2318 F50 X104 Y116 Z0.5 ; extrude filament
G1 E1746.3682 F50 X103 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1746.5045 F50 X104 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1746.5795 F50 X105.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1746.6295 F50 X105.5 Y117.5 Z0.5 ; extrude filament
G1 E1746.7659 F50 X106.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1746.9022 F50 X105.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1746.9772 F50 X104 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1747.0272 F50 X104 Y119 Z0.5 ; extrude filament
G1 E1744.0272 F5000 X104 Y119 Z0.5 ; retract filament
G0 E1744.0272 F5000 X104 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1744.0272 F5000 X110 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1744.0272 F5000 X110 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1747.0272 F50 X110 Y20 Z0.5 ; extrude filament
G1 E1747.1636 F50 X109 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1747.2999 F50 X110 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1747.3749 F50 X111.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1747.4249 F50 X111.5 Y21.5 Z0.5 ; extrude filament
G1 E1747.5613 F50 X112.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1747.6976 F50 X111.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1747.7726 F50 X110 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1747.8226 F50 X110 Y23 Z0.5 ; extrude filament
G1 E1747.959 F50 X109 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1748.0953 F50 X110 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1748.1703 F50 X111.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1748.2203 F50 X111.5 Y24.5 Z0.5 ; extrude filament
G1 E1748.3567 F50 X112.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1748.493 F50 X111.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1748.568 F50 X110 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1748.618 F50 X110 Y26 Z0.5 ; extrude filament
G1 E1748.7544 F50 X109 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1748.8907 F50 X110 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1748.9657 F50 X111.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1749.0157 F50 X111.5 Y27.5 Z0.5 ; extrude filament
G1 E1749.1521 F50 X112.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1749.2885 F50 X111.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1749.3634 F50 X110 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1749.4135 F50 X110 Y29 Z0.5 ; extrude filament
G1 E1749.5498 F50 X109 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1749.6862 F50 X110 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1749.7611 F50 X111.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1749.8112 F50 X111.5 Y30.5 Z0.5 ; extrude filament
G1 E1749.9475 F50 X112.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1750.0839 F50 X111.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1750.1588 F50 X110 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1750.2089 F50 X110 Y32 Z0.5 ; extrude filament
G1 E1750.3452 F50 X109 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1750.4816 F50 X110 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1750.5565 F50 X111.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1750.6066 F50 X111.5 Y33.5 Z0.5 ; extrude filament
G1 E1750.7429 F50 X112.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1750.8793 F50 X111.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1750.9542 F50 X110 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1751.0043 F50 X110 Y35 Z0.5 ; extrude filament
G1 E1751.1406 F50 X109 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1751.277 F50 X110 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1751.3519 F50 X111.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1751.402 F50 X111.5 Y36.5 Z0.5 ; extrude filament
G1 E1751.5383 F50 X112.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1751.6747 F50 X111.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1751.7496 F50 X110 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1751.7997 F50 X110 Y38 Z0.5 ; extrude filament
G1 E1751.936 F50 X109 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1752.0724 F50 X110 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1752.1473 F50 X111.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1752.1974 F50 X111.5 Y39.5 Z0.5 ; extrude filament
G1 E1752.3337 F50 X112.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1752.4701 F50 X111.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1752.545 F50 X110 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1752.5951 F50 X110 Y41 Z0.5 ; extrude filament
G1 E1752.7314 F50 X109 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1752.8678 F50 X110 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1752.9427 F50 X111.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1752.9928 F50 X111.5 Y42.5 Z0.5 ; extrude filament
G1 E1753.1292 F50 X112.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1753.2655 F50 X111.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1753.3405 F50 X110 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1753.3905 F50 X110 Y44 Z0.5 ; extrude filament
G1 E1753.5269 F50 X109 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1753.6632 F50 X110 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1753.7382 F50 X111.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1753.7882 F50 X111.5 Y45.5 Z0.5 ; extrude filament
G1 E1753.9246 F50 X112.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1754.0609 F50 X111.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1754.1359 F50 X110 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1754.1859 F50 X110 Y47 Z0.5 ; extrude filament
G1 E1754.3223 F50 X109 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1754.4586 F50 X110 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1754.5336 F50 X111.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1754.5836 F50 X111.5 Y48.5 Z0.5 ; extrude filament
G1 E1754.72 F50 X112.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1754.8563 F50 X111.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1754.9313 F50 X110 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1754.9813 F50 X110 Y50 Z0.5 ; extrude filament
G1 E1755.1177 F50 X109 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1755.254 F50 X110 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1755.329 F50 X111.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1755.379 F50 X111.5 Y51.5 Z0.5 ; extrude filament
G1 E1755.5154 F50 X112.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1755.6517 F50 X111.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1755.7267 F50 X110 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1755.7767 F50 X110 Y53 Z0.5 ; extrude filament
G1 E1755.9131 F50 X109 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1756.0494 F50 X110 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1756.1244 F50 X111.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1756.1744 F50 X111.5 Y54.5 Z0.5 ; extrude filament
G1 E1756.3108 F50 X112.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1756.4471 F50 X111.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1756.5221 F50 X110 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1756.5721 F50 X110 Y56 Z0.5 ; extrude filament
G1 E1756.7085 F50 X109 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1756.8448 F50 X110 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1756.9198 F50 X111.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1756.9698 F50 X111.5 Y57.5 Z0.5 ; extrude filament
G1 E1757.1062 F50 X112.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1757.2426 F50 X111.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1757.3175 F50 X110 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1757.3676 F50 X110 Y59 Z0.5 ; extrude filament
G1 E1757.5039 F50 X109 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1757.6403 F50 X110 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1757.7152 F50 X111.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1757.7653 F50 X111.5 Y60.5 Z0.5 ; extrude filament
G1 E1757.9016 F50 X112.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1758.038 F50 X111.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1758.1129 F50 X110 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1758.163 F50 X110 Y62 Z0.5 ; extrude filament
G1 E1758.2993 F50 X109 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1758.4357 F50 X110 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1758.5106 F50 X111.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1758.5607 F50 X111.5 Y63.5 Z0.5 ; extrude filament
G1 E1758.697 F50 X112.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1758.8334 F50 X111.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1758.9083 F50 X110 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1758.9584 F50 X110 Y65 Z0.5 ; extrude filament
G1 E1759.0947 F50 X109 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1759.2311 F50 X110 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1759.306 F50 X111.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1759.3561 F50 X111.5 Y66.5 Z0.5 ; extrude filament
G1 E1759.4924 F50 X112.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1759.6288 F50 X111.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1759.7037 F50 X110 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1759.7538 F50 X110 Y68 Z0.5 ; extrude filament
G1 E1759.8901 F50 X109 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1760.0265 F50 X110 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1760.1014 F50 X111.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1760.1515 F50 X111.5 Y69.5 Z0.5 ; extrude filament
G1 E1760.2878 F50 X112.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1760.4242 F50 X111.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1760.4991 F50 X110 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1760.5492 F50 X110 Y71 Z0.5 ; extrude filament
G1 E1760.6855 F50 X109 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1760.8219 F50 X110 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1760.8969 F50 X111.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1760.9469 F50 X111.5 Y72.5 Z0.5 ; extrude filament
G1 E1761.0833 F50 X112.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1761.2196 F50 X111.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1761.2946 F50 X110 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1761.3446 F50 X110 Y74 Z0.5 ; extrude filament
G1 E1761.481 F50 X109 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1761.6173 F50 X110 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1761.6923 F50 X111.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1761.7423 F50 X111.5 Y75.5 Z0.5 ; extrude filament
G1 E1761.8787 F50 X112.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1762.015 F50 X111.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1762.09 F50 X110 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1762.14 F50 X110 Y77 Z0.5 ; extrude filament
G1 E1762.2764 F50 X109 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1762.4127 F50 X110 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1762.4877 F50 X111.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1762.5377 F50 X111.5 Y78.5 Z0.5 ; extrude filament
G1 E1762.6741 F50 X112.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1762.8104 F50 X111.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1762.8854 F50 X110 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1762.9354 F50 X110 Y80 Z0.5 ; extrude filament
G1 E1763.0718 F50 X109 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1763.2081 F50 X110 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1763.2831 F50 X111.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1763.3331 F50 X111.5 Y81.5 Z0.5 ; extrude filament
G1 E1763.4695 F50 X112.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1763.6058 F50 X111.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1763.6808 F50 X110 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1763.7308 F50 X110 Y83 Z0.5 ; extrude filament
G1 E1763.8672 F50 X109 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1764.0035 F50 X110 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1764.0785 F50 X111.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1764.1285 F50 X111.5 Y84.5 Z0.5 ; extrude filament
G1 E1764.2649 F50 X112.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1764.4012 F50 X111.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1764.4762 F50 X110 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1764.5262 F50 X110 Y86 Z0.5 ; extrude filament
G1 E1764.6626 F50 X109 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1764.799 F50 X110 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1764.8739 F50 X111.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1764.924 F50 X111.5 Y87.5 Z0.5 ; extrude filament
G1 E1765.0603 F50 X112.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1765.1967 F50 X111.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1765.2716 F50 X110 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1765.3217 F50 X110 Y89 Z0.5 ; extrude filament
G1 E1765.458 F50 X109 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1765.5944 F50 X110 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1765.6693 F50 X111.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1765.7194 F50 X111.5 Y90.5 Z0.5 ; extrude filament
G1 E1765.8557 F50 X112.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1765.9921 F50 X111.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1766.067 F50 X110 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1766.1171 F50 X110 Y92 Z0.5 ; extrude filament
G1 E1766.2534 F50 X109 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1766.3898 F50 X110 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1766.4647 F50 X111.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1766.5148 F50 X111.5 Y93.5 Z0.5 ; extrude filament
G1 E1766.6511 F50 X112.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1766.7875 F50 X111.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1766.8624 F50 X110 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1766.9125 F50 X110 Y95 Z0.5 ; extrude filament
G1 E1767.0488 F50 X109 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1767.1852 F50 X110 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1767.2601 F50 X111.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1767.3102 F50 X111.5 Y96.5 Z0.5 ; extrude filament
G1 E1767.4465 F50 X112.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1767.5829 F50 X111.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1767.6578 F50 X110 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1767.7079 F50 X110 Y98 Z0.5 ; extrude filament
G1 E1767.8442 F50 X109 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1767.9806 F50 X110 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1768.0555 F50 X111.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1768.1056 F50 X111.5 Y99.5 Z0.5 ; extrude filament
G1 E1768.242 F50 X112.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1768.3783 F50 X111.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1768.4532 F50 X110 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1768.5033 F50 X110 Y101 Z0.5 ; extrude filament
G1 E1768.6396 F50 X109 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1768.776 F50 X110 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1768.851 F50 X111.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1768.901 F50 X111.5 Y102.5 Z0.5 ; extrude filament
G1 E1769.0374 F50 X112.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1769.1737 F50 X111.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1769.2487 F50 X110 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1769.2987 F50 X110 Y104 Z0.5 ; extrude filament
G1 E1769.435 F50 X109 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1769.5714 F50 X110 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1769.6464 F50 X111.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1769.6964 F50 X111.5 Y105.5 Z0.5 ; extrude filament
G1 E1769.8328 F50 X112.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1769.9691 F50 X111.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1770.0441 F50 X110 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1770.0941 F50 X110 Y107 Z0.5 ; extrude filament
G1 E1770.2305 F50 X109 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1770.3668 F50 X110 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1770.4418 F50 X111.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1770.4918 F50 X111.5 Y108.5 Z0.5 ; extrude filament
G1 E1770.6282 F50 X112.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1770.7645 F50 X111.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1770.8395 F50 X110 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1770.8895 F50 X110 Y110 Z0.5 ; extrude filament
G1 E1771.0259 F50 X109 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1771.1622 F50 X110 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1771.2372 F50 X111.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1771.2872 F50 X111.5 Y111.5 Z0.5 ; extrude filament
G1 E1771.4236 F50 X112.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1771.5599 F50 X111.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1771.6349 F50 X110 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1771.6849 F50 X110 Y113 Z0.5 ; extrude filament
G1 E1771.8213 F50 X109 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1771.9576 F50 X110 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1772.0326 F50 X111.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1772.0826 F50 X111.5 Y114.5 Z0.5 ; extrude filament
G1 E1772.219 F50 X112.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1772.3553 F50 X111.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1772.4303 F50 X110 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1772.4803 F50 X110 Y116 Z0.5 ; extrude filament
G1 E1772.6167 F50 X109 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1772.753 F50 X110 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1772.828 F50 X111.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1772.878 F50 X111.5 Y117.5 Z0.5 ; extrude filament
G1 E1773.0144 F50 X112.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1773.1508 F50 X111.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1773.2257 F50 X110 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1773.2758 F50 X110 Y119 Z0.5 ; extrude filament
G1 E1770.2758 F5000 X110 Y119 Z0.5 ; retract filament
G0 E1770.2758 F5000 X110 Y119 Z2.5 ; Move to (x,y,z) without extruding
G0 E1770.2758 F5000 X116 Y20 Z2.5 ; Move to (x,y,z) without extruding
G0 E1770.2758 F5000 X116 Y20 Z0.5 ; Move to (x,y,z) without extruding
G1 E1773.2758 F50 X116 Y20 Z0.5 ; extrude filament
G1 E1773.4121 F50 X115 Y19 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1773.5485 F50 X116 Y20 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1773.6234 F50 X117.5 Y21.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1773.6735 F50 X117.5 Y21.5 Z0.5 ; extrude filament
G1 E1773.8098 F50 X118.5 Y20.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1773.9462 F50 X117.5 Y21.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1774.0211 F50 X116 Y23 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1774.0712 F50 X116 Y23 Z0.5 ; extrude filament
G1 E1774.2075 F50 X115 Y22 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1774.3439 F50 X116 Y23 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1774.4188 F50 X117.5 Y24.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1774.4689 F50 X117.5 Y24.5 Z0.5 ; extrude filament
G1 E1774.6052 F50 X118.5 Y23.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1774.7416 F50 X117.5 Y24.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1774.8165 F50 X116 Y26 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1774.8666 F50 X116 Y26 Z0.5 ; extrude filament
G1 E1775.0029 F50 X115 Y25 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1775.1393 F50 X116 Y26 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1775.2142 F50 X117.5 Y27.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1775.2643 F50 X117.5 Y27.5 Z0.5 ; extrude filament
G1 E1775.4006 F50 X118.5 Y26.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1775.537 F50 X117.5 Y27.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1775.6119 F50 X116 Y29 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1775.662 F50 X116 Y29 Z0.5 ; extrude filament
G1 E1775.7983 F50 X115 Y28 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1775.9347 F50 X116 Y29 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1776.0096 F50 X117.5 Y30.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1776.0597 F50 X117.5 Y30.5 Z0.5 ; extrude filament
G1 E1776.196 F50 X118.5 Y29.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1776.3324 F50 X117.5 Y30.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1776.4073 F50 X116 Y32 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1776.4574 F50 X116 Y32 Z0.5 ; extrude filament
G1 E1776.5938 F50 X115 Y31 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1776.7301 F50 X116 Y32 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1776.805 F50 X117.5 Y33.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1776.8551 F50 X117.5 Y33.5 Z0.5 ; extrude filament
G1 E1776.9915 F50 X118.5 Y32.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1777.1278 F50 X117.5 Y33.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1777.2028 F50 X116 Y35 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1777.2528 F50 X116 Y35 Z0.5 ; extrude filament
G1 E1777.3892 F50 X115 Y34 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1777.5255 F50 X116 Y35 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1777.6005 F50 X117.5 Y36.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1777.6505 F50 X117.5 Y36.5 Z0.5 ; extrude filament
G1 E1777.7869 F50 X118.5 Y35.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1777.9232 F50 X117.5 Y36.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1777.9982 F50 X116 Y38 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1778.0482 F50 X116 Y38 Z0.5 ; extrude filament
G1 E1778.1846 F50 X115 Y37 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1778.3209 F50 X116 Y38 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1778.3959 F50 X117.5 Y39.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1778.4459 F50 X117.5 Y39.5 Z0.5 ; extrude filament
G1 E1778.5823 F50 X118.5 Y38.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1778.7186 F50 X117.5 Y39.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1778.7936 F50 X116 Y41 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1778.8436 F50 X116 Y41 Z0.5 ; extrude filament
G1 E1778.98 F50 X115 Y40 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1779.1163 F50 X116 Y41 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1779.1913 F50 X117.5 Y42.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1779.2413 F50 X117.5 Y42.5 Z0.5 ; extrude filament
G1 E1779.3777 F50 X118.5 Y41.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1779.514 F50 X117.5 Y42.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1779.589 F50 X116 Y44 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1779.639 F50 X116 Y44 Z0.5 ; extrude filament
G1 E1779.7754 F50 X115 Y43 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1779.9117 F50 X116 Y44 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1779.9867 F50 X117.5 Y45.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1780.0367 F50 X117.5 Y45.5 Z0.5 ; extrude filament
G1 E1780.1731 F50 X118.5 Y44.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1780.3094 F50 X117.5 Y45.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1780.3844 F50 X116 Y47 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1780.4344 F50 X116 Y47 Z0.5 ; extrude filament
G1 E1780.5708 F50 X115 Y46 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1780.7072 F50 X116 Y47 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1780.7821 F50 X117.5 Y48.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1780.8322 F50 X117.5 Y48.5 Z0.5 ; extrude filament
G1 E1780.9685 F50 X118.5 Y47.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1781.1049 F50 X117.5 Y48.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1781.1798 F50 X116 Y50 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1781.2299 F50 X116 Y50 Z0.5 ; extrude filament
G1 E1781.3662 F50 X115 Y49 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1781.5026 F50 X116 Y50 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1781.5775 F50 X117.5 Y51.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1781.6276 F50 X117.5 Y51.5 Z0.5 ; extrude filament
G1 E1781.7639 F50 X118.5 Y50.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1781.9003 F50 X117.5 Y51.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1781.9752 F50 X116 Y53 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1782.0253 F50 X116 Y53 Z0.5 ; extrude filament
G1 E1782.1616 F50 X115 Y52 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1782.298 F50 X116 Y53 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1782.3729 F50 X117.5 Y54.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1782.423 F50 X117.5 Y54.5 Z0.5 ; extrude filament
G1 E1782.5593 F50 X118.5 Y53.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1782.6957 F50 X117.5 Y54.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1782.7706 F50 X116 Y56 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1782.8207 F50 X116 Y56 Z0.5 ; extrude filament
G1 E1782.957 F50 X115 Y55 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1783.0934 F50 X116 Y56 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1783.1683 F50 X117.5 Y57.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1783.2184 F50 X117.5 Y57.5 Z0.5 ; extrude filament
G1 E1783.3547 F50 X118.5 Y56.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1783.4911 F50 X117.5 Y57.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1783.566 F50 X116 Y59 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1783.6161 F50 X116 Y59 Z0.5 ; extrude filament
G1 E1783.7524 F50 X115 Y58 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1783.8888 F50 X116 Y59 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1783.9637 F50 X117.5 Y60.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1784.0138 F50 X117.5 Y60.5 Z0.5 ; extrude filament
G1 E1784.1501 F50 X118.5 Y59.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1784.2865 F50 X117.5 Y60.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1784.3615 F50 X116 Y62 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1784.4115 F50 X116 Y62 Z0.5 ; extrude filament
G1 E1784.5479 F50 X115 Y61 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1784.6842 F50 X116 Y62 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1784.7592 F50 X117.5 Y63.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1784.8092 F50 X117.5 Y63.5 Z0.5 ; extrude filament
G1 E1784.9456 F50 X118.5 Y62.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1785.0819 F50 X117.5 Y63.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1785.1569 F50 X116 Y65 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1785.2069 F50 X116 Y65 Z0.5 ; extrude filament
G1 E1785.3433 F50 X115 Y64 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1785.4796 F50 X116 Y65 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1785.5546 F50 X117.5 Y66.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1785.6046 F50 X117.5 Y66.5 Z0.5 ; extrude filament
G1 E1785.741 F50 X118.5 Y65.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1785.8773 F50 X117.5 Y66.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1785.9523 F50 X116 Y68 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1786.0023 F50 X116 Y68 Z0.5 ; extrude filament
G1 E1786.1387 F50 X115 Y67 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1786.275 F50 X116 Y68 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1786.35 F50 X117.5 Y69.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1786.4 F50 X117.5 Y69.5 Z0.5 ; extrude filament
G1 E1786.5364 F50 X118.5 Y68.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1786.6727 F50 X117.5 Y69.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1786.7477 F50 X116 Y71 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1786.7977 F50 X116 Y71 Z0.5 ; extrude filament
G1 E1786.9341 F50 X115 Y70 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1787.0704 F50 X116 Y71 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1787.1454 F50 X117.5 Y72.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1787.1954 F50 X117.5 Y72.5 Z0.5 ; extrude filament
G1 E1787.3318 F50 X118.5 Y71.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1787.4681 F50 X117.5 Y72.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1787.5431 F50 X116 Y74 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1787.5931 F50 X116 Y74 Z0.5 ; extrude filament
G1 E1787.7295 F50 X115 Y73 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1787.8658 F50 X116 Y74 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1787.9408 F50 X117.5 Y75.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1787.9908 F50 X117.5 Y75.5 Z0.5 ; extrude filament
G1 E1788.1272 F50 X118.5 Y74.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1788.2635 F50 X117.5 Y75.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1788.3385 F50 X116 Y77 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1788.3885 F50 X116 Y77 Z0.5 ; extrude filament
G1 E1788.5249 F50 X115 Y76 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1788.6613 F50 X116 Y77 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1788.7362 F50 X117.5 Y78.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1788.7863 F50 X117.5 Y78.5 Z0.5 ; extrude filament
G1 E1788.9226 F50 X118.5 Y77.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1789.059 F50 X117.5 Y78.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1789.1339 F50 X116 Y80 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1789.184 F50 X116 Y80 Z0.5 ; extrude filament
G1 E1789.3203 F50 X115 Y79 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1789.4567 F50 X116 Y80 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1789.5316 F50 X117.5 Y81.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1789.5817 F50 X117.5 Y81.5 Z0.5 ; extrude filament
G1 E1789.718 F50 X118.5 Y80.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1789.8544 F50 X117.5 Y81.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1789.9293 F50 X116 Y83 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1789.9794 F50 X116 Y83 Z0.5 ; extrude filament
G1 E1790.1157 F50 X115 Y82 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1790.2521 F50 X116 Y83 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1790.327 F50 X117.5 Y84.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1790.3771 F50 X117.5 Y84.5 Z0.5 ; extrude filament
G1 E1790.5134 F50 X118.5 Y83.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1790.6498 F50 X117.5 Y84.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1790.7247 F50 X116 Y86 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1790.7748 F50 X116 Y86 Z0.5 ; extrude filament
G1 E1790.9111 F50 X115 Y85 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1791.0475 F50 X116 Y86 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1791.1224 F50 X117.5 Y87.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1791.1725 F50 X117.5 Y87.5 Z0.5 ; extrude filament
G1 E1791.3088 F50 X118.5 Y86.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1791.4452 F50 X117.5 Y87.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1791.5201 F50 X116 Y89 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1791.5702 F50 X116 Y89 Z0.5 ; extrude filament
G1 E1791.7065 F50 X115 Y88 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1791.8429 F50 X116 Y89 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1791.9178 F50 X117.5 Y90.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1791.9679 F50 X117.5 Y90.5 Z0.5 ; extrude filament
G1 E1792.1042 F50 X118.5 Y89.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1792.2406 F50 X117.5 Y90.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1792.3156 F50 X116 Y92 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1792.3656 F50 X116 Y92 Z0.5 ; extrude filament
G1 E1792.502 F50 X115 Y91 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1792.6383 F50 X116 Y92 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1792.7133 F50 X117.5 Y93.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1792.7633 F50 X117.5 Y93.5 Z0.5 ; extrude filament
G1 E1792.8997 F50 X118.5 Y92.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1793.036 F50 X117.5 Y93.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1793.111 F50 X116 Y95 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1793.161 F50 X116 Y95 Z0.5 ; extrude filament
G1 E1793.2974 F50 X115 Y94 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1793.4337 F50 X116 Y95 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1793.5087 F50 X117.5 Y96.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1793.5587 F50 X117.5 Y96.5 Z0.5 ; extrude filament
G1 E1793.6951 F50 X118.5 Y95.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1793.8314 F50 X117.5 Y96.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1793.9064 F50 X116 Y98 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1793.9564 F50 X116 Y98 Z0.5 ; extrude filament
G1 E1794.0928 F50 X115 Y97 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1794.2291 F50 X116 Y98 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1794.3041 F50 X117.5 Y99.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1794.3541 F50 X117.5 Y99.5 Z0.5 ; extrude filament
G1 E1794.4905 F50 X118.5 Y98.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1794.6268 F50 X117.5 Y99.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1794.7018 F50 X116 Y101 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1794.7518 F50 X116 Y101 Z0.5 ; extrude filament
G1 E1794.8882 F50 X115 Y100 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1795.0245 F50 X116 Y101 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1795.0995 F50 X117.5 Y102.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1795.1495 F50 X117.5 Y102.5 Z0.5 ; extrude filament
G1 E1795.2859 F50 X118.5 Y101.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1795.4222 F50 X117.5 Y102.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1795.4972 F50 X116 Y104 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1795.5472 F50 X116 Y104 Z0.5 ; extrude filament
G1 E1795.6836 F50 X115 Y103 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1795.82 F50 X116 Y104 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1795.8949 F50 X117.5 Y105.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1795.945 F50 X117.5 Y105.5 Z0.5 ; extrude filament
G1 E1796.0813 F50 X118.5 Y104.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1796.2177 F50 X117.5 Y105.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1796.2926 F50 X116 Y107 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1796.3427 F50 X116 Y107 Z0.5 ; extrude filament
G1 E1796.479 F50 X115 Y106 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1796.6154 F50 X116 Y107 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1796.6903 F50 X117.5 Y108.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1796.7404 F50 X117.5 Y108.5 Z0.5 ; extrude filament
G1 E1796.8767 F50 X118.5 Y107.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1797.0131 F50 X117.5 Y108.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1797.088 F50 X116 Y110 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1797.1381 F50 X116 Y110 Z0.5 ; extrude filament
G1 E1797.2744 F50 X115 Y109 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1797.4108 F50 X116 Y110 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1797.4857 F50 X117.5 Y111.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1797.5358 F50 X117.5 Y111.5 Z0.5 ; extrude filament
G1 E1797.6721 F50 X118.5 Y110.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1797.8085 F50 X117.5 Y111.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1797.8834 F50 X116 Y113 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1797.9335 F50 X116 Y113 Z0.5 ; extrude filament
G1 E1798.0698 F50 X115 Y112 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1798.2062 F50 X116 Y113 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1798.2811 F50 X117.5 Y114.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1798.3312 F50 X117.5 Y114.5 Z0.5 ; extrude filament
G1 E1798.4675 F50 X118.5 Y113.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1798.6039 F50 X117.5 Y114.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1798.6788 F50 X116 Y116 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1798.7289 F50 X116 Y116 Z0.5 ; extrude filament
G1 E1798.8652 F50 X115 Y115 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1799.0016 F50 X116 Y116 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1799.0765 F50 X117.5 Y117.5 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1799.1266 F50 X117.5 Y117.5 Z0.5 ; extrude filament
G1 E1799.263 F50 X118.5 Y116.5 Z2 ; Extrude line from current location to destination (x,y,z)
G1 E1799.3993 F50 X117.5 Y117.5 Z3.5 ; Extrude line from current location to destination (x,y,z)
G1 E1799.4742 F50 X116 Y119 Z0.5 ; Extrude line from current location to destination (x,y,z)
G1 E1799.5243 F50 X116 Y119 Z0.5 ; extrude filament
G1 E1796.5243 F5000 X116 Y119 Z0.5 ; retract filament
G0 E1796.5243 F5000 X116 Y119 Z2.5 ; Move to (x,y,z) without extruding
G1 E1791.5243 F5000 X116 Y119 Z2.5 ; retract filament
M140 S0 ; Set bed temp and do not wait
M104 S0 ; Set hotend temp and do not wait
M106 S0 ; Set fan speed in 0-255 range
G0 E1791.5243 F5000 X0 Y200 Z12.5 ; Move to (x,y,z) without extruding


```