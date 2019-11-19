use crate::construction::constraints::capacity::CURRENT_CAPACITY_KEY;
use crate::construction::constraints::{ActivityConstraintViolation, RouteConstraintViolation};
use crate::construction::states::{ActivityContext, RouteContext, RouteState};
use crate::helpers::construction::constraints::*;
use crate::helpers::models::problem::*;
use crate::helpers::models::solution::*;
use crate::models::problem::{Fleet, Vehicle};
use crate::models::solution::TourActivity;
use std::sync::Arc;

fn create_test_vehicle(capacity: i32) -> Vehicle {
    VehicleBuilder::new().id("v1").capacity(capacity).build()
}

fn create_activity_violation() -> Option<ActivityConstraintViolation> {
    Some(ActivityConstraintViolation { code: 2, stopped: false })
}

fn get_simple_capacity_state(key: i32, state: &RouteState, activity: Option<&TourActivity>) -> i32 {
    *state.get_activity_state::<i32>(key, activity.unwrap()).unwrap()
}

parameterized_test! {can_calculate_current_capacity_state_values, (s1, s2, s3, start, end, exp_s1, exp_s2, exp_s3), {
    can_calculate_current_capacity_state_values_impl(s1, s2, s3, start, end, exp_s1, exp_s2, exp_s3);
}}

can_calculate_current_capacity_state_values! {
    case01: (-1, 2, -3, 4, 2, 3, 5, 2),
    case02: (1, -2, 3, 2, 4, 3, 1, 4),
    case03: (0, 1, 0, 0, 1, 0, 1, 1),
}

fn can_calculate_current_capacity_state_values_impl(
    s1: i32,
    s2: i32,
    s3: i32,
    start: i32,
    end: i32,
    exp_s1: i32,
    exp_s2: i32,
    exp_s3: i32,
) {
    let fleet = Fleet::new(vec![test_driver()], vec![create_test_vehicle(10)]);
    let mut ctx = RouteContext {
        route: Arc::new(create_route_with_activities(
            &fleet,
            "v1",
            vec![
                test_tour_activity_with_simple_demand(create_simple_demand(s1)),
                test_tour_activity_with_simple_demand(create_simple_demand(s2)),
                test_tour_activity_with_simple_demand(create_simple_demand(s3)),
            ],
        )),
        state: Arc::new(RouteState::default()),
    };

    create_constraint_pipeline_with_simple_capacity().accept_route_state(&mut ctx);

    let tour = &ctx.route.tour;
    let state = &ctx.state;
    assert_eq!(get_simple_capacity_state(CURRENT_CAPACITY_KEY, state, tour.start()), start);
    assert_eq!(get_simple_capacity_state(CURRENT_CAPACITY_KEY, state, tour.end()), end);
    assert_eq!(get_simple_capacity_state(CURRENT_CAPACITY_KEY, state, tour.get(1)), exp_s1);
    assert_eq!(get_simple_capacity_state(CURRENT_CAPACITY_KEY, state, tour.get(2)), exp_s2);
    assert_eq!(get_simple_capacity_state(CURRENT_CAPACITY_KEY, state, tour.get(3)), exp_s3);
}

parameterized_test! {can_evaluate_demand_on_route, (size, expected), {
    can_evaluate_demand_on_route_impl(size, expected);
}}

can_evaluate_demand_on_route! {
    case01: (11, Some(RouteConstraintViolation { code: 2})),
    case02: (10, None),
    case03: (9, None),
}

fn can_evaluate_demand_on_route_impl(size: i32, expected: Option<RouteConstraintViolation>) {
    let fleet = Fleet::new(vec![test_driver()], vec![create_test_vehicle(10)]);
    let ctx = RouteContext {
        route: Arc::new(create_route_with_activities(&fleet, "v1", vec![])),
        state: Arc::new(RouteState::default()),
    };
    let job = Arc::new(test_single_job_with_simple_demand(create_simple_demand(size)));

    let result = create_constraint_pipeline_with_simple_capacity().evaluate_hard_route(&ctx, &job);

    assert_eq_option!(result, expected);
}

parameterized_test! {can_evaluate_demand_on_activity, (sizes, neighbours, size, expected), {
    can_evaluate_demand_on_activity_impl(sizes, neighbours, size, expected);
}}

can_evaluate_demand_on_activity! {
    case01: (vec![1, 1], (1, 2), 1, None),
    case02: (vec![1, 1], (1, 2),10, create_activity_violation()),
    case03: (vec![-5, -5], (1, 2),-1, create_activity_violation()),
    case04: (vec![5, 5], (1, 2),1, create_activity_violation()),
    case05: (vec![-5, 5], (1, 2),1, None),
    case06: (vec![5, -5], (1, 2), 1, create_activity_violation()),
    case07: (vec![4, -5], (1, 2),-1, None),
    case08: (vec![-3, -5, -2], (0, 1),-1, create_activity_violation()),
    case09: (vec![-3, -5, -2], (0, 2), -1, create_activity_violation()),
    case10: (vec![-3, -5, -2], (1, 3), -1, create_activity_violation()),
    case11: (vec![-3, -5, -2], (3, 4), -1, create_activity_violation()),
}

fn can_evaluate_demand_on_activity_impl(
    sizes: Vec<i32>,
    neighbours: (usize, usize),
    size: i32,
    expected: Option<ActivityConstraintViolation>,
) {
    let fleet = Fleet::new(vec![test_driver()], vec![create_test_vehicle(10)]);
    let mut route_ctx = RouteContext {
        route: Arc::new(create_route_with_activities(
            &fleet,
            "v1",
            sizes.into_iter().map(|size| test_tour_activity_with_simple_demand(create_simple_demand(size))).collect(),
        )),
        state: Arc::new(RouteState::default()),
    };
    let pipeline = create_constraint_pipeline_with_simple_capacity();
    pipeline.accept_route_state(&mut route_ctx);
    let target = test_tour_activity_with_simple_demand(create_simple_demand(size));
    let activity_ctx = ActivityContext {
        index: 0,
        prev: route_ctx.route.tour.get(neighbours.0).unwrap(),
        target: &target,
        next: route_ctx.route.tour.get(neighbours.1),
    };

    let result = pipeline.evaluate_hard_activity(&route_ctx, &activity_ctx);

    assert_eq_option!(result, expected);
}
