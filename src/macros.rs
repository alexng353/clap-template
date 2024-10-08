#[macro_export]
macro_rules! commands_enum {
    ($($module:ident),*) => (
      paste::paste! {
        #[derive(Subcommand)]
        enum Commands {
            $(
              [<$module:camel>]($module::Args),
            )*
        }

        impl Commands {
            async fn exec(cli: Args) -> anyhow::Result<()> {
              match cli.command {
                $(
                  Commands::[<$module:camel>](args) => $module::command(args).await?,
                )*
              }
              Ok(())
            }
        }
      }
    );
}

/// Ensure running in a terminal or bail with the provided message
#[macro_export]
macro_rules! interact_or {
    ($message:expr) => {
        if !std::io::stdout().is_terminal() {
            use anyhow::bail;
            bail!($message);
        }
    };
}
