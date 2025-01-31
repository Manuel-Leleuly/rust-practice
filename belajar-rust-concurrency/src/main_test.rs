#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        sync::{
            atomic::{AtomicI32, Ordering},
            mpsc::channel,
            Arc, Barrier, Mutex, Once,
        },
        thread::{self, current, sleep, spawn},
        time::Duration,
    };

    use tokio::runtime::Runtime;

    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            for i in 1..=5 {
                println!("counter: {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("application finish");
        thread::sleep(Duration::from_secs(7));
    }

    #[test]
    fn test_join_thread() {
        let handle = thread::spawn(|| -> i32 {
            let mut counter = 0;
            for i in 1..=5 {
                println!("counter: {}", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1;
            }
            counter
        });

        let result = handle.join();
        match result {
            Ok(counter) => println!("total counter: {}", counter),
            Err(error) => println!("error: {:?}", error),
        }
        println!("application finish");
    }

    fn calculate() -> i32 {
        let mut counter = 0;
        let current_thread = thread::current();
        for i in 1..=5 {
            match current_thread.name() {
                Some(name) => println!("{}: counter: {}", name, i),
                None => println!("{:?}: counter: {}", current_thread.id(), i),
            }
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }
        counter
    }

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();
        println!("total counter1: {}", result1);
        println!("total counter2: {}", result2);
        println!("application finish");
    }

    #[test]
    fn test_parallel() {
        let handle1 = thread::spawn(calculate);
        let handle2 = thread::spawn(calculate);

        let result1 = handle1.join();
        let result2 = handle2.join();
        match result1 {
            Ok(counter) => println!("total counter 1: {}", counter),
            Err(error) => println!("error: {:?}", error),
        }
        match result2 {
            Ok(counter) => println!("total counter 2: {}", counter),
            Err(error) => println!("error: {:?}", error),
        }
        println!("application finish");
    }

    #[test]
    fn test_closure() {
        let current_thread = thread::current();
        println!("{}", current_thread.name().unwrap());

        let name = String::from("Manuel");
        let closure = move || {
            thread::sleep(Duration::from_secs(2));
            println!("Hello {}", name);
        };

        // closure();
        let handler = thread::spawn(closure);
        handler.join().unwrap();
    }

    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name(String::from("My Thread"));

        let handler = factory
            .spawn(calculate)
            .expect("Failed to create a new thread");

        let total = handler.join().unwrap();
        println!("total counter: {}", total);
    }

    #[test]
    fn test_channel() {
        let (sender, receiver) = channel::<String>();

        let handler1 = spawn(move || {
            sleep(Duration::from_secs(2));
            sender.send(String::from("Hello from thread"));
        });

        let handler2 = spawn(move || {
            let message = receiver.recv().unwrap();
            println!("{}", message);
        });

        let _ = handler1.join();
        let _ = handler2.join();
    }

    #[test]
    fn test_channel_queue() {
        let (sender, receiver) = channel::<String>();

        let handler1 = spawn(move || {
            for _ in 0..5 {
                sleep(Duration::from_secs(2));
                sender.send(String::from("Hello from thread"));
            }
            sender.send(String::from("Exit"));
        });

        let handler2 = spawn(move || loop {
            let message = receiver.recv().unwrap();
            if message == "Exit" {
                break;
            }
            println!("{}", message);
        });

        let _ = handler1.join();
        let _ = handler2.join();
    }

    #[test]
    fn test_channel_iterator() {
        let (sender, receiver) = channel::<String>();

        let handler1 = spawn(move || {
            for _ in 0..5 {
                sleep(Duration::from_secs(2));
                sender.send(String::from("Hello from thread"));
            }
        });

        let handler2 = spawn(move || {
            for value in receiver.iter() {
                println!("{}", value);
            }
        });

        let _ = handler1.join();
        let _ = handler2.join();
    }

    #[test]
    fn test_multi_sender() {
        let (sender, receiver) = channel::<String>();
        let sender2 = sender.clone();

        let handler3 = spawn(move || {
            for _ in 0..5 {
                sleep(Duration::from_secs(1));
                sender2.send(String::from("Hello from sender2"));
            }
        });

        let handler1 = spawn(move || {
            for _ in 0..5 {
                sleep(Duration::from_secs(2));
                sender.send(String::from("Hello from thread"));
            }
        });

        let handler2 = spawn(move || {
            for value in receiver.iter() {
                println!("{}", value);
            }
        });

        let _ = handler1.join();
        let _ = handler2.join();
        let _ = handler3.join();
    }

    static mut COUNTER: i32 = 0;

    #[test]
    fn test_race_condition() {
        let mut handlers = vec![];
        for _ in 0..10 {
            let handler = spawn(|| unsafe {
                for _ in 0..1000000 {
                    COUNTER += 1;
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        println!("counter: {}", unsafe { COUNTER });
    }

    #[test]
    fn test_atomic() {
        static COUNTER: AtomicI32 = AtomicI32::new(0);
        let mut handlers = vec![];

        for _ in 0..10 {
            let handler = spawn(move || {
                for _ in 0..1000000 {
                    COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        println!("counter: {}", COUNTER.load(Ordering::Relaxed));
    }

    #[test]
    fn test_atomic_reference() {
        let counter = Arc::new(AtomicI32::new(0));
        let mut handlers = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handler = spawn(move || {
                for _ in 0..1000000 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        println!("counter: {}", counter.load(Ordering::Relaxed));
    }

    #[test]
    fn test_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handlers = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handler = spawn(move || {
                for _ in 0..1000000 {
                    let mut data = counter_clone.lock().unwrap();
                    *data += 1;
                }
                // data akan di unklock secara otomatis setelah keluar dari scope
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }

        println!("counter: {}", *counter.lock().unwrap());
    }

    thread_local! {
        pub static NAME: RefCell<String> = RefCell::new(String::from("Default"));
    }
    thread_local! {
        pub static OTHER_NAME: RefCell<String> = RefCell::new(String::from("Default"));
    }

    #[test]
    fn test_thread_local() {
        let handler = spawn(|| {
            NAME.with_borrow_mut(|name| {
                *name = String::from("Manuel");
            });

            NAME.with_borrow(|name| {
                println!("name: {}", name);
            });
        });
        handler.join().unwrap();

        NAME.with_borrow(|name| {
            println!("name: {}", name);
        });
    }

    #[test]
    fn test_thread_panic() {
        let handle = spawn(|| {
            panic!("oops! something went wrong");
        });

        match handle.join() {
            Ok(_) => println!("thread finish"),
            Err(_) => println!("thread panic"),
        }

        println!("application finish");
    }

    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(10));
        let mut handlers = vec![];

        for i in 0..10 {
            let barrier_clone = Arc::clone(&barrier);
            let handler = spawn(move || {
                println!("Join Gamer-{}", i);
                barrier_clone.wait();
                println!("Gamer-{} Start!", i);
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }
    }

    static mut TOTAL_COUNTER: i32 = 0;
    static TOTAL_INIT: Once = Once::new();

    fn get_total() -> i32 {
        unsafe {
            TOTAL_INIT.call_once(|| {
                TOTAL_COUNTER += 1;
            });
            TOTAL_COUNTER
        }
    }

    #[test]
    fn test_once() {
        let mut handlers = vec![];
        for _ in 0..10 {
            let handler = spawn(|| {
                let total = get_total();
                println!("total: {}", total);
            });
            handlers.push(handler);
        }

        for handler in handlers {
            handler.join().unwrap();
        }
    }

    async fn get_async_data() -> String {
        sleep(Duration::from_secs(2));
        println!("Hello from Async");
        String::from("Hello from async")
    }

    #[tokio::test]
    async fn test_async() {
        let function = get_async_data();
        println!("Finish call async");
        let data = function.await;
        println!("{}", data);
    }

    async fn get_database_data(wait: u64) -> String {
        println!("{:?}: get database data", current().id());
        tokio::time::sleep(Duration::from_secs(wait)).await;
        println!("{:?}: hello from database", current().id());

        String::from("Hello from database")
    }

    #[tokio::test]
    async fn test_concurrent() {
        let mut handlers = vec![];
        for i in 0..5 {
            let handler = tokio::spawn(get_database_data(i));
            handlers.push(handler);
        }

        for handler in handlers {
            let data = handler.await.unwrap();
            println!("{}", data);
        }
    }

    async fn run_concurrent(runtime: Arc<Runtime>) {
        let mut handlers = vec![];
        for i in 0..5 {
            let handler = runtime.spawn(get_database_data(i));
            handlers.push(handler);
        }

        for handler in handlers {
            let data = handler.await.unwrap();
            println!("{}", data);
        }
    }

    #[test]
    fn test_runtime() {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(10)
                .enable_time()
                .build()
                .unwrap(),
        );

        // menjalankan async function di dalam runtime
        runtime.block_on(run_concurrent(Arc::clone(&runtime)));
    }
}
