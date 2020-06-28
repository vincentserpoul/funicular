use funicular::commands::FunicularOptions;
use gumdrop::Options;

fn main() {
    let opts = FunicularOptions::parse_args_default_or_exit();

    println!("{:#?}", opts);
}
