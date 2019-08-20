use ocl::{self, Platform, Device};


fn print_info() -> ocl::Result<()> {
    println!("Available platforms:\n");
    for p in Platform::list() {
        println!("Name:    {}", p.name()?);
        println!("Version: {}", p.version()?);
        println!("Vendor:  {}", p.vendor()?);
        println!("Devices:");
        for d in Device::list_all(&p)? {
            println!("    {}", d.name()?);
        }
        println!("");
    }
    Ok(())
}

fn main() {
    match print_info() {
        Ok(()) => (),
        Err(err) => panic!("{}", err),
    }
}
