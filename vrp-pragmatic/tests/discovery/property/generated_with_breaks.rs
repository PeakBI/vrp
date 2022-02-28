use crate::format::problem::*;
use crate::format::Location;
use crate::generator::*;
use crate::helpers::solve_with_metaheuristic_and_iterations;
use proptest::prelude::*;

mod optional {
    use super::*;

    fn get_optional_breaks() -> impl Strategy<Value = Option<Vec<VehicleBreak>>> {
        let places_proto = get_optional_break_places(
            prop_oneof![Just(None), generate_location(&DEFAULT_BOUNDING_BOX).prop_map(|location| Some(location))],
            generate_durations(10..100),
        );
        let break_proto = generate_break(
            prop::collection::vec(places_proto, 1..2),
            prop_oneof![get_optional_break_offset_time(), get_optional_break_time_windows()],
            Just(None),
        );

        prop::collection::vec(break_proto, 1..2).prop_map(|break_| Some(break_))
    }

    prop_compose! {
        pub fn get_optional_break_places(
           locations: impl Strategy<Value = Option<Location>>,
           durations: impl Strategy<Value = f64>,
        )
        (
         location in locations,
         duration in durations
        ) -> VehicleOptionalBreakPlace {
            VehicleOptionalBreakPlace { location, duration, tag: None }
        }
    }
    prop_compose! {
        fn get_optional_break_offset_time()
        (
         start in 3600..14400,
         length in 600..1800
        ) -> VehicleOptionalBreakTime {
            VehicleOptionalBreakTime::TimeOffset(vec![start as f64, (start + length) as f64])
        }
    }

    fn get_optional_break_time_windows() -> impl Strategy<Value = VehicleOptionalBreakTime> {
        generate_multiple_time_windows_fixed(
            START_DAY,
            vec![from_hours(11), from_hours(13)],
            vec![from_hours(2), from_hours(4)],
            1..2,
        )
        .prop_map(|tws| VehicleOptionalBreakTime::TimeWindow(tws.first().unwrap().clone()))
    }

    prop_compose! {
        fn get_vehicle_type_with_optional_breaks()
        (
         vehicle in default_vehicle_type_prototype(),
         breaks in get_optional_breaks()
        ) -> VehicleType {
            assert_eq!(vehicle.shifts.len(), 1);

            let mut vehicle = vehicle;
            vehicle.shifts.first_mut().unwrap().breaks = breaks;

            vehicle
        }
    }

    prop_compose! {
        pub(crate) fn get_problem_with_optional_breaks()
        (
         plan in generate_plan(generate_jobs(job_prototype(), 1..256)),
         fleet in generate_fleet(
            generate_vehicles(get_vehicle_type_with_optional_breaks(), 1..4),
            default_matrix_profiles())
        ) -> Problem {
            Problem { plan, fleet, objectives: None }
        }
    }
}

fn job_prototype() -> impl Strategy<Value = Job> {
    delivery_job_prototype(
        job_task_prototype(
            job_place_prototype(
                generate_location(&DEFAULT_BOUNDING_BOX),
                generate_durations(1..10),
                generate_no_time_windows(),
                generate_no_tags(),
            ),
            generate_simple_demand(1..5),
            generate_no_order(),
        ),
        generate_no_jobs_skills(),
        generate_no_jobs_value(),
        generate_no_jobs_group(),
        generate_no_jobs_compatibility(),
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]
    #[test]
    #[ignore]
    fn can_solve_problem_with_optional_breaks(problem in optional::get_problem_with_optional_breaks()) {
        solve_with_metaheuristic_and_iterations(problem, None, 10);
    }
}
