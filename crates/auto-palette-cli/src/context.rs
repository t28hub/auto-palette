use crate::{args::Options, env::Env};

/// The context for the command line application.
#[derive(Debug, PartialEq, Eq)]
pub struct Context {
    args: Options,
    env: Env,
}

impl Context {
    /// Creates a new `Context` instance with the given arguments.
    ///
    /// # Arguments
    /// * `args` - The command line arguments.
    /// * `env` - The environment variables.
    ///
    /// # Returns
    /// A new `Context` instance.
    #[must_use]
    pub fn new(args: Options, env: Env) -> Self {
        Self { args, env }
    }

    /// Returns the command line arguments.
    ///
    /// # Returns
    /// The command line arguments.
    #[must_use]
    pub fn args(&self) -> &Options {
        &self.args
    }

    /// Returns the environment variables.
    ///
    /// # Returns
    /// The environment variables.
    #[must_use]
    pub fn env(&self) -> &Env {
        &self.env
    }
}
