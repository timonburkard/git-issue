#![deny(warnings, clippy::unwrap_used, clippy::expect_used)]

pub mod cmd {
    pub mod edit;
    pub mod init;
    pub mod link;
    pub mod list;
    pub mod model;
    pub mod new;
    pub mod set;
    pub mod show;
}
