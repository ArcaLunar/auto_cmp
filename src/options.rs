use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CmdOptions {
    #[structopt(long="ignore_ac")]
    _ignore_ac_cases: bool,
    // todo
}