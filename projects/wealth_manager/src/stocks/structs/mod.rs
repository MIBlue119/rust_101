//Where the mod.rs files come in. mode.rs files are essentially __init__.py files in Python
pub mod stock;
pub mod order;
//We are not make it pubic, we only allow files in the directory `src/stocks/structs/` to access it.
mod utils;