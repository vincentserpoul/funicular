use anyhow::Result;
use funicular::commands::FunicularOpts;
use gumdrop::Options;

pub fn main() -> Result<()> {
    let opts: FunicularOpts = FunicularOpts::parse_args_default_or_exit();

    match opts.run() {
        Err(e) => println!("{:?}", e),
        Ok(()) => println!(),
    }

    Ok(())
}
