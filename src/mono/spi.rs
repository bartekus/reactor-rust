use super::misc::BlockSubscriber;
use super::{
  DoOnError, Foreach, MonoDoFinally, MonoFilter, MonoFlatMap, MonoScheduleOn, MonoTransform,
};
use crate::schedulers::Scheduler;
use crate::spi::Publisher;

pub trait Mono<T, E>: Publisher<Item = T, Error = E> {
  fn block(self) -> Result<Option<Self::Item>, Self::Error>
  where
    Self::Item: 'static + Send,
    Self::Error: 'static + Send,
    Self: Sized,
  {
    let (sub, rx) = BlockSubscriber::new();
    self.subscribe(sub);
    rx.recv().unwrap()
  }

  fn do_on_error<F>(self, f: F) -> DoOnError<Self::Item, Self::Error, Self, F>
  where
    F: 'static + Send + Fn(&Self::Error),
    Self: Sized,
  {
    DoOnError::new(self, f)
  }

  fn do_on_success<F>(self, f: F) -> Foreach<Self, Self::Item, F, Self::Error>
  where
    F: 'static + Send + Fn(&Self::Item),
    Self: Sized,
  {
    Foreach::new(self, f)
  }

  fn map<A, F>(self, transform: F) -> MonoTransform<Self, Self::Item, A, F, Self::Error>
  where
    F: 'static + Send + Fn(Self::Item) -> A,
    Self: Sized,
  {
    MonoTransform::new(self, transform)
  }

  fn flatmap<A, M, F>(self, mapper: F) -> MonoFlatMap<Self::Item, A, Self::Error, Self, M, F>
  where
    Self: Sized,
    M: Mono<A, Self::Error>,
    F: 'static + Send + Fn(Self::Item) -> M,
  {
    MonoFlatMap::new(self, mapper)
  }

  fn do_finally<F>(self, action: F) -> MonoDoFinally<Self::Item, Self::Error, Self, F>
  where
    Self: Sized,
    F: 'static + Send + Fn(),
  {
    MonoDoFinally::new(self, action)
  }

  fn filter<F>(self, predicate: F) -> MonoFilter<Self, Self::Item, F, Self::Error>
  where
    Self: Sized,
    F: 'static + Send + Fn(&Self::Item) -> bool,
  {
    MonoFilter::new(self, predicate)
  }

  fn subscribe_on<C>(self, scheduler: C) -> MonoScheduleOn<Self::Item, Self::Error, Self, C>
  where
    Self: 'static + Send + Sized,
    C: Scheduler<Item = Self::Item, Error = Self::Error>,
  {
    MonoScheduleOn::new(self, scheduler)
  }
}
