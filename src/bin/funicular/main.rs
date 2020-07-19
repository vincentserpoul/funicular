use funicular::commands::FunicularOpts;
use gumdrop::Options;

fn main() {
    let opts = FunicularOpts::parse_args_default_or_exit();

    println!("{:#?}", opts);
}
