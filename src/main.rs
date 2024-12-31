mod dims;

use netcdf::{self};
use std::path::Path;
use colored::Colorize;
use dims::get_dim_info;

trait HasName {
    fn name(&self) -> &str;
}

impl<'a> HasName for netcdf::Attribute<'a> {
    fn name(&self) -> &str {
        self.name()
    }
}



struct VarWrapper<'a> {
    _var : netcdf::Variable<'a>,
    name : String,
}

impl<'a> VarWrapper<'a> {
    fn new(_var : netcdf::Variable<'a>) -> Self {
        let name = _var.name().clone();
        VarWrapper{ _var, name}
    }
}

impl<'a> HasName for VarWrapper<'a> {
    fn name(&self) -> &str {
        &self.name
    }
}





fn main() -> Result<(), netcdf::Error> {

    let path = Path::new("/Users/u1166368/Rust/nchdr/tests/data/access-om2/output001/ocean/ocean.nc");

    let file: netcdf::File = netcdf::open(path)?;

    let file_stem = extract_fname(&path)?;
    print_skeleton_opening(&file_stem);

    let variables: Vec<_> = file.variables().collect();
    let dimensions: Vec<_> = file.dimensions().collect();
    let global_attrs: Vec<_> = file.attributes().collect();

    println!("{}","dimensions:".green());
    for dim in dimensions {
        get_dim_info(&file, &dim.name());
    }

    let wrapped_vars = variables.into_iter().map(VarWrapper::new).collect();
    println!("{}","variables:".green());
    print_attrs(wrapped_vars);


    println!("{}","// global attributes:".green());
    print_attrs(global_attrs);
    print_skeleton_close();
    
    Ok(())
}


fn print_attrs<T>(attrs : Vec<T>) where T: HasName {
    for attr in attrs {
        println!("\t{} ;",  attr.name())
    }
}

fn print_skeleton_opening( f_stem : &str){
    println!("netcdf {} {{ ", f_stem.cyan().bold());
}

fn print_skeleton_close(){
    println!("}}\n")
}

fn extract_fname<'a>(f_handle: &'a std::path::Path) -> Result<&str, &str>{
    // Take the file handle, get the file stem, extract it.

    match f_handle.file_stem() {
        Some(stem) => match stem.to_str() {
            Some(stem_str) => Ok(stem_str),
            None => Err("Could not convert file stem to valid utf-8 string"),
        }
        None => Err("Could not identify filename"),
    }

}
