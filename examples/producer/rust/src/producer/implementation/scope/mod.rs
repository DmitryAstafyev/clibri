use super::{identification, producer::Control, Context};
use clibri::server;
use std::{future::Future, pin::Pin};

pub struct Scope<'a, E: server::Error, C: server::Control<E>> {
    deferred: Option<Pin<Box<dyn Future<Output = ()>>>>,
    pub context: &'a mut Context,
    pub control: &'a Control<E, C>,
    pub identification: &'a identification::Identification,
    pub filter: &'a identification::Filter<'a>,
}

impl<'a, E: server::Error, C: server::Control<E>> Scope<'a, E, C> {
    pub fn new(
        context: &'a mut Context,
        control: &'a Control<E, C>,
        identification: &'a identification::Identification,
        filter: &'a identification::Filter<'a>,
    ) -> Self {
        Self {
            deferred: None,
            context,
            control,
            identification,
            filter,
        }
    }

    pub fn deferred(&mut self, cb: Pin<Box<dyn Future<Output = ()>>>) {
        self.deferred = Some(cb);
    }

    pub async fn call(&mut self) {
        if let Some(cb) = self.deferred.take() {
            cb.await;
        }
    }
}

pub struct AnonymousScope<'a, E: server::Error, C: server::Control<E>> {
    deferred: Option<Pin<Box<dyn Future<Output = ()>>>>,
    pub context: &'a mut Context,
    pub control: &'a Control<E, C>,
    pub filter: &'a identification::Filter<'a>,
}

impl<'a, E: server::Error, C: server::Control<E>> AnonymousScope<'a, E, C> {
    pub fn new(
        context: &'a mut Context,
        control: &'a Control<E, C>,
        filter: &'a identification::Filter<'a>,
    ) -> Self {
        Self {
            deferred: None,
            context,
            control,
            filter,
        }
    }

    pub fn deferred(&mut self, cb: Pin<Box<dyn Future<Output = ()>>>) {
        self.deferred = Some(cb);
    }

    pub async fn call(&mut self) {
        if let Some(cb) = self.deferred.take() {
            cb.await;
        }
    }
}
