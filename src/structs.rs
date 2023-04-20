use serde::{Deserialize, Serialize};
use std::fmt;
const KEYSIZE: usize = 5;

#[derive(Debug, Clone)]
struct Key(String);
impl Key {
    pub fn new(k: &str) -> Self {
        Key(k.to_string())
    }
    pub fn k(&self) -> String {
        self.0.to_owned()
    }
    const NO: Self = Key("KC_NO".to_string());
}
impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let k = match self.0.as_str() {
            "KC_NO" => " ".repeat(KEYSIZE),
            _ => self
                .0
                .trim_start_matches("S(")
                .trim_start_matches("KC_")
                .trim_start_matches("FN_")
                .trim_end_matches(')')
                .chars()
                .take(KEYSIZE)
                .collect::<String>(),
        };
        write!(f, "[{:^5}]", k)
    }
}

type Layer = Vec<Vec<Key>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}
impl Side {
    pub fn opposite(&self) -> Self {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyMatrix {
    name: String,
    vendor_id: usize,
    macros: Vec<String>,
    side: Side,
    keymap: Vec<Layer>,
}

impl KeyMatrix {
    pub fn width(&self) -> usize {
        self.keymap
            .first()
            .expect("At least one Layer exists.")
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or_default()
    }
    pub fn change_cols_center(&mut self, cols: i8) {
        match self.side {
            Side::Left => self.change_cols(cols, Side::Left),
            Side::Right => self.change_cols(cols, Side::Left),
        }
    }
    pub fn change_cols_sides(&mut self, cols: i8) {
        match self.side {
            Side::Left => self.change_cols(cols, Side::Right),
            Side::Right => self.change_cols(cols, Side::Right),
        }
    }

    pub fn change_rows_top(&mut self, rows: i8) {
        let width = self.width();
        if rows >= 0 {
            for layer in self.keymap.iter_mut() {
                layer.reverse();
                for _ in 0..(rows as usize) {
                    layer.push(vec![Key::NO; width]);
                }
                layer.reverse();
            }
        } else {
            for layer in self.keymap.iter_mut() {
                layer.reverse();
                for _ in 0..(rows as usize) {
                    layer.pop();
                }
                layer.reverse();
            }
        }
    }
    pub fn change_rows_bottom(&mut self, rows: i8) {
        let width = self.width();
        if rows >= 0 {
            for layer in self.keymap.iter_mut() {
                for _ in 0..(rows as usize) {
                    layer.push(vec![Key::NO; width]);
                }
            }
        } else {
            for layer in self.keymap.iter_mut() {
                for _ in 0..(rows as usize) {
                    layer.pop();
                }
            }
        }
    }

    fn change_cols(&mut self, cols: i8, side: Side) {
        for layer in self.keymap.iter_mut() {
            for row in layer.iter_mut() {
                if cols >= 0 {
                    if self.side == side {
                        row.append(&mut vec![Key::NO; cols as usize])
                    } else {
                        let mut new_row = vec![Key::NO; cols as usize];
                        new_row.append(row);
                        *row = new_row;
                    }
                } else {
                    for _ in 0..cols.abs() {
                        match self.side {
                            Side::Left => _ = row.pop(),
                            Side::Right => {
                                row.reverse();
                                _ = row.pop();
                                row.reverse();
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn to_mirrored(&self) -> Self {
        KeyMatrix {
            side: self.side.opposite(),
            keymap: self
                .keymap
                .iter()
                .map(|layer| {
                    layer
                        .iter()
                        .map(|row| {
                            let mut m = row.clone();
                            m.reverse();
                            m
                        })
                        .collect::<Layer>()
                })
                .collect(),
            ..{ self.clone() }
        }
    }
    pub fn layers(&self) -> usize {
        self.keymap.len()
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub vendorProductId: usize,
    pub macros: Vec<String>,
    pub layers: Vec<Vec<String>>,
}
impl Config {
    pub fn split_map(&self, w: usize, h: usize) -> Option<[KeyMatrix; 2]> {
        if self.layers.iter().any(|l| l.len() != w * h * 2) {
            return None;
        }
        let mut left_layers = Vec::<Layer>::new();
        let mut right_layers = Vec::<Layer>::new();

        for layer in &self.layers {
            let mut left_keymap = Layer::new();
            let mut right_keymap = Layer::new();
            for y in 0..h {
                left_keymap.push(
                    layer[y * w..(y + 1) * w]
                        .iter()
                        .map(|k| Key::new(k))
                        .collect(),
                );
                right_keymap.push(
                    layer[(h + y) * w..(y + 1 + h) * w]
                        .iter()
                        .map(|k| Key::new(k))
                        .rev()
                        .collect(),
                );
            }

            left_layers.push(left_keymap);
            right_layers.push(right_keymap);
        }
        let left = KeyMatrix {
            name: self.name.to_owned(),
            vendor_id: self.vendorProductId,
            macros: self.macros.to_owned(),
            side: Side::Left,
            keymap: left_layers,
        };
        let right = KeyMatrix {
            name: self.name.to_owned(),
            vendor_id: self.vendorProductId,
            macros: self.macros.to_owned(),
            side: Side::Right,
            keymap: right_layers,
        };
        Some([left, right])
    }

    pub fn join_maps(left: &KeyMatrix, right: &KeyMatrix) -> Self {
        let layers = (0..left.keymap.len())
            .map(|i| {
                left.keymap
                    .get(i)
                    .unwrap()
                    .iter()
                    .flatten()
                    .collect::<Vec<&Key>>()
                    .iter()
                    .map(|key| key.k())
                    .chain(
                        right
                            .keymap
                            .get(i)
                            .unwrap()
                            .iter()
                            .flat_map(|row| row.iter().rev().collect::<Vec<&Key>>())
                            .collect::<Vec<&Key>>()
                            .iter()
                            .map(|key| key.k()),
                    )
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        Config {
            name: left.name.to_owned(),
            vendorProductId: left.vendor_id,
            macros: left.macros.to_owned(),
            layers,
        }
    }
}

fn layer_string(mat: &KeyMatrix, layer: usize) -> String {
    let mut matrix = String::new();
    for row in mat.keymap.get(layer).expect("Layer exists").iter() {
        match mat.side {
            Side::Left => {
                matrix.push_str(&row.iter().map(|k| format!(" {} ", k)).collect::<String>())
            }
            Side::Right => {
                matrix.push_str(&row.iter().map(|k| format!(" {} ", k)).collect::<String>())
            }
        };
        matrix.push('\n');
    }
    matrix
}

fn join_layer_strings(left: String, right: String, space: usize) -> String {
    let rows_l = left.lines();
    let rows_r = right.lines();

    rows_l
        .zip(rows_r)
        .map(|(l, r)| format!("{l}{}{r}\n", " ".repeat(space)))
        .collect::<String>()
}

pub fn print_layer(left: &KeyMatrix, right: &KeyMatrix, layer: usize) {
    println!("Keyboard: {}. Layer: {}", left.name, layer);
    println!(
        "{}",
        join_layer_strings(layer_string(left, layer), layer_string(right, layer), 6)
    );
}
