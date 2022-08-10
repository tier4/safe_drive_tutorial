use my_interfaces_rs::my_interfaces;
use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    msg::{I32Seq, RosStringSeq},
    pr_info,
};
use std::time::Duration;

fn main() -> Result<(), DynError> {
    // Create a context.
    let ctx = Context::new()?;

    // Create a node.
    let node = ctx.create_node("talker", None, Default::default())?;

    // Create a publisher.
    let publisher = node.create_publisher::<my_interfaces::msg::MyMsgs>("my_topic", None)?;

    // Create a logger.
    let logger = Logger::new("talker");

    // Create a message
    let my_msg1 = create_message()?;
    let my_msg2 = create_message()?;

    let mut my_msgs = my_interfaces::msg::MyMsgs::new().ok_or("failed to create MyMsgs")?;
    my_msgs.msg1 = my_msg1;
    my_msgs.msg2 = my_msg2;

    loop {
        pr_info!(logger, "send: {:?}", my_msgs); // Print log.

        // Send a message.
        publisher.send(&my_msgs)?;

        std::thread::sleep(Duration::from_secs(1));
    }
}

fn create_message() -> Result<my_interfaces::msg::MyMsg, DynError> {
    let mut my_msg = my_interfaces::msg::MyMsg::new().unwrap();

    my_msg.integer_value = 10;

    // int32[5] five_integers_array
    my_msg.five_integers_array[0] = 11;
    my_msg.five_integers_array[1] = 13;
    my_msg.five_integers_array[2] = 49;
    my_msg.five_integers_array[3] = 55;
    my_msg.five_integers_array[4] = 19;

    // int32[] unbounded_integer_array
    let mut msgs = I32Seq::new(3).unwrap();
    let ref_msgs = msgs.as_slice_mut();
    ref_msgs[0] = 6;
    ref_msgs[1] = 7;
    ref_msgs[2] = 8;
    my_msg.unbounded_integer_array = msgs;

    // int32[<=5] up_to_five_integers_array
    let mut msgs = I32Seq::new(2).unwrap();
    let ref_msgs = msgs.as_slice_mut();
    ref_msgs[0] = 2;
    ref_msgs[1] = 3;
    my_msg.up_to_five_integers_array = msgs;

    Ok(my_msg)
}

fn _create_message_str() -> Result<my_interfaces::msg::MyMsgStr, DynError> {
    let mut my_msg = my_interfaces::msg::MyMsgStr::new().unwrap();

    // string message
    my_msg.message.assign("Hello, World!");

    // string[2] static_array_str
    my_msg.static_array_str[0].assign("static array 0");
    my_msg.static_array_str[1].assign("static array 1");

    // string[] dynamic_array_str
    let mut msgs = RosStringSeq::new(3).ok_or("failed to create string")?;
    let ref_msgs = msgs.as_slice_mut();
    ref_msgs[0].assign("dynamic array 0");
    ref_msgs[1].assign("dynamic array 1");
    ref_msgs[2].assign("dynamic array 2");
    my_msg.dynamic_array_str = msgs;

    // string[<=3] bounded_array_str
    let mut msgs = RosStringSeq::new(2).ok_or("failed to create string")?;
    let ref_msgs = msgs.as_slice_mut();
    ref_msgs[0].assign("bounded array 0");
    ref_msgs[1].assign("bounded array 1");
    my_msg.bounded_array_str = msgs;

    // string<=10 bounded_str
    my_msg.bounded_str.assign("Hello!");

    // string<=10[2] static_array_bounded_str
    my_msg.static_array_bounded_str[0].assign("msg1");
    my_msg.static_array_bounded_str[1].assign("msg2");

    // string<=10[] dynamic_array_bounded_str
    let mut msgs = RosStringSeq::new(3).ok_or("failed to create string")?;
    let ref_msgs = msgs.as_slice_mut();
    ref_msgs[0].assign("msg3");
    ref_msgs[1].assign("msg4");
    ref_msgs[2].assign("msg5");
    my_msg.dynamic_array_bounded_str = msgs;

    // string<=10[<=3] bounded_array_bounded_str
    let mut msgs = RosStringSeq::new(2).ok_or("failed to create string")?;
    let ref_msgs = msgs.as_slice_mut();
    ref_msgs[0].assign("msg3");
    ref_msgs[1].assign("msg5");
    my_msg.bounded_array_bounded_str = msgs;

    Ok(my_msg)
}
