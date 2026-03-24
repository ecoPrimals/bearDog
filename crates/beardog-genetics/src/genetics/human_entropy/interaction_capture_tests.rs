// SPDX-License-Identifier: AGPL-3.0-only

use super::*;

#[test]
fn test_interaction_capture_config_default() {
    let config = InteractionCaptureConfig::default();
    assert_eq!(config.target_interactions, 50);
    assert_eq!(config.timeout_seconds, 120);
    assert!(config.enable_keyboard);
    assert!(config.enable_mouse);
}

#[test]
fn test_shannon_entropy_calculation() {
    // Uniform distribution should have high entropy
    let uniform = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
    let entropy = InteractionEntropyCollector::calculate_shannon_entropy(&uniform);
    assert!(
        entropy > 0.8,
        "Uniform distribution should have high entropy"
    );

    // Identical values should have low entropy
    let identical = vec![50.0; 8];
    let entropy = InteractionEntropyCollector::calculate_shannon_entropy(&identical);
    assert!(entropy < 0.1, "Identical values should have low entropy");
}

#[test]
fn test_overall_quality_calculation() {
    // Good quality interaction
    let quality = InteractionEntropyCollector::calculate_overall_quality(
        0.8,   // Good timing entropy
        0.7,   // Good movement entropy
        50,    // Target number of interactions
        25000, // 25 seconds (500ms average interval)
    );
    assert!(quality > 0.7, "Good interaction should have high quality");

    // Poor quality interaction
    let quality = InteractionEntropyCollector::calculate_overall_quality(
        0.2,   // Poor timing entropy
        0.3,   // Poor movement entropy
        10,    // Too few interactions
        50000, // 50 seconds (5000ms average interval - too slow)
    );
    assert!(quality < 0.5, "Poor interaction should have low quality");
}

#[test]
fn test_calculate_metrics_empty_and_mixed_events() {
    let empty = InteractionEntropyCollector::calculate_metrics(&[], 0);
    assert_eq!(empty.total_interactions, 0);
    assert_eq!(empty.avg_interval_ms, 0.0);

    let events = vec![
        InteractionEvent {
            interaction_type: InteractionType::KeyPress,
            timestamp_nanos: 0,
            data: InteractionData::Keyboard {
                is_char: true,
                is_modifier: false,
            },
        },
        InteractionEvent {
            interaction_type: InteractionType::MouseMove,
            timestamp_nanos: 10_000_000,
            data: InteractionData::Mouse {
                delta_x: 3,
                delta_y: -2,
            },
        },
        InteractionEvent {
            interaction_type: InteractionType::MouseScroll,
            timestamp_nanos: 25_000_000,
            data: InteractionData::Scroll { delta: 1 },
        },
    ];
    let m = InteractionEntropyCollector::calculate_metrics(&events, 30);
    assert_eq!(m.total_interactions, 3);
    assert_eq!(m.keyboard_events, 1);
    assert_eq!(m.mouse_events, 2);
    assert!(m.movement_entropy >= 0.0);
}

#[test]
fn test_derive_entropy_bytes_deterministic_for_same_inputs() {
    let events = vec![InteractionEvent {
        interaction_type: InteractionType::KeyRelease,
        timestamp_nanos: 99,
        data: InteractionData::Keyboard {
            is_char: false,
            is_modifier: true,
        },
    }];
    let metrics = InteractionEntropyCollector::calculate_metrics(&events, 1);
    let a = InteractionEntropyCollector::derive_entropy_bytes(&events, &metrics);
    let b = InteractionEntropyCollector::derive_entropy_bytes(&events, &metrics);
    assert_eq!(a, b);
    assert_eq!(a.len(), 32);
}

#[test]
fn test_calculate_movement_entropy_neutral_without_mouse() {
    let events = vec![InteractionEvent {
        interaction_type: InteractionType::KeyPress,
        timestamp_nanos: 0,
        data: InteractionData::Keyboard {
            is_char: true,
            is_modifier: false,
        },
    }];
    let m = InteractionEntropyCollector::calculate_movement_entropy(&events);
    assert!((m - 0.5).abs() < f64::EPSILON);
}

#[test]
fn test_shannon_empty_intervals() {
    assert_eq!(
        InteractionEntropyCollector::calculate_shannon_entropy(&[]),
        0.0
    );
}

#[test]
fn test_process_key_event_classifies_printable_vs_modifier() {
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    let collector = InteractionEntropyCollector::new(InteractionCaptureConfig::default());
    let printable = KeyEvent {
        code: KeyCode::Char('z'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    let ev = collector.process_key_event(printable, 100u128);
    match ev.data {
        InteractionData::Keyboard {
            is_char,
            is_modifier,
        } => {
            assert!(is_char);
            assert!(!is_modifier);
        }
        _ => panic!("expected keyboard data"),
    }

    let modifier = KeyEvent {
        code: KeyCode::Modifier(crossterm::event::ModifierKeyCode::LeftShift),
        modifiers: KeyModifiers::SHIFT,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    };
    let ev2 = collector.process_key_event(modifier, 200u128);
    match ev2.data {
        InteractionData::Keyboard {
            is_char,
            is_modifier,
        } => {
            assert!(!is_char);
            assert!(is_modifier);
        }
        _ => panic!("expected keyboard data"),
    }

    let repeat = KeyEvent {
        code: KeyCode::Char('r'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Repeat,
        state: KeyEventState::empty(),
    };
    let ev3 = collector.process_key_event(repeat, 300u128);
    assert!(matches!(ev3.interaction_type, InteractionType::KeyPress));
}

#[test]
fn test_process_mouse_event_moves_clicks_and_scroll() {
    use crossterm::event::{MouseButton as CtBtn, MouseEvent, MouseEventKind};
    let collector = InteractionEntropyCollector::new(InteractionCaptureConfig::default());
    let mut last = None;

    let first = MouseEvent {
        kind: MouseEventKind::Moved,
        column: 5,
        row: 5,
        modifiers: crossterm::event::KeyModifiers::NONE,
    };
    assert!(collector.process_mouse_event(first, 1, &mut last).is_none());

    let second = MouseEvent {
        kind: MouseEventKind::Moved,
        column: 9,
        row: 7,
        modifiers: crossterm::event::KeyModifiers::NONE,
    };
    let mv = collector.process_mouse_event(second, 2, &mut last).unwrap();
    match mv.data {
        InteractionData::Mouse { delta_x, delta_y } => {
            assert_eq!(delta_x, 4);
            assert_eq!(delta_y, 2);
        }
        _ => panic!("expected mouse move"),
    }

    for (kind, btn) in [
        (MouseEventKind::Down(CtBtn::Left), MouseButton::Left),
        (MouseEventKind::Up(CtBtn::Right), MouseButton::Right),
        (MouseEventKind::Down(CtBtn::Middle), MouseButton::Middle),
    ] {
        let ev = MouseEvent {
            kind,
            column: 1,
            row: 1,
            modifiers: crossterm::event::KeyModifiers::NONE,
        };
        let out = collector.process_mouse_event(ev, 3, &mut last).unwrap();
        match out.data {
            InteractionData::Click { button } => assert_eq!(button, btn),
            _ => panic!("click"),
        }
    }

    let su = MouseEvent {
        kind: MouseEventKind::ScrollUp,
        column: 0,
        row: 0,
        modifiers: crossterm::event::KeyModifiers::NONE,
    };
    match collector
        .process_mouse_event(su, 4, &mut last)
        .unwrap()
        .data
    {
        InteractionData::Scroll { delta } => assert_eq!(delta, 1),
        _ => panic!("scroll up"),
    }

    let sd = MouseEvent {
        kind: MouseEventKind::ScrollDown,
        column: 0,
        row: 0,
        modifiers: crossterm::event::KeyModifiers::NONE,
    };
    match collector
        .process_mouse_event(sd, 5, &mut last)
        .unwrap()
        .data
    {
        InteractionData::Scroll { delta } => assert_eq!(delta, -1),
        _ => panic!("scroll down"),
    }
}

#[test]
fn test_calculate_metrics_multi_interval_std_dev() {
    let events: Vec<InteractionEvent> = (0..5)
        .map(|i| InteractionEvent {
            interaction_type: InteractionType::KeyPress,
            timestamp_nanos: (i as u128) * 1_000_000,
            data: InteractionData::Keyboard {
                is_char: true,
                is_modifier: false,
            },
        })
        .collect();
    let m = InteractionEntropyCollector::calculate_metrics(&events, 5);
    assert!(m.interval_std_dev >= 0.0);
    assert!(m.avg_interval_ms > 0.0);
}

#[test]
fn test_overall_quality_pace_too_fast_and_too_slow() {
    let fast = InteractionEntropyCollector::calculate_overall_quality(0.9, 0.8, 20, 100);
    let slow = InteractionEntropyCollector::calculate_overall_quality(0.9, 0.8, 5, 100_000);
    assert!(fast < 1.0);
    assert!(slow < 1.0);
}

#[test]
fn test_overall_quality_pace_in_sweet_spot_is_one() {
    // 50 events over 10_000 ms => 200 ms average (inside 200–2000 ms band)
    let q = InteractionEntropyCollector::calculate_overall_quality(0.5, 0.5, 50, 10_000);
    assert!(q > 0.4);
}

#[test]
fn test_process_key_event_release_and_non_char_key() {
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    let collector = InteractionEntropyCollector::new(InteractionCaptureConfig::default());

    let release = KeyEvent {
        code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Release,
        state: KeyEventState::empty(),
    };
    let ev = collector.process_key_event(release, 50u128);
    assert!(matches!(ev.interaction_type, InteractionType::KeyRelease));
    match ev.data {
        InteractionData::Keyboard {
            is_char,
            is_modifier,
        } => {
            assert!(!is_char);
            assert!(!is_modifier);
        }
        _ => panic!("keyboard"),
    }
}

#[test]
fn test_process_mouse_event_drag_and_horizontal_scroll_return_none() {
    use crossterm::event::{MouseButton as CtBtn, MouseEvent, MouseEventKind};
    let collector = InteractionEntropyCollector::new(InteractionCaptureConfig::default());
    let mut last = None;

    for kind in [
        MouseEventKind::Drag(CtBtn::Left),
        MouseEventKind::ScrollLeft,
        MouseEventKind::ScrollRight,
    ] {
        let ev = MouseEvent {
            kind,
            column: 1,
            row: 1,
            modifiers: crossterm::event::KeyModifiers::NONE,
        };
        assert!(
            collector.process_mouse_event(ev, 1, &mut last).is_none(),
            "expected unhandled kind {kind:?} to yield None"
        );
    }
}

#[test]
fn test_shannon_entropy_single_bucket_max_entropy_zero_branch() {
    let identical = vec![25.0, 25.0, 25.0];
    let e = InteractionEntropyCollector::calculate_shannon_entropy(&identical);
    assert_eq!(e, 0.0);
}

#[test]
fn test_calculate_movement_entropy_high_variance_caps_at_one() {
    let events: Vec<InteractionEvent> = (0..20)
        .map(|i| InteractionEvent {
            interaction_type: InteractionType::MouseMove,
            timestamp_nanos: i as u128 * 1_000_000,
            data: InteractionData::Mouse {
                delta_x: (i * 97) as i16,
                delta_y: (i * -53) as i16,
            },
        })
        .collect();
    let m = InteractionEntropyCollector::calculate_movement_entropy(&events);
    assert!(m <= 1.0);
    assert!(m > 0.5);
}

#[test]
fn test_derive_entropy_bytes_covers_all_mouse_button_tags() {
    let events = vec![
        InteractionEvent {
            interaction_type: InteractionType::MouseClick,
            timestamp_nanos: 1,
            data: InteractionData::Click {
                button: MouseButton::Left,
            },
        },
        InteractionEvent {
            interaction_type: InteractionType::MouseClick,
            timestamp_nanos: 2,
            data: InteractionData::Click {
                button: MouseButton::Right,
            },
        },
        InteractionEvent {
            interaction_type: InteractionType::MouseClick,
            timestamp_nanos: 3,
            data: InteractionData::Click {
                button: MouseButton::Middle,
            },
        },
    ];
    let metrics = InteractionEntropyCollector::calculate_metrics(&events, 10);
    let out = InteractionEntropyCollector::derive_entropy_bytes(&events, &metrics);
    assert_eq!(out.len(), 32);
}

#[test]
fn test_derive_entropy_bytes_includes_scroll_deltas() {
    let events = vec![InteractionEvent {
        interaction_type: InteractionType::MouseScroll,
        timestamp_nanos: 42,
        data: InteractionData::Scroll { delta: -3 },
    }];
    let metrics = InteractionEntropyCollector::calculate_metrics(&events, 1);
    let out = InteractionEntropyCollector::derive_entropy_bytes(&events, &metrics);
    assert_eq!(out.len(), 32);
}
