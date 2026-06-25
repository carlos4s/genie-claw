use genie_core::tools::home_action::{HOME_CONTROL_ACTIONS, canonicalize_household_action};
use genie_core::tools::quick::route;

const ROUTER_EMITTED_ACTIONS: &[&str] = &[
    "activate",
    "activate_until_5pm",
    "allow_10_to_10_20",
    "allow_mom_only",
    "apply_scene",
    "arm",
    "block_until_math_done",
    "check_and_alert",
    "check_and_vent",
    "clean",
    "cool_down",
    "create",
    "create_threshold_10",
    "create_until_21",
    "cut_power_and_vent",
    "enable",
    "hold",
    "lock_except",
    "mute_for_practice",
    "open",
    "pause",
    "pause_until_dinner",
    "play_low_volume",
    "privacy_20",
    "remote_start",
    "run",
    "schedule",
    "schedule_after_21",
    "schedule_gradual_blinds",
    "schedule_on_alarm",
    "schedule_on_arrival",
    "schedule_pulse",
    "send_destination",
    "set_color_blue",
    "set_for_tomorrow",
    "set_level",
    "set_preset",
    "set_volume",
    "show",
    "show_agenda",
    "show_guest_card",
    "shut_water_zone",
    "start",
    "start_video_call",
    "test",
    "turn_off",
    "turn_off_except",
    "unlock",
    "verify_and_alert",
    "warm_for_minutes",
];

#[test]
fn every_router_emitted_action_canonicalizes_to_a_dispatch_action() {
    for &action in ROUTER_EMITTED_ACTIONS {
        let (canon, _) = canonicalize_household_action(action, None);
        assert!(
            HOME_CONTROL_ACTIONS.contains(&canon),
            "action '{action}' canonicalized to '{canon}', which is not a valid home_control action"
        );
    }
}

#[test]
fn valid_actions_and_synonyms_pass_through() {
    assert_eq!(
        canonicalize_household_action("turn_off", None).0,
        "turn_off"
    );
    assert_eq!(canonicalize_household_action("open", None).0, "open");
    assert_eq!(
        canonicalize_household_action("activate", None).0,
        "activate"
    );
    assert_eq!(canonicalize_household_action("enable", None).0, "turn_on");
}

#[test]
fn level_maps_to_set_brightness_and_keeps_value() {
    let (action, value) = canonicalize_household_action("set_level", Some(90.0));
    assert_eq!(action, "set_brightness");
    assert_eq!(value, Some(90.0));
}

#[test]
fn except_variants_map_to_base_verb() {
    assert_eq!(
        canonicalize_household_action("turn_off_except", None).0,
        "turn_off"
    );
    assert_eq!(canonicalize_household_action("lock_except", None).0, "lock");
}

#[test]
fn scene_and_mode_verbs_map_to_activate_and_drop_value() {
    assert_eq!(
        canonicalize_household_action("apply_scene", None).0,
        "activate"
    );
    let (action, value) = canonicalize_household_action("set_volume", Some(25.0));
    assert_eq!(action, "activate");
    assert_eq!(value, None);
}

#[test]
fn quick_router_home_control_emissions_are_all_valid() {
    let utterances = [
        "Put the house in low power mode until five",
        "Mia: Set my room to sleepover lights.",
        "Give me focus mode until five",
        "Stop the sprinklers, it's raining",
        "Jared: Lock everything except the back gate",
        "Jared: Turn off everything downstairs except the kitchen lights",
        "Leo: Make the stairs bright.",
        "Set the oven to 400 degrees",
    ];
    for utterance in utterances {
        let call = route(utterance).expect("utterance should route");
        if call.name == "home_control" {
            let action = call.arguments["action"].as_str().unwrap();
            assert!(
                HOME_CONTROL_ACTIONS.contains(&action),
                "'{utterance}' routed home_control with invalid action '{action}'"
            );
        }
    }
}
