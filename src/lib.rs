use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::{mpsc, Arc, Mutex},
    thread::{spawn, JoinHandle},
    vec,
};
pub type MyResult<T> = Result<T, Box<dyn Error>>;
pub type MyNull = MyResult<()>;

pub struct Threadpool {
    handles: Vec<Worker>,
    tx: mpsc::Sender<TaskPair>,
}

impl Threadpool {
    pub fn new(n: usize) -> Self {
        let mut w = vec![];
        let c = mpsc::channel();
        let rx = Arc::new(Mutex::new(c.1));
        for _i in 0..n {
            w.push(Worker::new(rx.clone()));
        }

        Self {
            handles: w,
            tx: c.0,
        }

        // todo!()
    }

    pub fn task(&self, t: TaskPair) -> MyNull {
        Ok(self.tx.send(t)?)
    }
}

pub struct TaskPair {
    func: Box<dyn Fn(Vec<u8>) -> MyResult<Vec<u8>> + Send + 'static>,
    data: TcpStream,
}

impl TaskPair {
    pub fn new<F>(data: TcpStream, func: F) -> Self
    where F: Fn(Vec<u8>) -> MyResult<Vec<u8>> + Send + 'static
    {
        Self { func: Box::new(func), data }
    }
}

struct Worker {
    thread: JoinHandle<()>,
}

type Work = Arc<Mutex<mpsc::Receiver<TaskPair>>>;
impl Worker {
    fn new(rx: Work) -> Self {
        Self {
            thread: spawn(move || loop {
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
        }
    }
}
