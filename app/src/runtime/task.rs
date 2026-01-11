use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub struct TaskHandler<T: Send + 'static> {
    task: Option<Task<T>>,
}

impl<T: Send + 'static> Default for TaskHandler<T> {
    fn default() -> Self {
        Self { task: None }
    }
}

impl<T: Send + 'static> TaskHandler<T> {
    pub fn spawn(&mut self, f: impl FnOnce(TaskContext) -> T + Send + 'static) {
        self.task = Some(Task::spawn(f));
    }

    pub fn stop(&mut self) {
        self.task.take();
    }

    pub fn poll(&mut self) -> Option<T> {
        let result = self.task.as_mut()?.poll();
        if result.is_some() {
            self.stop();
        }
        result
    }

    pub fn busy(&self) -> bool {
        self.task.is_some()
    }

    pub fn context(&self) -> Option<&TaskContext> {
        self.task.as_ref().map(|task| task.context())
    }
}

#[derive(Clone)]
pub struct TaskContext {
    status: Arc<Mutex<String>>,
}

impl Default for TaskContext {
    fn default() -> Self {
        Self {
            status: Arc::new(Mutex::new("".to_string())),
        }
    }
}

impl TaskContext {
    pub fn get_status(&self) -> String {
        self.status.lock().unwrap().clone()
    }

    pub fn set_status(&self, status: impl Into<String>) {
        *self.status.lock().unwrap() = status.into();
    }
}

pub struct Task<T: Send + 'static> {
    rx: Receiver<T>,
    context: TaskContext,
    _handle: JoinHandle<()>,
}

impl<T: Send + 'static> Task<T> {
    pub fn spawn(f: impl FnOnce(TaskContext) -> T + Send + 'static) -> Self {
        let context = TaskContext::default();
        let context_clone = context.clone();
        let (tx, rx) = std::sync::mpsc::channel();

        let handle = std::thread::spawn(move || {
            let result = f(context_clone);
            let _ = tx.send(result);
        });

        Self {
            rx,
            context,
            _handle: handle,
        }
    }

    pub fn poll(&mut self) -> Option<T> {
        self.rx.try_recv().ok()
    }

    pub fn context(&self) -> &TaskContext {
        &self.context
    }
}
