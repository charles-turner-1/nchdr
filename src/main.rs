mod dims;
mod vars;

use colored::Colorize;
use dims::get_dim_info;
use netcdf::{self};
use std::path::Path;
use vars::get_var_info;

trait HasName {
    fn name(&self) -> &str;
}

impl<'a> HasName for netcdf::Attribute<'a> {
    fn name(&self) -> &str {
        self.name()
    }
}

fn main() -> Result<(), netcdf::Error> {
    let path =
        Path::new("/Users/u1166368/Rust/nchdr/tests/data/access-om2/output001/ocean/ocean.nc");

    let file: netcdf::File = netcdf::open(path)?;

    let file_stem = extract_fname(&path)?;
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

    println!("{}", "// global attributes:".cyan().bold());
    print_attrs(global_attrs);
    print_skeleton_close();

    Ok(())
}

fn print_attrs<T>(attrs: Vec<T>)
where
    T: HasName,
{
    for attr in attrs {
        println!("\t{} ;", attr.name())
    }
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
