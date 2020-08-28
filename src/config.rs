use argh::FromArgs;

#[derive(Debug, PartialEq, FromArgs, Getters)]
/// Retry command execution until successful.
pub struct Config {
    /// command and options
    #[argh(positional)]
    pub command: String,

    /// maximum number of executions
    #[argh(option, short = 'c')]
    pub count: Option<usize>,

    /// execution interval (sec)
    #[argh(option, short = 'i', default = "0.0")]
    pub interval: f64,
}
