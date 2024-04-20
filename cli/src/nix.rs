use crate::{
    cli::GlobalOptions, customize::NoCustomizeMode, error::CliResult, input::InputOptions,
};
use nickel_lang_core::cache::Cache;
use nickel_lang_core::cache::ErrorTolerance;
use nickel_lang_core::nix::parse;

#[derive(clap::Parser, Debug)]
pub struct NixinCommand {
    #[command(flatten)]
    pub input: InputOptions<NoCustomizeMode>,
}

impl NixinCommand {
    pub fn run(self, _global: GlobalOptions) -> CliResult<()> {
        let mut cache = Cache::new(ErrorTolerance::Strict);
        for file in self.input.files {
            let file_id = cache.add_file(file)?;
            let rt = parse(&cache, file_id)?;
            println!("{}", rt);
        }
        Ok(())
    }
}
