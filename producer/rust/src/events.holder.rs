use super::context::Context;
use super::event_observer::EventObserverErrors;
use std::cmp::Eq;
use std::hash::Hash;

pub trait EventsHolder<Request: Clone, Identification, Conclusion: Eq + Hash> {
    fn emit(
        &mut self,
        conclusion: Conclusion,
        cx: &mut dyn Context<Identification>,
        request: Request,
    ) -> Result<(), EventObserverErrors>;
}
