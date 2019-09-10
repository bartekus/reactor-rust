use crate::mono::Mono;
use crate::spi::Subscriber;

pub struct MonoFilter<M, T, F, E>
where
  M: Mono<Item = T, Error = E> + Sized,
  F: Fn(&T) -> bool,
{
  parent: M,
  predicate: F,
}

impl<M, T, F, E> MonoFilter<M, T, F, E>
where
  M: Mono<Item = T, Error = E> + Sized,
  F: Fn(&T) -> bool,
{
  pub(crate) fn new(m: M, f: F) -> MonoFilter<M, T, F, E> {
    MonoFilter {
      parent: m,
      predicate: f,
    }
  }
}

impl<M, T, F, E> Mono for MonoFilter<M, T, F, E>
where
  M: Mono<Item = T, Error = E> + Sized,
  F: Fn(&T) -> bool,
{
  type Item = T;
  type Error = E;

  fn subscribe<S>(self, subscriber: S)
  where
    S: Subscriber<Item = T, Error = E>,
  {
    let m = self.parent;
    let f = self.predicate;
    let sub = FilterSubscriber::new(subscriber, f);
    m.subscribe(sub);
  }
}

struct FilterSubscriber<T, S, F, E>
where
  S: Subscriber<Item = T, Error = E>,
  F: Fn(&T) -> bool,
{
  actual: S,
  predicate: F,
}

impl<T, S, F, E> FilterSubscriber<T, S, F, E>
where
  S: Subscriber<Item = T, Error = E>,
  F: Fn(&T) -> bool,
{
  fn new(actual: S, predicate: F) -> FilterSubscriber<T, S, F, E> {
    FilterSubscriber { actual, predicate }
  }
}

impl<T, S, F, E> Subscriber for FilterSubscriber<T, S, F, E>
where
  S: Subscriber<Item = T, Error = E>,
  F: Fn(&T) -> bool,
{
  type Item = T;
  type Error = E;

  fn on_complete(&self) {
    self.actual.on_complete()
  }

  fn on_next(&self, t: T) {
    if (self.predicate)(&t) {
      self.actual.on_next(t);
    }
  }

  fn on_error(&self, e: E) {
    self.actual.on_error(e);
  }
}