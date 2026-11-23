use crate::prelude::*;

pub trait Plugin{
    fn add_plugin(self, builder: SchedulerBuilder)->SchedulerBuilder;
}