use color_eyre::eyre::Result;
use funicular::commands::FunicularOpts;
use gumdrop::Options;

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let opts: FunicularOpts = FunicularOpts::parse_args_default_or_exit();

    match opts.run() {
        Err(e) => println!("{:?}", e),
        Ok(()) => println!(),
    }

    Ok(())
}
