use color_eyre::Result;
use engine::uci;
use engine::util::log::log;

fn main() -> Result<()> {
    color_eyre::install()?;

    std::panic::set_hook(Box::new(|info| {
        println!("{info}");
        log(format!("{info:?}"));
    }));

    engine::init();
    uci::uci()
}
