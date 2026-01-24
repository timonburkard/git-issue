pub mod edit;
pub mod init;
pub mod link;
pub mod list;
pub mod new;
pub mod set;
pub mod show;
pub mod util;

pub struct CmdResult<T> {
    pub value: T,
    pub infos: Vec<String>,
}

pub type Cmd<T> = Result<CmdResult<T>, String>;
