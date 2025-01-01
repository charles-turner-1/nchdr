use colored::Colorize;
use netcdf::{self};

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
            netcdf::AttributeValue::Double(db) => {
                if db.abs() < 1000.0 {
                    format!("{}", db)
                } else {
                    format!("{:.1e}", db)
                }
            }
            netcdf::AttributeValue::Float(f) => {
                if f.abs() < 1_000.0 {
                    format!("{}", f)
                } else {
                    format!("{:.1e}", f)
                }
            }
            netcdf::AttributeValue::Int(i) => {
                if i.abs() < 1_000 {
                    format!("{}", i)
                } else {
                    format!("{:.1e}", i as f64)
                }
            }
            netcdf::AttributeValue::Longlong(l) => {
                if l.abs() < 1_000 {
                    format!("{}", l)
                } else {
                    format!("{:.1e}", l as f64)
                }
            }
            netcdf::AttributeValue::Schar(s) => format!("{}", s),
            netcdf::AttributeValue::Short(sh) => format!("{}", sh),
            netcdf::AttributeValue::Str(s) => format!("\"{}\"", s),
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
                .map(|f| format!("{}.f", f))
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
