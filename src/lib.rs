mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Format, Base64SubCommand, HttpSubCommand, Opts, OutputFormat, SubCommand, TextSignFormat,
    TextSubCommand,
};
pub use process::{
    process_csv, process_decode, process_encode, process_gen_pass, process_generate,
    process_http_serve, process_text_sign, process_text_verify,
};
pub use utils::get_reader;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(&self) -> anyhow::Result<()>;
}
