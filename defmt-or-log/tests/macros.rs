#[test]
fn run_macros() {
    defmt_or_log::assert!(true);
    defmt_or_log::assert!(true, "hello");
    defmt_or_log::assert!(true, "hello {} {}", 3, 4);
    defmt_or_log::assert_eq!(1, 1);
    defmt_or_log::assert_eq!(1, 1, "hello");
    defmt_or_log::assert_eq!(1, 1, "hello {} {}", 3, 4,);
    defmt_or_log::assert_ne!(2, 1);
    defmt_or_log::debug_assert!(true);
    defmt_or_log::debug_assert_eq!(1, 1);
    defmt_or_log::debug_assert_ne!(2, 1);

    defmt_or_log::trace!("Hello");
    defmt_or_log::trace!("Hello {}", "world");
    defmt_or_log::trace!("Hello {} {}", "cruel", "world");
    defmt_or_log::trace!("Hello {} {}", "cruel", "world",);

    defmt_or_log::debug!("Hello");
    defmt_or_log::debug!("Hello {}", "world");
    defmt_or_log::debug!("Hello {} {}", "cruel", "world");
    defmt_or_log::debug!("Hello {} {}", "cruel", "world",);

    defmt_or_log::info!("Hello");
    defmt_or_log::info!("Hello {}", "world");
    defmt_or_log::info!("Hello {} {}", "cruel", "world");
    defmt_or_log::info!("Hello {} {}", "cruel", "world",);

    defmt_or_log::warn!("Hello");
    defmt_or_log::warn!("Hello {}", "world");
    defmt_or_log::warn!("Hello {} {}", "cruel", "world");
    defmt_or_log::warn!("Hello {} {}", "cruel", "world",);

    defmt_or_log::error!("Hello");
    defmt_or_log::error!("Hello {}", "world");
    defmt_or_log::error!("Hello {} {}", "cruel", "world");
    defmt_or_log::error!("Hello {} {}", "cruel", "world",);
}

#[test]
fn intern() {
    defmt_or_log::info!("{}", defmt_or_log::intern!("hello"));
}

#[test]
fn unwrap() {
    defmt_or_log::unwrap!(Some(5));
    defmt_or_log::unwrap!(Ok::<_, ()>(5));
}

#[test]
fn expect() {
    defmt_or_log::expect!(Some(5), "should be something");
    defmt_or_log::unwrap!(Ok::<_, ()>(5), "should be ok");
}

#[test]
#[should_panic]
fn panic_no_msg() {
    defmt_or_log::panic!();
}

#[test]
#[should_panic]
fn panic_simple_msg() {
    defmt_or_log::panic!("Hello");
}

#[test]
#[should_panic]
fn panic_formatting() {
    defmt_or_log::panic!("Hello {} {}", 1, 2);
}

#[test]
#[should_panic]
fn unreachable_no_msg() {
    defmt_or_log::unreachable!();
}

#[test]
#[should_panic]
fn unreachable_simple_msg() {
    defmt_or_log::unreachable!("Hello");
}

#[test]
#[should_panic]
fn unreachable_formatting() {
    defmt_or_log::unreachable!("Hello {} {}", 1, 2);
}

#[test]
#[should_panic]
fn todo_no_msg() {
    defmt_or_log::todo!();
}

#[test]
#[should_panic]
fn todo_simple_msg() {
    defmt_or_log::todo!("Hello");
}

#[test]
#[should_panic]
fn todo_formatting() {
    defmt_or_log::todo!("Hello {} {}", 1, 2);
}
