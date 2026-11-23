use std::ops::Deref;
use ecs_macros::ComponentInCrate;

type InnerId = u16;
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, ComponentInCrate)]
pub struct EntityId(InnerId);
impl EntityId {
    pub(crate) const fn new(id: InnerId)->Self{
        Self(id)
    }
    pub const fn id(&self)->InnerId{
        self.0
    }
}
impl Deref for EntityId {
    type Target = InnerId;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}