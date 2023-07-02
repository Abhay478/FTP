use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::{mpsc, Arc, Mutex},
    thread::{spawn, JoinHandle},
    vec, fs, fmt::Display,
};
pub type Res<T> = Result<T, Box<dyn Error>>;
pub type Null = Res<()>;
pub type Job = Box<dyn Fn(Vec<u8>) -> Res<Vec<u8>> + Send + 'static>;

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

    pub fn task(&self, t: TaskPair) -> Null {
        Ok(self.tx.send(t)?)
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

#[derive(Debug)]
pub enum CustomError {
    Absent,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Absent => {write!(f, "Primitive absent.")?;}
        }
        Ok(())
    }
}

impl Error for CustomError {}

pub enum Primitives {
    FTP,
}

impl Primitives {
    pub fn get_job(&self) -> Res<Job> {
        match &self {
            Self::FTP => Ok(Box::new(ftp)),
        }
    }
}

fn ftp(v: Vec<u8>) -> Res<Vec<u8>> {
    let path = v.iter().map(|u| *u as char).collect::<String>();
    println!("FTP request for {path} received.");
    match fs::read(path) {
        Ok(x) => Ok(x),
        Err(e) => Ok(e.to_string().as_bytes().to_vec()),
    }
}