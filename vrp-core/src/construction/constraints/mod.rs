//! Various built-in constraints applied to customers and vehicles/drivers.
//!
//!
//! ## Constraint
//!
//! Constraint represents some limitation which should be applied to solution. A good examples:
//!
//! - **time**: customer can be visited only in specific time window, e.g. from 9am till 11am
//! - **capacity**: there is a fleet and multiple customers with total demand exceeding capacity
//!   of one vehicle from the fleet.
//! - **shift-time**: vehicle or driver cannot operate more than specific amount of time.
//!
//! Typically, VRP can have many of such constraints applied to its solution.
//!
//!
//! ## Design
//!
//! There are multiple types of constraints described below in details. In common, all of them try
//! to identify insertion possibility or cost of given customer known as `Job` into given route.
//!
//!
//! ### Constraint characteristics
//! Each constraint has two characteristic:
//!
//! - **hard or soft**: this characteristic defines what should happen when constraint is violated.
//!     When hard constraint is violated, it means that given customer cannot be served with given
//!     route. In contrast to this, soft constraint allows insertion but applies some penalty to
//!     make violation less attractive.
//!
//! - **route or activity**: this characteristic defines on which level constrain is executed.
//!     As a heuristic algorithm is based on insertion heuristic, insertion of one customer is
//!     evaluated on each leg of one route. When it does not make sense, the route constraint
//!     can be used as it is called only once to check whether customer can be inserted in given
//!     route.
//!
//!
//! ### Constraint module
//!
//! Sometimes you might need multiple constraints with different characteristics to implement some
//! aspect of VRP variation. This is where `ConstraintModule` supposed to be used: it allows you
//! to group multiple constraints together keeping implementation details hidden outside of module.
//! Additionally, `ConstraintModule` provides the way to share some state between insertions.
//! This is really important as allows you to avoid unneeded computations.
//!
//!
//! ### Sharing state
//!
//! You can share some state using `RouteState` object which is part of `RouteContext`. It is
//! read-only during insertion evaluation in all constraint types, but it is mutable via `ConstraintModule`
//! methods once best insertion is identified.
//!
//!
//! ### Constraint pipeline
//!
//! All constraint modules are organized inside one `ConstraintPipeline` which specifies the order
//! of their execution.

// region state keys

/// A key which tracks latest arrival.
pub const LATEST_ARRIVAL_KEY: i32 = 1;
/// A key which tracks waiting time.
pub const WAITING_KEY: i32 = 2;
/// A key which tracks total distance.
pub const TOTAL_DISTANCE_KEY: i32 = 3;
/// A key which track total duration.
pub const TOTAL_DURATION_KEY: i32 = 4;
/// A key which track duration limit.
pub const LIMIT_DURATION_KEY: i32 = 5;

/// A key which tracks current vehicle capacity.
pub const CURRENT_CAPACITY_KEY: i32 = 11;
/// A key which tracks maximum vehicle capacity ahead in route.
pub const MAX_FUTURE_CAPACITY_KEY: i32 = 12;
/// A key which tracks maximum capacity backward in route.
pub const MAX_PAST_CAPACITY_KEY: i32 = 13;
/// A key which tracks reload intervals.
pub const RELOAD_INTERVALS_KEY: i32 = 14;
/// A key which tracks max load in tour.
pub const MAX_LOAD_KEY: i32 = 15;
/// A key which tracks total value.
pub const TOTAL_VALUE_KEY: i32 = 16;
/// A key which tracks tour order statistics.
pub const TOUR_ORDER_KEY: i32 = 17;

// endregion

// region dimension keys

/// A key used to track job id. It is defined mostly for convenience.
pub const JOB_ID_DIMEN_KEY: i32 = 1;
/// A key used to track vehicle id. It is defined mostly for convenience.
pub const VEHICLE_ID_DIMEN_KEY: i32 = 2;
/// A key used to track a weak reference to multi job.
pub const MULTI_REF_DIMEN_KEY: i32 = 3;
/// A key to track vehicle capacity.
pub const CAPACITY_DIMEN_KEY: i32 = 4;
/// A key to track job demand.
pub const DEMAND_DIMEN_KEY: i32 = 5;
/// A key to track areas.
pub const AREA_DIMEN_KEY: i32 = 6;
/// A key to track order.
pub const ORDER_DIMEN_KEY: i32 = 7;
/// A key to track value.
pub const VALUE_DIMEN_KEY: i32 = 8;
/// A key to track clustered jobs.
pub const CLUSTER_JOBS_DIMEN_KEY: i32 = 9;

// TODO use lazy static to fill hashmap and do the check on high level?

// endregion

mod pipeline;
pub use self::pipeline::*;

mod area;
pub use self::area::*;

mod transport;
pub use self::transport::*;

mod capacity;
pub use self::capacity::*;

mod locking;
pub use self::locking::*;

mod tour_size;
pub use self::tour_size::*;

mod conditional;
pub use self::conditional::*;

mod fleet_usage;
pub use self::fleet_usage::*;

use crate::construction::heuristics::RouteContext;
use crate::models::problem::TransportCost;

/// Updates route schedule.
pub fn update_route_schedule(route_ctx: &mut RouteContext, transport: &(dyn TransportCost + Send + Sync)) {
    TransportConstraintModule::update_route_schedules(route_ctx, transport);
    TransportConstraintModule::update_route_states(route_ctx, transport);
    TransportConstraintModule::update_statistics(route_ctx, transport);
}
