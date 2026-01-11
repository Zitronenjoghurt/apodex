use tokio::sync::{mpsc, watch};

pub struct TaskHandler<T: Send + 'static> {
    result_rx: Option<mpsc::Receiver<T>>,
    status_rx: Option<watch::Receiver<String>>,
    abort: Option<tokio::task::AbortHandle>,
}

impl<T: Send + 'static> Default for TaskHandler<T> {
    fn default() -> Self {
        Self {
            result_rx: None,
            status_rx: None,
            abort: None,
        }
    }
}

impl<T: Send + 'static> TaskHandler<T> {
    pub fn spawn<F, Fut>(&mut self, handle: &tokio::runtime::Handle, f: F)
    where
        F: FnOnce(TaskContext) -> Fut + Send + 'static,
        Fut: Future<Output = T> + Send,
    {
        self.abort();

        let (result_tx, result_rx) = mpsc::channel(1);
        let (status_tx, status_rx) = watch::channel(String::new());

        let ctx = TaskContext { status: status_tx };

        let join_handle = handle.spawn(async move {
            let result = f(ctx).await;
            let _ = result_tx.send(result).await;
        });

        self.result_rx = Some(result_rx);
        self.status_rx = Some(status_rx);
        self.abort = Some(join_handle.abort_handle());
    }

    pub fn abort(&mut self) {
        if let Some(handle) = self.abort.take() {
            handle.abort();
        }
        self.result_rx = None;
        self.status_rx = None;
    }

    pub fn poll(&mut self) -> Option<T> {
        let rx = self.result_rx.as_mut()?;
        match rx.try_recv() {
            Ok(result) => {
                self.abort = None;
                self.result_rx = None;
                self.status_rx = None;
                Some(result)
            }
            Err(mpsc::error::TryRecvError::Empty) => None,
            Err(mpsc::error::TryRecvError::Disconnected) => {
                self.abort();
                None
            }
        }
    }

    pub fn is_busy(&self) -> bool {
        self.result_rx.is_some()
    }

    pub fn status(&self) -> Option<String> {
        let status = self.status_rx.as_ref()?.borrow();
        if status.is_empty() {
            None
        } else {
            Some(status.clone())
        }
    }
}

#[derive(Clone)]
pub struct TaskContext {
    status: watch::Sender<String>,
}

impl TaskContext {
    pub fn set_status(&self, status: impl Into<String>) {
        let _ = self.status.send(status.into());
    }
}
