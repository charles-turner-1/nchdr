use colored::Colorize;
use netcdf::{self};
use num::{self, Signed, ToPrimitive};

pub fn get_attr_info(file: &netcdf::File, attr_name: &str) {
    // Take reference to a file and a variable in it, and get all the attributes
    // and their values

    let attr_opt = file.attributes().find(|attr| attr.name() == attr_name);

    // Nasty - get rid of unwrap if possible.
    let attr = attr_opt.unwrap();

    println!("\t\t:{} = {} ;", attr.name().cyan(), format_attr(&attr),);
}

pub fn format_attr(attr: &netcdf::Attribute) -> String {
    // Get a single attributes out, in the format:
    // $VARNAME:$ATTR_NAME = "$ATTR_VALUE" ;

    let attr_value = attr.value();

    // Gotta be a smarter way to do this?
    match attr_value {
        Ok(value) => match value {
            netcdf::AttributeValue::Uchar(ch) => format!("{}", ch),
            netcdf::AttributeValue::Schar(item) => format!("{}", item),
            netcdf::AttributeValue::Short(sh) => format!("{}", sh),
            netcdf::AttributeValue::Str(s) => format!("\"{}\"", s),
            netcdf::AttributeValue::Uint(u) => format!("{}", u),
            netcdf::AttributeValue::Ulonglong(ull) => format!("{}", ull),
            netcdf::AttributeValue::Ushort(ush) => format!("{}", ush),
            netcdf::AttributeValue::Double(item) => format_val(item),
            netcdf::AttributeValue::Float(item) => format_val(item),
            netcdf::AttributeValue::Int(item) => format_val(item),
            netcdf::AttributeValue::Longlong(item) => format_val(item),
            netcdf::AttributeValue::Doubles(items) => format_values(items),
            netcdf::AttributeValue::Floats(items) => format_values(items),
            netcdf::AttributeValue::Ints(items) => format_values(items),
            netcdf::AttributeValue::Longlongs(items) => format_values(items),
            netcdf::AttributeValue::Schars(items) => format_values(items),
            netcdf::AttributeValue::Shorts(items) => format_values(items),
            netcdf::AttributeValue::Strs(items) => format_values(items),
            netcdf::AttributeValue::Uchars(items) => format_values(items),
            netcdf::AttributeValue::Uints(items) => format_values(items),
            netcdf::AttributeValue::Ulonglongs(items) => format_values(items),
            netcdf::AttributeValue::Ushorts(items) => format_values(items),
        },
        Err(_) => "unknown".to_string(),
    }
}

fn format_val<T>(val: T) -> String
where
    T: Signed + ToPrimitive + std::fmt::Display + std::cmp::PartialOrd,
{
    // Would be nice to get rid of the unwraps, don't know how to do that well.
    if (val.to_f64().unwrap()).abs() < 1_000 as f64 {
        format!("{}", val)
    } else {
        format!("{:.1e}", val.to_f64().unwrap())
    }
}

fn format_values<T: std::fmt::Display>(vals: Vec<T>) -> String {
    vals.iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join(", ")
}
