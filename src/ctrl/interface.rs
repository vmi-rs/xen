use std::rc::Rc;

use super::XenInterfaceHandle;
use crate::XenError;

#[derive(Debug, Clone)]
pub struct XenInterface {
    pub(crate) handle: Rc<XenInterfaceHandle>,
}

impl XenInterface {
    pub fn new() -> Result<Self, XenError> {
        Ok(Self {
            handle: Rc::new(XenInterfaceHandle::new()?),
        })
    }
}
