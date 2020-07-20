use funicular::commands::FunicularOpts;
use gumdrop::Options;

fn main() {
    let opts: FunicularOpts = FunicularOpts::parse_args_default_or_exit();

    match opts.run() {
        Err(e) => println!("{:?}", e),
        Ok(()) => println!(),
    }
}
