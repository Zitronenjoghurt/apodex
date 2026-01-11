use crate::runtime::RuntimeEvent;
use crate::windows::WindowId;
use apodex::date::ApodDate;
use std::cell::RefCell;

#[derive(Debug)]
pub enum AppAction {
    DetailsSelectDate(ApodDate),
    OpenAndFocusWindow(WindowId),
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

    pub fn details_select_date(&self, date: ApodDate) {
        self.push_action(AppAction::DetailsSelectDate(date));
    }

    pub fn open_and_focus_window(&self, window_id: WindowId) {
        self.push_action(AppAction::OpenAndFocusWindow(window_id));
    }

    pub fn runtime_event(&self, event: RuntimeEvent) {
        self.push_action(AppAction::RuntimeEvent(event));
    }
}
