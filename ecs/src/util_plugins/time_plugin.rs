use std::time::{Duration, Instant};
use crate::prelude::{Plugin, ResMut};
use ecs_macros::ComponentInCrate;

#[derive(ComponentInCrate)]
pub struct DeltaTime{
    previous_frame: Instant,
    dt: Duration
}
impl DeltaTime{
    fn new()->Self{
        Self { previous_frame: Instant::now(), dt: Duration::ZERO }
    }
    pub fn dt(&self)->Duration{
        self.dt
    }
    pub fn dt_secs_f32(&self)->f32{
        self.dt().as_secs_f32()
    }
}

pub struct TimePlugin;
impl Plugin for TimePlugin {
    fn add_plugin(self, builder: crate::prelude::SchedulerBuilder)->crate::prelude::SchedulerBuilder {
        builder
            .add_resource(DeltaTime::new())
            .add_system(set_delta_time)
    }
}

fn set_delta_time(mut dt: ResMut<DeltaTime>){
    let this_frame = Instant::now();
    dt.dt = this_frame - dt.previous_frame;
    dt.previous_frame = this_frame;
}

#[derive(Clone)]
/// Remember to call .tick() on this timer once per frame
pub struct Timer{
    start: Instant,
    duration: Duration,
    state: TimerState,
}
#[derive(PartialEq, Eq, Clone)]
enum TimerState{
    Repeat,
    RepeatJustFinished,
    Once,
    OnceJustFinished,
    OnceFinished
}
impl TimerState {
    fn is_repeat(&self) -> bool {
        matches!(self,Self::Repeat|Self::RepeatJustFinished)
    }
    fn just_finished(&self) -> bool {
        matches!(self,Self::OnceJustFinished|Self::RepeatJustFinished)
    }
    fn finished(&self) -> bool {
        matches!(self,Self::OnceJustFinished|Self::RepeatJustFinished|Self::OnceFinished)
    }
}
impl Timer{
    pub fn new(duration: Duration, repeat: bool)->Self{
        Self { start: Instant::now(), duration, state: if repeat {TimerState::Repeat} else {TimerState::Once} }
    }
    //returns self.just_finished()
    pub fn tick(&mut self) -> bool {
        if self.state.is_repeat() {
            self.state = TimerState::Repeat;
            if Instant::now() > self.start + self.duration {
                self.state = TimerState::RepeatJustFinished;
                self.start += self.duration;
            }
        }else{
            if self.state == TimerState::OnceJustFinished {
                self.state = TimerState::OnceFinished;
            } else if self.state == TimerState::Once && Instant::now() > self.start + self.duration {
                self.state = TimerState::OnceJustFinished;
            }
        }
        self.just_finished()
    }
    //true for the tick after it finished, once ticked again, this returns false
    pub fn just_finished(&self) -> bool {
        self.state.just_finished()
    }
    pub fn finished(&self) -> bool {
        self.state.finished()
    }
}