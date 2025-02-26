#[derive(thiserror::Error, Debug)]
pub enum XenError {
    #[error(transparent)]
    Xen(#[from] XcError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(&'static str),
}

#[derive(Debug)]
pub struct XcError {
    #[expect(unused)]
    pub(crate) rc: i32,
    #[expect(unused)]
    pub(crate) code: u32,
    pub(crate) desc: &'static str,
    #[expect(unused)]
    pub(crate) backtrace: std::backtrace::Backtrace,
}

impl std::error::Error for XcError {}

impl std::fmt::Display for XcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.desc)
    }
}

impl XcError {
    pub fn new(rc: i32, code: u32, desc: &'static str) -> Self {
        Self {
            rc,
            code,
            desc,
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }
}
