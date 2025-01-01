mod attrs;
mod dims;
mod vars;

use attrs::get_attr_info;
use colored::Colorize;
use dims::get_dim_info;
use netcdf::{self};
use std::path::Path;
use vars::get_var_info;
use std::env;
use std::error::Error;


fn main() -> Result<(), Box< dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        return Err("Did not provide valid filename".into());
    } else if args.len() > 2 {
        return Err("Must provide single filename".into());
    }

    let f_path = Path::new(args.last().unwrap());

    let file: netcdf::File = netcdf::open(f_path)?;

    let file_stem = extract_fname(&f_path)?;
    print_skeleton_opening(&file_stem);

    let variables: Vec<_> = file.variables().collect();
    let dimensions: Vec<_> = file.dimensions().collect();
    let global_attrs: Vec<_> = file.attributes().collect();

    println!("{}", "dimensions:".blue().bold());
    for dim in dimensions {
        get_dim_info(&file, &dim.name());
    }

    println!("{}", "variables:".green().bold());
    for var in variables {
        get_var_info(&file, &var.name());
    }

    println!("{}", "\n// global attributes:".cyan().bold());
    for attr in global_attrs {
        get_attr_info(&file, attr.name())
    }
    print_skeleton_close();

    Ok(())
}

fn print_skeleton_opening(f_stem: &str) {
    println!("netcdf {} {{ ", f_stem.bold());
}

fn print_skeleton_close() {
    println!("}}\n")
}

fn extract_fname<'a>(f_handle: &'a std::path::Path) -> Result<&str, &str> {
    // Take the file handle, get the file stem, extract it.

    match f_handle.file_stem() {
        Some(stem) => match stem.to_str() {
            Some(stem_str) => Ok(stem_str),
            None => Err("Could not convert file stem to valid utf-8 string"),
        },
        None => Err("Could not identify filename"),
    }
}
