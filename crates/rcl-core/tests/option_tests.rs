use rcl_core::option::OptionInstance;
use std::sync::{Arc, Mutex};

#[test]
fn test_option_basic_lifecycle() {
    let mut opt = OptionInstance::new("render_distance", 12);
    assert_eq!(*opt.get(), 12);
    assert!(!opt.is_dirty());

    opt.set_pending(16).unwrap();
    assert_eq!(*opt.get(), 16);
    assert!(opt.is_dirty());

    opt.apply();
    assert_eq!(*opt.get(), 16);
    assert!(!opt.is_dirty());
}

#[test]
fn test_option_validator() {
    let mut opt = OptionInstance::new("fov", 70);
    opt.add_validator(|&val| {
        if (30..=110).contains(&val) {
            Ok(())
        } else {
            Err("FOV must be between 30 and 110".to_string())
        }
    });

    assert!(opt.set_pending(90).is_ok());
    assert!(opt.set_pending(120).is_err());
    assert_eq!(*opt.get(), 90);
}

#[test]
fn test_option_reset_and_discard() {
    let mut opt = OptionInstance::new("fps_limit", 60);
    opt.set_pending(144).unwrap();
    assert_eq!(*opt.get(), 144);

    opt.discard();
    assert_eq!(*opt.get(), 60);

    opt.set_pending(144).unwrap();
    opt.apply();
    assert_eq!(*opt.get(), 144);

    opt.reset_to_default();
    assert_eq!(*opt.get(), 60);
    assert!(opt.is_dirty());
}

#[test]
fn test_option_listener() {
    let changed_value = Arc::new(Mutex::new(0));
    let changed_clone = Arc::clone(&changed_value);

    let mut opt = OptionInstance::new("audio_volume", 50);
    opt.add_listener(move |&val| {
        let mut lock = changed_clone.lock().unwrap();
        *lock = val;
    });

    opt.set_pending(80).unwrap();
    assert_eq!(*changed_value.lock().unwrap(), 0);

    opt.apply();
    assert_eq!(*changed_value.lock().unwrap(), 80);
}
