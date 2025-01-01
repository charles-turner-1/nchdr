use colored::Colorize;
use netcdf::{self, types::NcVariableType};

pub fn get_var_info(file: &netcdf::File, var_name: &String) {
    // Take reference to a file and a variable in it, and get all the attributes
    // and their values

    let var_opt = file.variables().find(|var| var.name() == *var_name);

    // Nasty - get rid of unwrap if possible.
    let var = var_opt.unwrap();

    let var_attrs: Vec<netcdf::Attribute> = var.attributes().collect();
    let var_dtype = var.vartype();
    let var_dims: &[netcdf::Dimension<'_>] = var.dimensions();

    let var_dimstr = format_dimensions(var_dims);
    let var_typestr = format_type(&var_dtype);

    // let var_fstr = format_var_info(var_info);

    println!(
        "\t{} {}({}) ;",
        var_typestr,
        var.name().green(),
        var_dimstr.blue()
    );

    // let attr_str = format_attr(&var_attrs[0]);
    // println!("{}\n", attr_str);

    let attr_str = format_attrs(&var.name(), var_attrs);
    println!("{}", attr_str);
}

fn format_dimensions(dims: &[netcdf::Dimension<'_>]) -> String {
    dims.iter()
        .map(|dim| format!("{}", dim.name()))
        .collect::<Vec<String>>()
        .join(", ")
}

fn format_type(nc_type: &NcVariableType) -> String {
    match nc_type {
        NcVariableType::Compound(_) => "compound".to_string(),
        NcVariableType::Opaque(_) => "opaque".to_string(),
        NcVariableType::Enum(_) => "enum".to_string(),
        NcVariableType::Vlen(_) => "vlen".to_string(),
        NcVariableType::Int(_) => "int".to_string(), // Might want to expand this?
        NcVariableType::Float(float_type) => match float_type {
            netcdf::types::FloatType::F32 => "float".to_string(),
            netcdf::types::FloatType::F64 => "double".to_string(),
        },
        NcVariableType::Char => "char".to_string(),
        NcVariableType::String => "string".to_string(),
    }
}

fn format_attr(attr: &netcdf::Attribute) -> String {
    // Get a single attributes out, in the format:
    // $VARNAME:$ATTR_NAME = "$ATTR_VALUE" ;

    let attr_value = attr.value();

    // Gotta be a smarter way to do this?
    match attr_value {
        Ok(value) => match value {
            netcdf::AttributeValue::Uchar(ch) => format!("{}", ch),
            netcdf::AttributeValue::Double(db) => format!("{}", db),
            netcdf::AttributeValue::Float(f) => format!("{}", f),
            netcdf::AttributeValue::Int(i) => format!("{}", i),
            netcdf::AttributeValue::Longlong(l) => format!("{}", l),
            netcdf::AttributeValue::Schar(s) => format!("{}", s),
            netcdf::AttributeValue::Short(sh) => format!("{}", sh),
            netcdf::AttributeValue::Str(s) => s,
            netcdf::AttributeValue::Uint(u) => format!("{}", u),
            netcdf::AttributeValue::Ulonglong(ull) => format!("{}", ull),
            netcdf::AttributeValue::Ushort(ush) => format!("{}", ush),
            netcdf::AttributeValue::Doubles(dbs) => dbs
                .iter()
                .map(|db| format!("{}", db))
                .collect::<Vec<String>>()
                .join(", "),
            netcdf::AttributeValue::Floats(fs) => fs
                .iter()
                .map(|f| format!("{}", f))
                .collect::<Vec<String>>()
                .join(", "),
            netcdf::AttributeValue::Ints(is) => is
                .iter()
                .map(|i| format!("{}", i))
                .collect::<Vec<String>>()
                .join(", "),
            netcdf::AttributeValue::Longlongs(lls) => lls
                .iter()
                .map(|ll| format!("{}", ll))
                .collect::<Vec<String>>()
                .join(", "),
            netcdf::AttributeValue::Schars(schs) => schs
                .iter()
                .map(|sch| format!("{}", sch))
                .collect::<Vec<String>>()
                .join(", "),
            netcdf::AttributeValue::Shorts(shts) => shts
                .iter()
                .map(|sht| format!("{}", sht))
                .collect::<Vec<String>>()
                .join(", "),
            netcdf::AttributeValue::Strs(ss) => ss
                .iter()
                .map(|s| format!("{}", s))
                .collect::<Vec<String>>()
                .join(", "),
            netcdf::AttributeValue::Uchars(uchs) => uchs
                .iter()
                .map(|uch| format!("{}", uch))
                .collect::<Vec<String>>()
                .join(", "),
            netcdf::AttributeValue::Uints(uis) => uis
                .iter()
                .map(|ui| format!("{}", ui))
                .collect::<Vec<String>>()
                .join(", "),
            netcdf::AttributeValue::Ulonglongs(ulls) => ulls
                .iter()
                .map(|ull| format!("{}", ull))
                .collect::<Vec<String>>()
                .join(", "),
            netcdf::AttributeValue::Ushorts(ushts) => ushts
                .iter()
                .map(|usht| format!("{}", usht))
                .collect::<Vec<String>>()
                .join(", "),
        },
        Err(_) => "unknown".to_string(),
    }
}

fn format_attrs(varname: &str, attrs: Vec<netcdf::Attribute>) -> String {
    // Like above, except we want to get all attributes.
    attrs
        .iter()
        .map(|attr| {
            format!(
                "\t\t{}:{} = \"{}\" ;",
                varname.green().dimmed(),
                attr.name(),
                format_attr(attr).cyan()
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}
