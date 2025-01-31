use std::thread::sleep;
use async_trait::async_trait;
use crate::core::{BookedAppointmentData, Notification, Notifier, LoggingNotifier};


pub struct NotifierTrigger {
    notifier: Box<dyn Notifier>
}

impl NotifierTrigger {
    pub fn new_logging_notifier_trigger() -> NotifierTrigger {
        NotifierTrigger::new(Box::new(LoggingNotifier::new()))
    }
    fn new(notifier: Box<dyn Notifier>) -> NotifierTrigger {
        NotifierTrigger { notifier }
    }

}
