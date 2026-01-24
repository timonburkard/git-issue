#![deny(warnings, clippy::unwrap_used, clippy::expect_used)]

pub mod cmd;
pub mod model;

pub use crate::cmd::edit::edit_end;
pub use crate::cmd::edit::edit_start;
pub use crate::cmd::init::init;
pub use crate::cmd::link::link;
pub use crate::cmd::list;
pub use crate::cmd::list::list;
pub use crate::cmd::new::new;
pub use crate::cmd::set::set;
pub use crate::cmd::show::show;

pub struct CmdResult<T> {
    pub value: T,
    pub infos: Vec<String>,
}

pub type Cmd<T> = Result<CmdResult<T>, String>;
