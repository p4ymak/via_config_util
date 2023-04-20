# via_config_util 
## Utility to check, adjust and mirror keymaps in VIA config JSON for Split Keyboards.

```sh
USAGE:
    via_config_util [OPTIONS] --input <INPUT> --width <WIDTH> --height <HEIGHT>

OPTIONS:
        --add_cols_center <ADD_COLS_CENTER>    Add columns to center. (OPTIONAL)
        --add_cols_sides <ADD_COLS_SIDES>      Add columns to sides. (OPTIONAL)
        --add_rows_bottom <ADD_ROWS_BOTTOM>    Add rows to bottom. (OPTIONAL)
        --add_rows_top <ADD_ROWS_TOP>          Add rows to top. (OPTIONAL)
    -h, --height <HEIGHT>                      Columns in one part of keyboard
        --help                                 Print help information
    -i, --input <INPUT>                        Path to existing VIA JSON config
    -m, --mirror                               Mirror keymap. (OPTIONAL)
    -o, --output <OUTPUT>                      Path to save new VIA JSON config. (OPTIONAL)
        --rm_cols_center <RM_COLS_CENTER>      Remove columns from center. (OPTIONAL)
        --rm_cols_sides <RM_COLS_SIDES>        Remove columns from sides. (OPTIONAL)
        --rm_rows_bottom <RM_ROWS_BOTTOM>      Remove rows from bottom. (OPTIONAL)
        --rm_rows_top <RM_ROWS_TOP>            Remove rows from top. (OPTIONAL)
    -v, --verbose                              Print keymap (OPTIONAL)
    -w, --width <WIDTH>                        Rows in one part of keyboard
```

## Examples

### Mirror Keymap
```sh
via_config_util -i 'path_to_config.json' -w 6 -h 4 -m -o 'path_to_new_config.json'

Keyboard: Crkbd. Layer: 0
 [ ESC ]  [  Q  ]  [  W  ]  [  E  ]  [  R  ]  [  T  ]        [  Y  ]  [  U  ]  [  I  ]  [  O  ]  [  P  ]  [BSPC ] 
 [LSFT ]  [  A  ]  [  S  ]  [  D  ]  [  F  ]  [  G  ]        [  H  ]  [  J  ]  [  K  ]  [  L  ]  [SCLN ]  [QUOT ] 
 [LCTL ]  [  Z  ]  [  X  ]  [  C  ]  [  V  ]  [  B  ]        [  N  ]  [  M  ]  [COMM ]  [ DOT ]  [SLSH ]  [RSFT ] 
 [     ]  [     ]  [     ]  [LGUI ]  [MO13 ]  [ SPC ]        [ ENT ]  [MO23 ]  [RALT ]  [     ]  [     ]  [     ] 

....

Mirrored Layout:
Keyboard: Crkbd. Layer: 0
 [BSPC ]  [  P  ]  [  O  ]  [  I  ]  [  U  ]  [  Y  ]        [  T  ]  [  R  ]  [  E  ]  [  W  ]  [  Q  ]  [ ESC ] 
 [QUOT ]  [SCLN ]  [  L  ]  [  K  ]  [  J  ]  [  H  ]        [  G  ]  [  F  ]  [  D  ]  [  S  ]  [  A  ]  [LSFT ] 
 [RSFT ]  [SLSH ]  [ DOT ]  [COMM ]  [  M  ]  [  N  ]        [  B  ]  [  V  ]  [  C  ]  [  X  ]  [  Z  ]  [LCTL ] 
 [     ]  [     ]  [     ]  [RALT ]  [MO23 ]  [ ENT ]        [ SPC ]  [MO13 ]  [LGUI ]  [     ]  [     ]  [     ] 
```

### Adding Columns and Rows
```sh
via_config_util -i 'path_to_config.json' -w 6 -h 4 -o 'path_to_new_config.json' --add_rows_top 2 --add_cols_sides 1

Added 2 row(s) to top:
Keyboard: Crkbd. Layer: 0
 [     ]  [     ]  [     ]  [     ]  [     ]  [     ]        [     ]  [     ]  [     ]  [     ]  [     ]  [     ] 
 [     ]  [     ]  [     ]  [     ]  [     ]  [     ]        [     ]  [     ]  [     ]  [     ]  [     ]  [     ] 
 [ ESC ]  [  Q  ]  [  W  ]  [  E  ]  [  R  ]  [  T  ]        [  Y  ]  [  U  ]  [  I  ]  [  O  ]  [  P  ]  [BSPC ] 
 [LSFT ]  [  A  ]  [  S  ]  [  D  ]  [  F  ]  [  G  ]        [  H  ]  [  J  ]  [  K  ]  [  L  ]  [SCLN ]  [QUOT ] 
 [LCTL ]  [  Z  ]  [  X  ]  [  C  ]  [  V  ]  [  B  ]        [  N  ]  [  M  ]  [COMM ]  [ DOT ]  [SLSH ]  [RSFT ] 
 [     ]  [     ]  [     ]  [LGUI ]  [MO13 ]  [ SPC ]        [ ENT ]  [MO23 ]  [RALT ]  [     ]  [     ]  [     ] 

Added 1 column(s) to sides:
Keyboard: Crkbd. Layer: 0
 [     ]  [     ]  [     ]  [     ]  [     ]  [     ]  [     ]        [     ]  [     ]  [     ]  [     ]  [     ]  [     ]  [     ] 
 [     ]  [     ]  [     ]  [     ]  [     ]  [     ]  [     ]        [     ]  [     ]  [     ]  [     ]  [     ]  [     ]  [     ] 
 [     ]  [ ESC ]  [  Q  ]  [  W  ]  [  E  ]  [  R  ]  [  T  ]        [  Y  ]  [  U  ]  [  I  ]  [  O  ]  [  P  ]  [BSPC ]  [     ] 
 [     ]  [LSFT ]  [  A  ]  [  S  ]  [  D  ]  [  F  ]  [  G  ]        [  H  ]  [  J  ]  [  K  ]  [  L  ]  [SCLN ]  [QUOT ]  [     ] 
 [     ]  [LCTL ]  [  Z  ]  [  X  ]  [  C  ]  [  V  ]  [  B  ]        [  N  ]  [  M  ]  [COMM ]  [ DOT ]  [SLSH ]  [RSFT ]  [     ] 
 [     ]  [     ]  [     ]  [     ]  [LGUI ]  [MO13 ]  [ SPC ]        [ ENT ]  [MO23 ]  [RALT ]  [     ]  [     ]  [     ]  [     ] 
```
