use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::{mpsc, Arc, Mutex},
    thread::{spawn, JoinHandle},
};

use error::CustomError;
pub type Res<T> = Result<T, Box<dyn Error>>;
pub type Null = Res<()>;
pub type Job = Box<dyn Fn(Vec<u8>) -> Res<Vec<u8>> + Send + 'static>;
pub type Work = Arc<Mutex<mpsc::Receiver<TaskPair>>>;
pub type Control = Arc<Mutex<mpsc::Receiver<ControlPrompt>>>;
const MAX_THREADS: usize = 32;

pub mod error;
pub mod primitives;

pub enum ControlPrompt {
    Terminate
}

pub struct Threadpool {
    handles: Vec<(Worker, mpsc::Sender<ControlPrompt>)>,
    tx: mpsc::Sender<TaskPair>,
    rx: Work,
}

impl Threadpool {
    pub fn new(n: usize) -> Self {
        let mut w = vec![];
        let c = mpsc::channel();
        let rx = Arc::new(Mutex::new(c.1));
        for _i in 0..n {
            let ctr = mpsc::channel();
            w.push((Worker::new(rx.clone(), Arc::new(Mutex::new(ctr.1))), ctr.0));

        }

        Self {
            handles: w,
            tx: c.0,
            rx: rx.clone(),
        }

        // todo!()
    }

    pub fn push(&mut self) -> Null{
        if self.handles.len() == MAX_THREADS {
            return Err(Box::new(CustomError::Full));
        }
        let ctr = mpsc::channel();
        self.handles.push((Worker::new(self.rx.clone(), Arc::new(Mutex::new(ctr.1))), ctr.0));

        Ok(())
    }

    pub fn pop(&mut self) -> Null{
        let j = self.handles.pop().unwrap();
        j.1.send(ControlPrompt::Terminate)?;
        j.0.retire();
        Ok(())
    }

    pub fn task(&self, t: TaskPair) -> Null {
        Ok(self.tx.send(t)?)
    }

    pub fn retire(self) -> Null{
        for t in self.handles {
            t.1.send(ControlPrompt::Terminate)?;
            t.0.retire();

        }

        Ok(())
    }
}

pub struct TaskPair {
    func: Job,
    data: TcpStream,
}

impl TaskPair {
    pub fn new<F>(data: TcpStream, func: F) -> Self
    where F: Fn(Vec<u8>) -> Res<Vec<u8>> + Send + 'static
    {
        Self { func: Box::new(func), data }
    }
}

struct Worker {
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(rx: Work, ctrl: Control) -> Self {
        Self {
            thread: spawn(move || loop {
                // check for control
                let ctr = ctrl.lock().unwrap().try_recv();
                if let Ok(pr) = ctr {
                    match pr {
                        ControlPrompt::Terminate => {break;}
                    }
                }
                // acquire the taskpair
                let mut tp = rx.lock().unwrap().recv().unwrap();

                // set up a buffered reader
                let mut rdr = BufReader::new(&mut tp.data);
                // actually read
                let v = rdr.fill_buf().unwrap_or(&[]).to_vec();
                // syntax things.
                let f = tp.func;
                // compute
                let out = f(v).unwrap_or(vec![]);
                // write back
                tp.data.write_all(&out).unwrap();
            }),
            // ctrl
        }
    }

    fn retire(self) {
        self.thread.join().unwrap();
        // self.thread.thread()
    }
}
