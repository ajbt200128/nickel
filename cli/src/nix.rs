use crate::{
    cli::GlobalOptions, customize::NoCustomizeMode, error::CliResult, input::InputOptions,
};

#[derive(clap::Parser, Debug)]
pub struct NixinCommand {
    #[command(flatten)]
    pub input: InputOptions<NoCustomizeMode>,
}

impl NixinCommand {
    pub fn run(self, _global: GlobalOptions) -> CliResult<()> {
        unimplemented!()
    }
}
