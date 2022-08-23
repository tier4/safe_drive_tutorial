use safe_drive::{context::Context, error::DynError, logger::Logger, pr_info};
use std::{cell::RefCell, collections::VecDeque, rc::Rc, time::Duration};

pub fn main() -> Result<(), DynError> {
    // Create a context, a publisher, and a logger.
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let logger = Rc::new(Logger::new("one-shot timer example"));

    let queue = Rc::new(RefCell::new(VecDeque::new()));

    // Add a one-shot timer.
    let queue1 = queue.clone();
    selector.add_timer(
        Duration::from_secs(2),
        Box::new(move || {
            pr_info!(logger, "fired!");

            // Insert a timer to the queue.
            let mut q = queue1.borrow_mut();
            let logger1 = logger.clone();
            q.push_back((
                Duration::from_secs(2),
                (Box::new(move || pr_info!(logger1, "fired! again!"))),
            ));
        }),
    );

    // Spin.
    loop {
        {
            // Set timers.
            let mut q = queue.borrow_mut();
            while let Some((dur, f)) = q.pop_front() {
                selector.add_timer(dur, f);
            }
        }

        selector.wait()?;
    }
}
