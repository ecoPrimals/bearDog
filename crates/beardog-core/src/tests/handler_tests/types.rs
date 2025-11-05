//! Handler Test Helper Types
//!
//! This module contains all the shared test helper types used across
//! the handler test suite.

use beardog_errors::BearDogError;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use std::collections::HashMap;

/// Event type
#[derive(Debug, Clone)]
pub struct Event {
    name: String,
}

impl Event {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Handler function type
type HandlerFn = Box<dyn Fn(&Event) -> Result<(), BearDogError> + Send + Sync>;

/// Event dispatcher
pub struct EventDispatcher {
    handlers: HashMap<String, Vec<(usize, HandlerFn)>>,
    next_id: usize,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn register<F>(&mut self, event_name: &str, handler: F) -> usize
    where
        F: Fn(&Event) -> Result<(), BearDogError> + Send + Sync + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;

        self.handlers
            .entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push((id, Box::new(handler)));

        id
    }

    pub fn deregister(&mut self, handler_id: usize) {
        for handlers in self.handlers.values_mut() {
            handlers.retain(|(id, _)| *id != handler_id);
        }
    }

    pub fn dispatch(&self, event: &Event) -> Result<(), BearDogError> {
        if let Some(handlers) = self.handlers.get(event.name()) {
            for (_, handler) in handlers {
                handler(event)?;
            }
        }
        Ok(())
    }

    pub fn handler_count(&self, event_name: &str) -> usize {
        self.handlers.get(event_name).map_or(0, |h| h.len())
    }
}

/// Error handler action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerAction {
    Continue,
    Stop,
}

/// Handler result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerResult {
    Handled,
    Unhandled,
}

/// Error handler chain
pub struct ErrorHandlerChain {
    handlers: Vec<Box<dyn Fn(&BearDogError) -> HandlerAction + Send + Sync>>,
}

impl ErrorHandlerChain {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler<F>(&mut self, handler: F)
    where
        F: Fn(&BearDogError) -> HandlerAction + Send + Sync + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    pub fn handle(&self, error: &BearDogError) -> HandlerResult {
        if self.handlers.is_empty() {
            return HandlerResult::Unhandled;
        }

        for handler in &self.handlers {
            match handler(error) {
                HandlerAction::Continue => continue,
                HandlerAction::Stop => return HandlerResult::Handled,
            }
        }

        HandlerResult::Handled
    }
}

/// Async dispatcher
pub struct AsyncDispatcher {
    pending: Arc<AtomicUsize>,
}

impl AsyncDispatcher {
    pub fn new() -> Self {
        Self {
            pending: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn dispatch_async<F>(&self, _event_name: &str, handler: F) -> AsyncHandle
    where
        F: FnOnce() + Send + 'static,
    {
        use std::thread;

        self.pending.fetch_add(1, Ordering::SeqCst);
        let pending = self.pending.clone();

        let handle = thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                handler();
            }));

            pending.fetch_sub(1, Ordering::SeqCst);

            result.map_err(|_| BearDogError::system("Handler panicked".to_string()))
        });

        AsyncHandle {
            handle: Some(handle),
        }
    }

    pub fn pending_count(&self) -> usize {
        self.pending.load(Ordering::SeqCst)
    }
}

/// Async handle
pub struct AsyncHandle {
    handle: Option<std::thread::JoinHandle<Result<(), BearDogError>>>,
}

impl AsyncHandle {
    pub fn wait(mut self) -> Result<(), BearDogError> {
        if let Some(handle) = self.handle.take() {
            handle
                .join()
                .unwrap_or_else(|_| Err(BearDogError::system("Thread join failed".to_string())))
        } else {
            Ok(())
        }
    }
}

/// Middleware result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiddlewareResult {
    Continue,
    Stop,
}

/// Context for middleware
pub struct Context {
    data: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

/// Middleware
pub struct Middleware {
    pre: Box<dyn Fn(&mut Context) -> MiddlewareResult + Send + Sync>,
    post: Box<dyn Fn(&mut Context) -> MiddlewareResult + Send + Sync>,
}

impl Middleware {
    pub fn new<F1, F2>(pre: F1, post: F2) -> Self
    where
        F1: Fn(&mut Context) -> MiddlewareResult + Send + Sync + 'static,
        F2: Fn(&mut Context) -> MiddlewareResult + Send + Sync + 'static,
    {
        Self {
            pre: Box::new(pre),
            post: Box::new(post),
        }
    }
}

/// Handler pipeline
pub struct HandlerPipeline {
    middleware: Vec<Middleware>,
    handler: Option<Box<dyn Fn(&mut Context) -> Result<(), BearDogError> + Send + Sync>>,
}

impl HandlerPipeline {
    pub fn new() -> Self {
        Self {
            middleware: Vec::new(),
            handler: None,
        }
    }

    pub fn add_middleware(&mut self, middleware: Middleware) {
        self.middleware.push(middleware);
    }

    pub fn set_handler<F>(&mut self, handler: F)
    where
        F: Fn(&mut Context) -> Result<(), BearDogError> + Send + Sync + 'static,
    {
        self.handler = Some(Box::new(handler));
    }

    pub fn execute(&self, ctx: &mut Context) -> Result<(), BearDogError> {
        // Execute pre-processing
        for middleware in &self.middleware {
            match (middleware.pre)(ctx) {
                MiddlewareResult::Continue => continue,
                MiddlewareResult::Stop => return Ok(()),
            }
        }

        // Execute handler
        if let Some(handler) = &self.handler {
            handler(ctx)?;
        }

        // Execute post-processing
        for middleware in self.middleware.iter().rev() {
            (middleware.post)(ctx);
        }

        Ok(())
    }
}

/// Handler state
pub struct HandlerState {
    counters: HashMap<String, usize>,
    last_event: String,
}

impl HandlerState {
    pub fn new() -> Self {
        Self {
            counters: HashMap::new(),
            last_event: String::new(),
        }
    }

    pub fn increment_counter(&mut self, name: &str) -> usize {
        let counter = self.counters.entry(name.to_string()).or_insert(0);
        *counter += 1;
        *counter
    }

    pub fn get_counter(&self, name: &str) -> usize {
        *self.counters.get(name).unwrap_or(&0)
    }

    pub fn set_last_event(&mut self, event_name: &str) {
        self.last_event = event_name.to_string();
    }

    pub fn last_event(&self) -> &str {
        &self.last_event
    }

    pub fn reset(&mut self) {
        self.counters.clear();
        self.last_event.clear();
    }
}

/// Stateful handler
pub struct StatefulHandler<F>
where
    F: FnMut(&Event, &mut HandlerState) -> usize,
{
    handler: F,
    state: Arc<Mutex<HandlerState>>,
}

impl<F> StatefulHandler<F>
where
    F: FnMut(&Event, &mut HandlerState) -> usize,
{
    pub fn new(handler: F, state: Arc<Mutex<HandlerState>>) -> Self {
        Self { handler, state }
    }

    pub fn handle(&mut self, event: &Event) -> usize {
        let mut state = self.state.lock().unwrap();
        (self.handler)(event, &mut *state)
    }
}

/// Timeout dispatcher
pub struct TimeoutDispatcher;

impl TimeoutDispatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn dispatch_with_timeout<F, T>(
        &self,
        _event_name: &str,
        timeout: Duration,
        handler: F,
    ) -> Result<T, BearDogError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let result = handler();
            let _ = tx.send(result);
        });

        rx.recv_timeout(timeout)
            .map_err(|_| BearDogError::system("Handler timeout".to_string()))
    }

    pub fn dispatch_async_with_timeout<F>(
        &self,
        _event_name: &str,
        timeout: Duration,
        handler: F,
    ) -> TimeoutHandle
    where
        F: FnOnce() + Send + 'static,
    {
        use std::sync::mpsc;
        use std::thread;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            handler();
            let _ = tx.send(());
        });

        TimeoutHandle { rx, timeout }
    }

    pub fn dispatch_with_timeout_and_cleanup<F, C, T>(
        &self,
        event_name: &str,
        timeout: Duration,
        handler: F,
        cleanup: C,
    ) -> Result<T, BearDogError>
    where
        F: FnOnce() -> T + Send + 'static,
        C: FnOnce() + Send + 'static,
        T: Send + 'static,
    {
        let result = self.dispatch_with_timeout(event_name, timeout, handler);

        if result.is_err() {
            std::thread::spawn(cleanup);
        }

        result
    }
}

/// Timeout handle
pub struct TimeoutHandle {
    rx: std::sync::mpsc::Receiver<()>,
    timeout: Duration,
}

impl TimeoutHandle {
    pub fn wait(self) -> Result<(), BearDogError> {
        self.rx
            .recv_timeout(self.timeout)
            .map_err(|_| BearDogError::system("Timeout".to_string()))
    }
}

/// Retry handler
pub struct RetryHandler {
    max_retries: usize,
}

impl RetryHandler {
    pub fn new(max_retries: usize) -> Self {
        Self { max_retries }
    }

    pub fn execute<F, T>(&self, mut handler: F) -> Result<T, BearDogError>
    where
        F: FnMut() -> Result<T, BearDogError>,
    {
        let mut attempts = 0;

        loop {
            match handler() {
                Ok(result) => return Ok(result),
                Err(error) => {
                    attempts += 1;
                    if attempts > self.max_retries {
                        return Err(error);
                    }
                }
            }
        }
    }
}

/// Fallback handler
pub struct FallbackHandler;

impl FallbackHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn execute_with_fallback<F1, F2, T>(
        &self,
        primary: F1,
        fallback: F2,
    ) -> Result<T, BearDogError>
    where
        F1: FnOnce() -> Result<T, BearDogError>,
        F2: FnOnce() -> Result<T, BearDogError>,
    {
        primary().or_else(|_| fallback())
    }
}

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

/// Circuit breaker
pub struct CircuitBreaker {
    state: Arc<Mutex<CircuitState>>,
    failure_count: Arc<AtomicUsize>,
    failure_threshold: usize,
    timeout: Duration,
    last_failure: Arc<Mutex<Option<Instant>>>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: usize, timeout: Duration) -> Self {
        Self {
            state: Arc::new(Mutex::new(CircuitState::Closed)),
            failure_count: Arc::new(AtomicUsize::new(0)),
            failure_threshold,
            timeout,
            last_failure: Arc::new(Mutex::new(None)),
        }
    }

    pub fn execute<F, T>(&self, handler: F) -> Result<T, BearDogError>
    where
        F: FnOnce() -> Result<T, BearDogError>,
    {
        // Check if circuit should transition from open to half-open
        {
            let mut state = self.state.lock().unwrap();
            if *state == CircuitState::Open {
                let last_failure = self.last_failure.lock().unwrap();
                if let Some(instant) = *last_failure {
                    if instant.elapsed() >= self.timeout {
                        *state = CircuitState::HalfOpen;
                    }
                }
            }
        }

        let state = *self.state.lock().unwrap();

        match state {
            CircuitState::Open => Err(BearDogError::system("Circuit breaker open".to_string())),
            CircuitState::Closed | CircuitState::HalfOpen => {
                match handler() {
                    Ok(result) => {
                        // Success - reset
                        self.failure_count.store(0, Ordering::SeqCst);
                        *self.state.lock().unwrap() = CircuitState::Closed;
                        Ok(result)
                    }
                    Err(error) => {
                        // Failure - increment
                        let failures = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
                        *self.last_failure.lock().unwrap() = Some(Instant::now());

                        if failures >= self.failure_threshold {
                            *self.state.lock().unwrap() = CircuitState::Open;
                        }

                        Err(error)
                    }
                }
            }
        }
    }

    pub fn is_open(&self) -> bool {
        *self.state.lock().unwrap() == CircuitState::Open
    }

    pub fn is_closed(&self) -> bool {
        *self.state.lock().unwrap() == CircuitState::Closed
    }
}

/// Transforming handler
pub struct TransformingHandler;

impl TransformingHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn execute_with_transform<F, T, E>(
        &self,
        handler: F,
        transform: E,
    ) -> Result<T, BearDogError>
    where
        F: FnOnce() -> Result<T, BearDogError>,
        E: FnOnce(BearDogError) -> BearDogError,
    {
        handler().map_err(transform)
    }
}
