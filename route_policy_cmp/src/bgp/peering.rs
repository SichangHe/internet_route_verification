use crate::parse::peering::Peering;

use super::{
    cmp::Compare,
    report::{AllReport, JoinReportItems, ToAllReport},
};

pub struct CheckPeering<'a> {
    pub compare: &'a Compare<'a>,
    pub peering: &'a Peering,
    pub accept_num: usize,
}

impl<'a> CheckPeering<'a> {
    pub fn check(&self) -> AllReport {
        self.check_remote_route()?
            .join(self.check_local_route()?)
            .join(self.check_remote_as()?)
            .to_all()
    }

    fn check_remote_route(&self) -> AllReport {
        todo!()
    }

    fn check_local_route(&self) -> AllReport {
        todo!()
    }

    fn check_remote_as(&self) -> AllReport {
        todo!()
    }
}
