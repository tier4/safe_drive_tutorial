use safe_drive::{
    allocator::ALLOCATOR, context::Context, error::DynError, msg::common_interfaces::std_msgs,
};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    rc::Rc,
    time::Duration,
};

static mut MY_ALLOCATOR: memac::Allocator<memac::buddy::Buddy32M> = memac::Allocator::new();
const MEMSIZE_32M: usize = 32 * 1024 * 1024; // 32MiB

const NUM: usize = 1024 * 1024;

fn main() -> Result<(), DynError> {
    let mut v = vec![None; NUM];

    let param = libc::sched_param { sched_priority: 1 };
    let result = unsafe { libc::sched_setscheduler(0, libc::SCHED_FIFO, &param) };
    if result != 0 {
        unsafe {
            libc::perror(b"failed sched_setscheduler".as_ptr() as *const _);
        }
    }

    unsafe {
        let layout = Layout::from_size_align(MEMSIZE_32M, memac::ALIGNMENT).unwrap();
        let heap_start = System.alloc(layout);

        assert!(!heap_start.is_null());

        // Initialize a custom allocator.
        MY_ALLOCATOR.init(heap_start as usize, MEMSIZE_32M);

        // Pass MY_ALLOCATOR to safe_drive's allocator.
        ALLOCATOR.init(&MY_ALLOCATOR, heap_start as usize, MEMSIZE_32M);
    }

    // Create a context.
    let ctx = Context::new()?;

    // Create a node.
    let node = ctx.create_node("requester", None, Default::default())?;

    // Create a subscriber.
    let subscriber = node.create_subscriber::<std_msgs::msg::Empty>("custom_alloc_topic2", None)?;

    // Create a publisher.
    let publisher = node.create_publisher::<std_msgs::msg::Empty>("custom_alloc_topic1", None)?;
    let publisher1 = Rc::new(publisher);
    let publisher2 = publisher1.clone();

    // Create a selector.
    let mut selector = ctx.create_selector()?;

    let mut start = std::time::Instant::now();
    publisher2.send(&std_msgs::msg::Empty::new().unwrap())?;

    // Add a callback function.
    let mut cnt = 0;
    let flag1 = Rc::new(Cell::new(true));
    let flag2 = flag1.clone();
    selector.add_subscriber(
        subscriber,
        Box::new(move |msg| {
            let t = start.elapsed();

            v[cnt] = Some(t);
            cnt += 1;

            if cnt >= NUM {
                flag1.set(false);

                for result in v.iter() {
                    if let Some(x) = result {
                        println!("{}", x.as_nanos());
                    }
                }
            }

            start = std::time::Instant::now();
            publisher1.send(&msg).unwrap();
        }),
    );

    while flag2.get() {
        if !selector.wait_timeout(Duration::from_millis(1000))? {
            // Timed out.
            publisher2.send(&std_msgs::msg::Empty::new().unwrap())?;
        }
    }

    Ok(())
}
