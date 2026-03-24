use clap::Args;

#[derive(Clone, Debug, Args)]
pub struct ProfileOptions {
    #[clap(long, short)]
    #[clap(help = "profile name")]
    pub profile_name: String,
}
