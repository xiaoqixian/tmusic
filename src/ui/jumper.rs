// Date: Wed Oct 25 15:10:24 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use super::component::Component;

pub fn jumper<C>(comp: C) -> Jumper<C> 
where C: Component
{
    Jumper::new(comp)
}

pub struct Jumper<C> {
    inner: C
}

impl<C> Jumper<C> {
    pub fn new(inner: C) -> Self {
        Self {
            inner
        }
    }
}

