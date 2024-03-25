use once_cell::sync::Lazy;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicU8;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::task::Waker;
use std::task::{Context, Poll};
use std::task::{RawWaker, RawWakerVTable};
use toxoid_api::Entity;

// Shared state structure
pub struct LoaderState {
    pub completed: bool,
    pub waker: Option<Waker>,
    pub response: Option<Result<Entity, ()>>, // Simplify as needed
}

// This needs to be accessible by the callback, so it might be part of a larzger struct
// that is passed as user_data to the fetch function.
pub static mut LOADER_FUTURES_STATE: Lazy<HashMap<u8, LoaderState>> = Lazy::new(|| HashMap::new());
pub static LOADER_FUTURE_ID: Lazy<AtomicU8> = Lazy::new(|| AtomicU8::new(0));


pub struct FetchFuture {
    pub id: u8,
}

impl FetchFuture {
    pub fn new() -> Self {
        let id = LOADER_FUTURE_ID.fetch_add(1, Ordering::SeqCst);
        unsafe { 
            LOADER_FUTURES_STATE.insert(id, LoaderState {
                completed: false,
                waker: None,
                response: None,
            }) 
        };
        println!("Created future {}", id);
        FetchFuture { id }
    }
}

impl Future for FetchFuture {
    type Output = Result<Entity, ()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Polling future {}", self.id);
        let shared_state = unsafe { LOADER_FUTURES_STATE.get_mut(&self.id).unwrap() };
        if shared_state.completed {
            Poll::Ready(shared_state.response.take().unwrap())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

pub fn block_on<F: Future>(mut future: F) -> F::Output {
    let waker = dummy_waker();
    let mut context = Context::from_waker(&waker);

    loop {
        match unsafe { Pin::new_unchecked(&mut future) }.poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => continue, // In a real executor, you'd yield to the event loop here
        }
    }
}

pub fn dummy_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    unsafe fn dummy_raw_waker() -> RawWaker {
        let vtable = &RawWakerVTable::new(clone, wake, wake_by_ref, drop);
        RawWaker::new(std::ptr::null(), vtable)
    }

    unsafe { Waker::from_raw(dummy_raw_waker()) }
}