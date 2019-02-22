#![feature(trace_macros)]
#![feature(dbg_macro)]
#![feature(try_trait)]

#[macro_use]
extern crate lazy_static;

mod string_test;
mod fizzbuzz;
mod csv;
mod accumulate;
mod acronym;
mod null;

fn main() {
    let res = accumulate::accumulate(&[2, 3, 4, 5], |x| { x * x });
    dbg!(res);

    acronym::acronym("dengjie de");
}
//trace_macros!(true);
//
//#![feature(arbitrary_self_types, futures_api, pin)]
//
//use std::{
//    future::Future,
//    pin::{Pin, Unpin},
//    sync::{Arc, Mutex},
//    task::{LocalWaker, Poll, Waker},
//    thread,
//    time::Duration,
//};
//
//use {
//    futures::future::FutureObj,
//    std::{
//        sync::mpsc::{sync_channel, SyncSender, Receiver},
//        task::{
//            local_waker_from_nonlocal,
//            Wake,
//        },
//    },
//};
//
//struct Executor {
//    ready_queue: Receiver<Arc<Task>>,
//}
//
//#[derive(Clone)]
//struct Spawner {
//    task_sender: SyncSender<Arc<Task>>,
//}
//
//struct Task {
//    // In-progress future that should be pushed to completion
//    //
//    // The `Mutex` is not necessary for correctness, since we only have
//    // one thread executing tasks at once. However, `rustc` isn't smart
//    // enough to know that `future` is only mutated from one thread,
//    // so we use it in order to provide safety. A production executor would
//    // not need this, and could use `UnsafeCell` instead.
//    future: Mutex<Option<FutureObj<'static, ()>>>,
//
//    // Handle to spawn tasks onto the task queue
//    task_sender: SyncSender<Arc<Task>>,
//}
//
//fn new_executor_and_spawner() -> (Executor, Spawner) {
//    // Maximum number of tasks to allow queueing in the channel at once.
//    // This is just to make `sync_channel` happy, and wouldn't be present in
//    // a real executor.
//    const MAX_QUEUED_TASKS: usize = 10_000;
//    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
//    (Executor { ready_queue }, Spawner { task_sender})
//}
//
//impl Spawner {
//    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
//        let future_obj = FutureObj::new(Box::new(future));
//        let task = Arc::new(Task {
//            future: Mutex::new(Some(future_obj)),
//            task_sender: self.task_sender.clone(),
//        });
//        self.task_sender.send(task).expect("too many tasks queued");
//    }
//}
//
//
//impl Wake for Task {
//    fn wake(arc_self: &Arc<Self>) {
//        // Implement `wake` by sending this task back onto the task channel
//        // so that it will be polled again by the executor.
//        let cloned = arc_self.clone();
//        arc_self.task_sender.send(cloned).expect("too many tasks queued");
//    }
//}
//
//impl Executor {
//    fn run(&self) {
//        while let Ok(task) = self.ready_queue.recv() {
//            let mut future_slot = task.future.lock().unwrap();
//            // Take the future, and if it has not yet completed (is still Some),
//            // poll it in an attempt to complete it.
//            if let Some(mut future) = future_slot.take() {
//                // Create a `LocalWaker` from the task itself
//                let lw = local_waker_from_nonlocal(task.clone());
//                if let Poll::Pending = Pin::new(&mut future).poll(&lw) {
//                    // We're not done processing the future, so put it
//                    // back in its task to be run again in the future.
//                    *future_slot = Some(future);
//                }
//            }
//        }
//    }
//}
//
//struct TimerFuture {
//    shared_state: Arc<Mutex<SharedState>>,
//}
//
///// Shared state between the future and the thread
//struct SharedState {
//    /// Whether or not the sleep time has elapsed
//    completed: bool,
//
//    /// The waker for the task that `TimerFuture` is running on.
//    /// The thread can use this after setting `completed = true` to tell
//    /// `TimerFuture`'s task to wake up, see that `completed = true`, and
//    /// move forward.
//    waker: Option<Waker>,
//}
//
//impl Unpin for TimerFuture {}
//
//impl Future for TimerFuture {
//    type Output = ();
//    fn poll(self: Pin<&mut Self>, lw: &LocalWaker)
//            -> Poll<Self::Output> {
//        let mut state = self.shared_state.lock().unwrap();
//        if shared_state.completed {
//            Poll::Ready(())
//        } else {
//            // Set waker so that the thread can wake up the current task
//            // when the timer has completed, ensuring that the future is polled
//            // again and sees that `completed = true`.
//            shared_state.waker = Some(lw.clone().into_waker());
//            Poll::Pending
//        }
//    }
//}
//
//impl TimerFuture {
//    pub fn new(duration: Duration) -> Self {
//        let shared_state = Arc::new(Mutex::new(SharedState {
//            completed: false,
//            waker: None,
//        }));
//
//        // Spawn the new thread
//        let thread_shared_state = shared_state.clone();
//        thread::spawn(move || {
//            thread::sleep(duration);
//            let mut shared_state = thread_shared_state.lock().unwrap();
//            // Signal that the timer has completed and wake up the last
//            // task on which the future was polled, if one exists.
//            shared_state.completed = true;
//            if let Some(waker) = &shared_state.waker {
//                waker.wake();
//            }
//        });
//
//        TimerFuture { shared_state }
//    }
//}
//
//trait SimpleFuture {
//    type Output;
//    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
//}
//
////enum Poll<T> {
////    Ready(T),
////    Pending,
////}
//
//struct AndThenFut<A: SimpleFuture> {
//    first: Option<A>,
//    second: A,
//}
//
//impl<A> SimpleFuture for AndThenFut<A> where A: SimpleFuture {
//    type Output = A::Output;
//    fn poll(&mut self, wake: fn()) -> Poll<A::Output> {
//        if let Some(first) = &mut self.first {
//            let c = first;
//        }
//        self.second.poll(wake)
//    }
//}

//use std::mem;
//
//
////trace_macros!(true);
//
//trait Bird {
//    fn fly(&self);
//    fn say(&self, b: Self) where Self: Sized {
//        println!("Biu Biu");
//    }
//}
//
//#[derive(Debug)]
//struct Duck;
//struct Swan;
//
//
//impl Bird for Duck {
//    fn fly(&self) {
//        println!("Duck Duck");
//    }
//}
//
//impl Bird for Swan {
//    fn fly(&self) {
//        println!("Swan Swan");
//    }
//}
//
//fn print_traitobject(p: &dyn Bird) {
//    let (data, vtable): (usize, *const usize) = unsafe { mem::transmute(p) };
//
//    println!("Trait Object [data: {}, vtable: {:p}]", data, vtable);
//
//    unsafe {
//        println!("data in vtable: [{}, {}, {}, {}, {}, {}, {}, {}, {}]", *vtable, *vtable.offset(1), *vtable.offset(2), *vtable.offset(3), *vtable.offset(4),*vtable.offset(5), *vtable.offset(6), *vtable.offset(7), *vtable.offset(8));
//    }
//}
//
//fn re(b: Box<dyn Bird>) {
//}
//
//#[macro_use]
//extern crate lazy_static;
//
//use std::collections::HashMap;
//
//lazy_static! {
//    static ref VEC:Vec<u8> = vec![0x18u8, 0x11u8];
//    static ref MAP: HashMap<u32, String> = {
//        let mut map = HashMap::new();
//        map.insert(18, "hury".to_owned());
//        map
//    };
//    static ref PAGE:u32 = mulit(18);
//}
//
//fn mulit(i: u32) -> u32 {
//    i * 2
//}
////
//pub struct Request<'a, 'b: 'a> {
//    a: &'a (),
//    b: &'b (),
//}
//pub trait Handler<'a>: Send + Sync + 'a {
//    fn handle(&self, a: &mut Request) -> Result<()>;
//}
//impl<'a, F: 'a> Handler<'a> for F where F: Send + Sync + 'a + Fn(&mut Request) -> Result<()> {
//    fn handle(&self, a: &mut Request) -> Result<()> {
//        self(a)
//    }
//}
//fn make_handler<'a>(h: &'a dyn Handler<'a>) -> Box<Handler<'a> + 'a> {
//    Box::new(move |req: &mut Request| h.handle(req))
//}
//enum Data<'a> {
//    Value(&'a String),
//    Null,
//}
//
//extern crate serde;
//
//extern crate serde_json;
//
//use serde_json::{Result, Value};
//
//fn untyped_example() -> Result<()> {
//    let data = r#"
//        {
//            "name": "John Doe",
//            "age": 43,
//            "phones": [
//                "+44 1234567",
//                "+44 2345678"
//            ]
//        }
//    "#;
//
//    let v: Value = serde_json::from_str(data)?;
//
//    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
//    Ok(())
//}
//
//use serde::{Deserialize, Serialize};
//#[macro_use]
//extern crate serde_derive;
//
//#[derive(Serialize, Deserialize)]
//struct Person {
//    name: String,
//    age: u8,
//    phones: Vec<String>
//}
//
//fn typed_example() -> Result<()> {
//    let data = r#"
//        {
//            "name": "John Doe",
//            "age": 43,
//            "phones": [
//                "+44 1234567",
//                "+44 2345678"
//            ]
//        }"#;
//    let  p: Person = serde_json::from_str(data)?;
//    println!("Please call {} at the number {}, {}", p.name, p.phones[0], serde_json::to_string(&p)?);
//    Ok(())
//}


//    csv::run();
//    let s = String::new();
//    let d = Data::Value(&s);
//
//    parse(&d, |d| match d {
//        Data::Value(s) => Some(s),
//        _ => None,
//    });
//    untyped_example();
//
//    typed_example();

//fn parse<'a, F>(data: &Data<'a>, filter: F)
//    where
//        F: Fn(&Data<'a>) -> Option<&'a String>,
//{
//    filter(data);
//}

//fn main() {
//    let d = Duck{};
//    let s = Swan{};
//
//    let df = Duck::fly as usize;
//    let sf = Swan::fly as usize;
//
//    let ds = Duck::say as usize;
//    let ss = Swan::say as usize;
//
//    re(Box::new(Duck{}));
//    println!("Duck fly {}, say {}", df, ds);
//    println!("Swan fly {}, sat {}", sf, ss);
//    print_traitobject(&*Box::new(Duck{}));
//    print_traitobject(&d);
//    print_traitobject(&s);
//
//    let dd = crate::Duck{};
//    println!("{:?}", dd);
//
////    println!("{:?}", VEC);
////    println!("{:?}", MAP);
//}