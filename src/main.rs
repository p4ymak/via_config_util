mod structs;
use clap::Parser;
use std::{
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
    result::Result,
};
use structs::{print_layer, Config};
#[derive(Parser, Debug)]
#[clap(about = "Tool to check and mirror keymaps in VIA config for Split Keyboard.")]
struct Args {
    /// Input: Path to VIA JSON config.
    #[clap(short = 'i', long = "input")]
    input: PathBuf,
    #[clap(short = 'o', long = "output")]
    output: Option<PathBuf>,

    #[clap(short = 'w', long = "width")]
    width: usize,
    #[clap(short = 'h', long = "height")]
    height: usize,

    #[clap(short = 'm', long = "mirror")]
    mirror: bool,

    #[clap(short = 'v', long = "verbose")]
    verbose: bool,

    #[clap(long = "add_cols_center")]
    add_cols_center: Option<u8>,
    #[clap(long = "add_cols_sides")]
    add_cols_sides: Option<u8>,
    #[clap(long = "add_rows_top")]
    add_rows_top: Option<u8>,
    #[clap(long = "add_rows_bottom")]
    add_rows_bottom: Option<u8>,

    #[clap(long = "rm_cols_center")]
    rm_cols_center: Option<u8>,
    #[clap(long = "rm_cols_sides")]
    rm_cols_sides: Option<u8>,
    #[clap(long = "rm_rows_top")]
    rm_rows_top: Option<u8>,
    #[clap(long = "rm_rows_bottom")]
    rm_rows_bottom: Option<u8>,
}

fn main() {
    let args = Args::parse();
    let path = &args.input;
    let content = fs::read_to_string(path).expect("Can't read file");
    let config: Result<Config, serde_json::Error> = serde_json::from_str(&content);
    if let Ok(config) = config {
        let parts = config.split_map(args.width, args.height);
        if let Some([mut left, mut right]) = parts {
            if args.verbose {
                for i in 0..left.layers() {
                    print_layer(&left, &right, i);
                }
            }
            if let Some(rows) = args.rm_rows_top {
                left.change_rows_top(-(rows as i8));
                right.change_rows_top(-(rows as i8));
                if args.verbose {
                    println!("Removed {rows} row(s) from top:");
                    print_layer(&left, &right, 0);
                }
            }
            if let Some(rows) = args.rm_rows_bottom {
                left.change_rows_bottom(-(rows as i8));
                right.change_rows_bottom(-(rows as i8));
                if args.verbose {
                    println!("Removed {rows} row(s) from bottom:");
                    print_layer(&left, &right, 0);
                }
            }
            if let Some(rows) = args.add_rows_top {
                left.change_rows_top(rows as i8);
                right.change_rows_top(rows as i8);
                if args.verbose {
                    println!("Added {rows} row(s) to top:");
                    print_layer(&left, &right, 0);
                }
            }
            if let Some(rows) = args.add_rows_bottom {
                left.change_rows_bottom(rows as i8);
                right.change_rows_bottom(rows as i8);
                if args.verbose {
                    println!("Added {rows} row(s) to bottom:");
                    print_layer(&left, &right, 0);
                }
            }

            if let Some(cols) = args.rm_cols_center {
                left.change_cols_center(-(cols as i8));
                right.change_cols_center(-(cols as i8));
                if args.verbose {
                    println!("Removed {cols} columns(s) from center:");
                    print_layer(&left, &right, 0);
                }
            }
            if let Some(cols) = args.rm_cols_sides {
                left.change_cols_sides(-(cols as i8));
                right.change_cols_sides(-(cols as i8));
                if args.verbose {
                    println!("Removed {cols} column(s) from sides:");
                    print_layer(&left, &right, 0);
                }
            }

            if let Some(cols) = args.add_cols_center {
                left.change_cols_center(cols as i8);
                right.change_cols_center(cols as i8);
                if args.verbose {
                    println!("Added {cols} column(s) to center:");
                    print_layer(&left, &right, 0);
                }
            }
            if let Some(cols) = args.add_cols_sides {
                left.change_cols_sides(cols as i8);
                right.change_cols_sides(cols as i8);
                if args.verbose {
                    println!("Added {cols} column(s) to sides:");
                    print_layer(&left, &right, 0);
                }
            }

            if args.mirror {
                let left_ori = left.clone();
                left = right.to_mirrored();
                right = left_ori.to_mirrored();
                if args.verbose {
                    println!("\nMirrored Layout:");
                    print_layer(&left, &right, 0);
                }
            }

            if let Some(output) = args.output {
                let config = Config::join_maps(&left, &right);
                let json = serde_json::to_string_pretty(&config);
                if let Ok(json) = json {
                    match save_json(json, &output) {
                        Ok(path) => println!("New config saved: {}", path),
                        Err(_) => println!("Couldn't save config.. =("),
                    }
                } else {
                    println!("Couldn't serialize new config.. =(")
                }
            }
        } else {
            println!("Couldn't split keymap in two halfs.. =(")
        }
    } else {
        println!("Couldn't read VIA JSON config.. =(")
    }
}

fn save_json(json: String, path: &Path) -> Result<String, Box<dyn Error + 'static>> {
    let output_path_string = path.to_string_lossy().to_string();

    let mut output_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    write!(output_file, "{}", json)?;

    Ok(output_path_string)
}
