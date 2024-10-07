/// Marks a component as being tied to a specific animation state. Used to drive automatic 
pub trait AnimationStateId {
    fn name(&self) -> &'static str;
}
