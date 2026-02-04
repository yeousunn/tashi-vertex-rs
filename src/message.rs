use std::ffi::c_int;
use std::os::raw::c_void;
use std::pin::Pin;
use std::task;

use crate::engine::TVEngine;
use crate::error::TVResult;
use crate::ptr::Pointer;
use crate::{Engine, Event, SyncPoint};

const MESSAGE_EVENT: c_int = 1;

const MESSAGE_SYNC_POINT: c_int = 2;

pub enum Message {
    Event(Event),
    SyncPoint(SyncPoint),
}

impl Message {
    /// Listens for the next incoming message on the given engine.
    pub(crate) fn recv<'e>(
        engine: &'e Engine,
    ) -> impl Future<Output = crate::Result<Option<Self>>> + 'e {
        MessageRecieve {
            engine,
            invoked: false,
            waker: None,
            result: None,
        }
    }
}

struct MessageRecieve<'e> {
    engine: &'e Engine,
    invoked: bool,
    waker: Option<task::Waker>,
    result: Option<crate::Result<Option<Message>>>,
}

impl Future for MessageRecieve<'_> {
    type Output = crate::Result<Option<Message>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        extern "C" fn callback(
            result: TVResult,
            message: c_int,
            data: *const c_void,
            user_data: *mut c_void,
        ) {
            let user_data = unsafe { &mut *(user_data as *mut MessageRecieve) };

            user_data.result = Some(match result.ok_with(message) {
                Ok(MESSAGE_EVENT) => {
                    let handle = unsafe { Pointer::from_ptr_unchecked(data as *mut _) };
                    let event = Event { handle };

                    Ok(Some(Message::Event(event)))
                }

                Ok(MESSAGE_SYNC_POINT) => {
                    let handle = unsafe { Pointer::from_ptr_unchecked(data as *mut _) };
                    let sync_point = SyncPoint { handle };

                    Ok(Some(Message::SyncPoint(sync_point)))
                }

                Ok(_) => Ok(None),
                Err(error) => Err(error),
            });

            if let Some(waker) = user_data.waker.take() {
                waker.wake();
            }
        }

        if !self.invoked {
            // this is the first time poll is called
            // invoke the bind operation

            self.waker = Some(cx.waker().clone());
            self.invoked = true;

            let res = unsafe {
                tv_message_recv(
                    self.engine.handle.as_ptr(),
                    callback,
                    self.get_mut() as *mut _ as *mut c_void,
                )
            };

            if let Err(error) = res.ok() {
                return task::Poll::Ready(Err(error));
            }
        } else if let Some(result) = self.result.take() {
            return task::Poll::Ready(result);
        }

        task::Poll::Pending
    }
}

type TVMessageRecvCallback =
    extern "C" fn(result: TVResult, message: c_int, data: *const c_void, user_data: *mut c_void);

unsafe extern "C" {
    fn tv_message_recv(
        engine: *const TVEngine,
        callback: TVMessageRecvCallback,
        user_data: *mut c_void,
    ) -> TVResult;
}
