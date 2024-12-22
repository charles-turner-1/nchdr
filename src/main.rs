use netcdf::{self};

trait HasName {
    fn name(&self) -> &str;
}

impl<'a> HasName for netcdf::Attribute<'a> {
    fn name(&self) -> &str {
        self.name()
    }
}

struct DimWrapper<'a> {
    _dim: netcdf::Dimension<'a>,
    name : String,
}

impl<'a> DimWrapper<'a> {
    fn new(_dim: netcdf::Dimension<'a>) -> Self {
        let name = _dim.name().clone();
        DimWrapper { _dim, name}
    }
}

impl<'a> HasName for DimWrapper<'a> {
    fn name(&self) -> &str {
        &self.name
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
    let file = netcdf::open(
        "/Users/u1166368/Rust/nchdr/tests/data/access-om2/output001/ocean/ocean.nc"
    )?;
    let variables: Vec<_> = file.variables().collect();
    let dimensions: Vec<_> = file.dimensions().collect();
    let global_attrs: Vec<_> = file.attributes().collect();

    let wrapped_vars = variables.into_iter().map(VarWrapper::new).collect();
    print_attrs(wrapped_vars);

    let wrapped_dims = dimensions.into_iter().map(DimWrapper::new).collect();
    print_attrs(wrapped_dims);

    print_attrs(global_attrs);
    
    Ok(())
}


fn print_attrs<T>(attrs : Vec<T>) where T: HasName {
    println!("\n");
    for attr in attrs {
        println!("Attributes: {}", attr.name())
    }
}