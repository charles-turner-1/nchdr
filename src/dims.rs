use colored::Colorize;

pub fn get_dim_info(file: &netcdf::File, dim_name : &String ) {
    // Take references to a file and a dimension in it, and print their size.

    let dim_opt = file.dimensions().find(|dim| dim.name() == *dim_name);

    // Nasty - get rid of unwrap if possible.
    let dim = dim_opt.unwrap();

    let is_dim_unlimited = dim.is_unlimited();
    let dim_len = dim.len();

    let dim_info = DimInfo::new(dim_name.clone(), is_dim_unlimited, dim_len);

    let dim_fstr = format_dim_info(dim_info);

    println!("{}",dim_fstr)

}

struct DimInfo {
    name : String,
    is_unlimited : bool,
    len : usize,
}

impl DimInfo {
    fn new(name : String, is_unlimited : bool, len : usize) -> Self{
        DimInfo{name, is_unlimited, len}
    }
}

fn format_dim_info(dim_info : DimInfo) -> String{
    // Return info about a dimension
    if dim_info.is_unlimited {
        format!("\t{} = {} ; // ({} currently) ", dim_info.name.blue().bold(), "UNLIMITED".yellow().bold(),  dim_info.len)
    } else {
        format!("\t{} = {} ;", dim_info.name.blue().bold(), dim_info.len)
    }

}