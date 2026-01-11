use crate::runtime::RuntimeEvent;
use std::cell::RefCell;

#[derive(Debug)]
pub enum AppAction {
    RuntimeEvent(RuntimeEvent),
}

#[derive(Default)]
pub struct AppActions {
    queue: RefCell<Vec<AppAction>>,
}

impl AppActions {
    pub fn take_actions(&self) -> Vec<AppAction> {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.drain(..).collect()
        } else {
            vec![]
        }
    }

    pub fn push_action(&self, action: AppAction) {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.push(action);
        }
    }

    pub fn runtime_event(&self, event: RuntimeEvent) {
        self.push_action(AppAction::RuntimeEvent(event));
    }
}
