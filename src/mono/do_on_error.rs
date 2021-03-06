use super::spi::Mono;
use crate::spi::{Publisher, Subscriber, Subscription};
use std::marker::PhantomData;

pub struct MonoDoOnError<T, E, M, F>
where
  T: 'static,
  E: 'static,
  M: Mono<T, E> + Sized,
  F: 'static + Send + Fn(&E),
{
  source: M,
  f: F,
  _t: PhantomData<T>,
  _e: PhantomData<E>,
}

impl<T, E, M, F> MonoDoOnError<T, E, M, F>
where
  T: 'static,
  E: 'static,
  M: Mono<T, E> + Sized,
  F: 'static + Send + Fn(&E),
{
  pub(crate) fn new(source: M, f: F) -> MonoDoOnError<T, E, M, F> {
    MonoDoOnError {
      source,
      f,
      _t: PhantomData,
      _e: PhantomData,
    }
  }
}

impl<T, E, M, F> Mono<T, E> for MonoDoOnError<T, E, M, F>
where
  M: Mono<T, E> + Sized,
  F: 'static + Send + Fn(&E),
{
}

impl<T, E, M, F> Publisher for MonoDoOnError<T, E, M, F>
where
  M: Mono<T, E> + Sized,
  F: 'static + Send + Fn(&E),
{
  type Item = T;
  type Error = E;

  fn subscribe(self, subscriber: impl Subscriber<Item = T, Error = E> + 'static + Send) {
    let sub = DoOnErrorSubscriber::new(subscriber, self.f);
    self.source.subscribe(sub);
  }
}

struct DoOnErrorSubscriber<T, E, S, F>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(&E),
{
  actual: S,
  action: F,
}

impl<T, E, S, F> DoOnErrorSubscriber<T, E, S, F>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(&E),
{
  fn new(actual: S, action: F) -> DoOnErrorSubscriber<T, E, S, F> {
    DoOnErrorSubscriber { actual, action }
  }
}

impl<T, E, S, F> Subscriber for DoOnErrorSubscriber<T, E, S, F>
where
  S: 'static + Send + Subscriber<Item = T, Error = E>,
  F: 'static + Send + Fn(&E),
{
  type Item = T;
  type Error = E;

  fn on_subscribe(&self, subscription: impl Subscription) {
    self.actual.on_subscribe(subscription);
  }
  fn on_complete(&self) {
    self.actual.on_complete();
  }
  fn on_next(&self, t: Self::Item) {
    self.actual.on_next(t);
  }
  fn on_error(&self, e: Self::Error) {
    (self.action)(&e);
    self.actual.on_error(e);
  }
}
