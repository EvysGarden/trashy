mod list;
pub mod put;
mod empty;
mod restore;

use structopt::StructOpt;
use eyre::{WrapErr, Result};

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    List(list::Opt),
    Put(put::Opt),
    Empty(empty::Opt),
    Restore(restore::Opt),
}

impl SubCommand {
    pub fn run(self) -> Result<()> {
        match self {
            SubCommand::List(opt) => list::list(opt)?,
            SubCommand::Put(opt) => {
                let _ = put::put(opt)?;
            },
            SubCommand::Empty(opt) => empty::empty(opt)?,
            SubCommand::Restore(opt) => restore::restore(opt)?,
        }
        Ok(())
    }
}

// pub trait OptionalSubcommand {
//     fn run_or_default(self) -> Result<()>;
// }

// impl OptionalSubcommand for Option<SubCommand> {
//     fn run_or_default(self) -> Result<()> {
//         match self {
//             Some(subcmd) => subcmd.run()?,
//             None => {
//                 let _ = put::put(opt)?;
//             },
//         }
//         Ok(())
//     }
// }
