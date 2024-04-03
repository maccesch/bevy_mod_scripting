#![allow(clippy::all, unused_imports, deprecated, dead_code)]
extern crate self as bevy_script_api;
use bevy_script_api::{lua::RegisterForeignLuaType, ReflectedValue};
/// A `Duration` type to represent a span of time, typically used for system
/// timeouts.
/// Each `Duration` is composed of a whole number of seconds and a fractional part
/// represented in nanoseconds. If the underlying system does not support
/// nanosecond-level precision, APIs binding a system timeout will typically round up
/// the number of nanoseconds.
/// [`Duration`]s implement many common traits, including [`Add`], [`Sub`], and other
/// [`ops`] traits. It implements [`Default`] by returning a zero-length `Duration`.
/// [`ops`]: crate::ops
/// # Examples
/// ```
/// use std::time::Duration;
/// let five_seconds = Duration::new(5, 0);
/// let five_seconds_and_five_nanos = five_seconds + Duration::new(0, 5);
/// assert_eq!(five_seconds_and_five_nanos.as_secs(), 5);
/// assert_eq!(five_seconds_and_five_nanos.subsec_nanos(), 5);
/// let ten_millis = Duration::from_millis(10);
/// ```
/// # Formatting `Duration` values
/// `Duration` intentionally does not have a `Display` impl, as there are a
/// variety of ways to format spans of time for human readability. `Duration`
/// provides a `Debug` impl that shows the full precision of the value.
/// The `Debug` output uses the non-ASCII "µs" suffix for microseconds. If your
/// program output may appear in contexts that cannot rely on full Unicode
/// compatibility, you may wish to format `Duration` objects yourself or use a
/// crate to do so.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::utils::Duration",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::utils::Duration) -> bevy::utils::Duration;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: u32) -> bevy::utils::Duration;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: u32) -> bevy::utils::Duration;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::utils::Duration) -> bevy::utils::Duration;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_utils::Duration) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::utils::Duration;

"#,
    r#"
/// Creates a new `Duration` from the specified number of whole seconds and
/// additional nanoseconds.
/// If the number of nanoseconds is greater than 1 billion (the number of
/// nanoseconds in a second), then it will carry over into the seconds provided.
/// # Panics
/// This constructor will panic if the carry from the nanoseconds overflows
/// the seconds counter.
/// # Examples
/// ```
/// use std::time::Duration;
/// let five_seconds = Duration::new(5, 0);
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn new(secs: u64, nanos: u32) -> bevy::utils::Duration;

"#,
    r#"
/// Creates a new `Duration` from the specified number of whole seconds.
/// # Examples
/// ```
/// use std::time::Duration;
/// let duration = Duration::from_secs(5);
/// assert_eq!(5, duration.as_secs());
/// assert_eq!(0, duration.subsec_nanos());
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_secs(secs: u64) -> bevy::utils::Duration;

"#,
    r#"
/// Creates a new `Duration` from the specified number of milliseconds.
/// # Examples
/// ```
/// use std::time::Duration;
/// let duration = Duration::from_millis(2569);
/// assert_eq!(2, duration.as_secs());
/// assert_eq!(569_000_000, duration.subsec_nanos());
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_millis(millis: u64) -> bevy::utils::Duration;

"#,
    r#"
/// Creates a new `Duration` from the specified number of microseconds.
/// # Examples
/// ```
/// use std::time::Duration;
/// let duration = Duration::from_micros(1_000_002);
/// assert_eq!(1, duration.as_secs());
/// assert_eq!(2000, duration.subsec_nanos());
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_micros(micros: u64) -> bevy::utils::Duration;

"#,
    r#"
/// Creates a new `Duration` from the specified number of nanoseconds.
/// # Examples
/// ```
/// use std::time::Duration;
/// let duration = Duration::from_nanos(1_000_000_123);
/// assert_eq!(1, duration.as_secs());
/// assert_eq!(123, duration.subsec_nanos());
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_nanos(nanos: u64) -> bevy::utils::Duration;

"#,
    r#"
/// Returns true if this `Duration` spans no time.
/// # Examples
/// ```
/// use std::time::Duration;
/// assert!(Duration::ZERO.is_zero());
/// assert!(Duration::new(0, 0).is_zero());
/// assert!(Duration::from_nanos(0).is_zero());
/// assert!(Duration::from_secs(0).is_zero());
/// assert!(!Duration::new(1, 1).is_zero());
/// assert!(!Duration::from_nanos(1).is_zero());
/// assert!(!Duration::from_secs(1).is_zero());
/// ```

    #[lua(kind = "Method")]
    fn is_zero(&self) -> bool;

"#,
    r#"
/// Returns the number of _whole_ seconds contained by this `Duration`.
/// The returned value does not include the fractional (nanosecond) part of the
/// duration, which can be obtained using [`subsec_nanos`].
/// # Examples
/// ```
/// use std::time::Duration;
/// let duration = Duration::new(5, 730023852);
/// assert_eq!(duration.as_secs(), 5);
/// ```
/// To determine the total number of seconds represented by the `Duration`
/// including the fractional part, use [`as_secs_f64`] or [`as_secs_f32`]
/// [`as_secs_f64`]: Duration::as_secs_f64
/// [`as_secs_f32`]: Duration::as_secs_f32
/// [`subsec_nanos`]: Duration::subsec_nanos

    #[lua(kind = "Method")]
    fn as_secs(&self) -> u64;

"#,
    r#"
/// Returns the fractional part of this `Duration`, in whole milliseconds.
/// This method does **not** return the length of the duration when
/// represented by milliseconds. The returned number always represents a
/// fractional portion of a second (i.e., it is less than one thousand).
/// # Examples
/// ```
/// use std::time::Duration;
/// let duration = Duration::from_millis(5432);
/// assert_eq!(duration.as_secs(), 5);
/// assert_eq!(duration.subsec_millis(), 432);
/// ```

    #[lua(kind = "Method")]
    fn subsec_millis(&self) -> u32;

"#,
    r#"
/// Returns the fractional part of this `Duration`, in whole microseconds.
/// This method does **not** return the length of the duration when
/// represented by microseconds. The returned number always represents a
/// fractional portion of a second (i.e., it is less than one million).
/// # Examples
/// ```
/// use std::time::Duration;
/// let duration = Duration::from_micros(1_234_567);
/// assert_eq!(duration.as_secs(), 1);
/// assert_eq!(duration.subsec_micros(), 234_567);
/// ```

    #[lua(kind = "Method")]
    fn subsec_micros(&self) -> u32;

"#,
    r#"
/// Returns the fractional part of this `Duration`, in nanoseconds.
/// This method does **not** return the length of the duration when
/// represented by nanoseconds. The returned number always represents a
/// fractional portion of a second (i.e., it is less than one billion).
/// # Examples
/// ```
/// use std::time::Duration;
/// let duration = Duration::from_millis(5010);
/// assert_eq!(duration.as_secs(), 5);
/// assert_eq!(duration.subsec_nanos(), 10_000_000);
/// ```

    #[lua(kind = "Method")]
    fn subsec_nanos(&self) -> u32;

"#,
    r#"
/// Returns the total number of whole milliseconds contained by this `Duration`.
/// # Examples
/// ```
/// use std::time::Duration;
/// let duration = Duration::new(5, 730023852);
/// assert_eq!(duration.as_millis(), 5730);
/// ```

    #[lua(kind = "Method")]
    fn as_millis(&self) -> u128;

"#,
    r#"
/// Returns the total number of whole microseconds contained by this `Duration`.
/// # Examples
/// ```
/// use std::time::Duration;
/// let duration = Duration::new(5, 730023852);
/// assert_eq!(duration.as_micros(), 5730023);
/// ```

    #[lua(kind = "Method")]
    fn as_micros(&self) -> u128;

"#,
    r#"
/// Returns the total number of nanoseconds contained by this `Duration`.
/// # Examples
/// ```
/// use std::time::Duration;
/// let duration = Duration::new(5, 730023852);
/// assert_eq!(duration.as_nanos(), 5730023852);
/// ```

    #[lua(kind = "Method")]
    fn as_nanos(&self) -> u128;

"#,
    r#"
/// Saturating `Duration` addition. Computes `self + other`, returning [`Duration::MAX`]
/// if overflow occurred.
/// # Examples
/// ```
/// #![feature(duration_constants)]
/// use std::time::Duration;
/// assert_eq!(Duration::new(0, 0).saturating_add(Duration::new(0, 1)), Duration::new(0, 1));
/// assert_eq!(Duration::new(1, 0).saturating_add(Duration::new(u64::MAX, 0)), Duration::MAX);
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(
        self,
        #[proxy]
        rhs: bevy::utils::Duration,
    ) -> bevy::utils::Duration;

"#,
    r#"
/// Saturating `Duration` subtraction. Computes `self - other`, returning [`Duration::ZERO`]
/// if the result would be negative or if overflow occurred.
/// # Examples
/// ```
/// use std::time::Duration;
/// assert_eq!(Duration::new(0, 1).saturating_sub(Duration::new(0, 0)), Duration::new(0, 1));
/// assert_eq!(Duration::new(0, 0).saturating_sub(Duration::new(0, 1)), Duration::ZERO);
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(
        self,
        #[proxy]
        rhs: bevy::utils::Duration,
    ) -> bevy::utils::Duration;

"#,
    r#"
/// Saturating `Duration` multiplication. Computes `self * other`, returning
/// [`Duration::MAX`] if overflow occurred.
/// # Examples
/// ```
/// #![feature(duration_constants)]
/// use std::time::Duration;
/// assert_eq!(Duration::new(0, 500_000_001).saturating_mul(2), Duration::new(1, 2));
/// assert_eq!(Duration::new(u64::MAX - 1, 0).saturating_mul(2), Duration::MAX);
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, rhs: u32) -> bevy::utils::Duration;

"#,
    r#"
/// Returns the number of seconds contained by this `Duration` as `f64`.
/// The returned value does include the fractional (nanosecond) part of the duration.
/// # Examples
/// ```
/// use std::time::Duration;
/// let dur = Duration::new(2, 700_000_000);
/// assert_eq!(dur.as_secs_f64(), 2.7);
/// ```

    #[lua(kind = "Method")]
    fn as_secs_f64(&self) -> f64;

"#,
    r#"
/// Returns the number of seconds contained by this `Duration` as `f32`.
/// The returned value does include the fractional (nanosecond) part of the duration.
/// # Examples
/// ```
/// use std::time::Duration;
/// let dur = Duration::new(2, 700_000_000);
/// assert_eq!(dur.as_secs_f32(), 2.7);
/// ```

    #[lua(kind = "Method")]
    fn as_secs_f32(&self) -> f32;

"#,
    r#"
/// Creates a new `Duration` from the specified number of seconds represented
/// as `f64`.
/// # Panics
/// This constructor will panic if `secs` is negative, overflows `Duration` or not finite.
/// # Examples
/// ```
/// use std::time::Duration;
/// let res = Duration::from_secs_f64(0.0);
/// assert_eq!(res, Duration::new(0, 0));
/// let res = Duration::from_secs_f64(1e-20);
/// assert_eq!(res, Duration::new(0, 0));
/// let res = Duration::from_secs_f64(4.2e-7);
/// assert_eq!(res, Duration::new(0, 420));
/// let res = Duration::from_secs_f64(2.7);
/// assert_eq!(res, Duration::new(2, 700_000_000));
/// let res = Duration::from_secs_f64(3e10);
/// assert_eq!(res, Duration::new(30_000_000_000, 0));
/// // subnormal float
/// let res = Duration::from_secs_f64(f64::from_bits(1));
/// assert_eq!(res, Duration::new(0, 0));
/// // conversion uses rounding
/// let res = Duration::from_secs_f64(0.999e-9);
/// assert_eq!(res, Duration::new(0, 1));
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_secs_f64(secs: f64) -> bevy::utils::Duration;

"#,
    r#"
/// Creates a new `Duration` from the specified number of seconds represented
/// as `f32`.
/// # Panics
/// This constructor will panic if `secs` is negative, overflows `Duration` or not finite.
/// # Examples
/// ```
/// use std::time::Duration;
/// let res = Duration::from_secs_f32(0.0);
/// assert_eq!(res, Duration::new(0, 0));
/// let res = Duration::from_secs_f32(1e-20);
/// assert_eq!(res, Duration::new(0, 0));
/// let res = Duration::from_secs_f32(4.2e-7);
/// assert_eq!(res, Duration::new(0, 420));
/// let res = Duration::from_secs_f32(2.7);
/// assert_eq!(res, Duration::new(2, 700_000_048));
/// let res = Duration::from_secs_f32(3e10);
/// assert_eq!(res, Duration::new(30_000_001_024, 0));
/// // subnormal float
/// let res = Duration::from_secs_f32(f32::from_bits(1));
/// assert_eq!(res, Duration::new(0, 0));
/// // conversion uses rounding
/// let res = Duration::from_secs_f32(0.999e-9);
/// assert_eq!(res, Duration::new(0, 1));
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_secs_f32(secs: f32) -> bevy::utils::Duration;

"#,
    r#"
/// Multiplies `Duration` by `f64`.
/// # Panics
/// This method will panic if result is negative, overflows `Duration` or not finite.
/// # Examples
/// ```
/// use std::time::Duration;
/// let dur = Duration::new(2, 700_000_000);
/// assert_eq!(dur.mul_f64(3.14), Duration::new(8, 478_000_000));
/// assert_eq!(dur.mul_f64(3.14e5), Duration::new(847_800, 0));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn mul_f64(self, rhs: f64) -> bevy::utils::Duration;

"#,
    r#"
/// Multiplies `Duration` by `f32`.
/// # Panics
/// This method will panic if result is negative, overflows `Duration` or not finite.
/// # Examples
/// ```
/// use std::time::Duration;
/// let dur = Duration::new(2, 700_000_000);
/// assert_eq!(dur.mul_f32(3.14), Duration::new(8, 478_000_641));
/// assert_eq!(dur.mul_f32(3.14e5), Duration::new(847800, 0));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn mul_f32(self, rhs: f32) -> bevy::utils::Duration;

"#,
    r#"
/// Divide `Duration` by `f64`.
/// # Panics
/// This method will panic if result is negative, overflows `Duration` or not finite.
/// # Examples
/// ```
/// use std::time::Duration;
/// let dur = Duration::new(2, 700_000_000);
/// assert_eq!(dur.div_f64(3.14), Duration::new(0, 859_872_611));
/// assert_eq!(dur.div_f64(3.14e5), Duration::new(0, 8_599));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn div_f64(self, rhs: f64) -> bevy::utils::Duration;

"#,
    r#"
/// Divide `Duration` by `f32`.
/// # Panics
/// This method will panic if result is negative, overflows `Duration` or not finite.
/// # Examples
/// ```
/// use std::time::Duration;
/// let dur = Duration::new(2, 700_000_000);
/// // note that due to rounding errors result is slightly
/// // different from 0.859_872_611
/// assert_eq!(dur.div_f32(3.14), Duration::new(0, 859_872_580));
/// assert_eq!(dur.div_f32(3.14e5), Duration::new(0, 8_599));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn div_f32(self, rhs: f32) -> bevy::utils::Duration;

"#]
)]
pub struct Duration {}
/// A measurement of a monotonically nondecreasing clock.
/// Opaque and useful only with [`Duration`].
/// Instants are always guaranteed, barring [platform bugs], to be no less than any previously
/// measured instant when created, and are often useful for tasks such as measuring
/// benchmarks or timing how long an operation takes.
/// Note, however, that instants are **not** guaranteed to be **steady**. In other
/// words, each tick of the underlying clock might not be the same length (e.g.
/// some seconds may be longer than others). An instant may jump forwards or
/// experience time dilation (slow down or speed up), but it will never go
/// backwards.
/// As part of this non-guarantee it is also not specified whether system suspends count as
/// elapsed time or not. The behavior varies across platforms and rust versions.
/// Instants are opaque types that can only be compared to one another. There is
/// no method to get "the number of seconds" from an instant. Instead, it only
/// allows measuring the duration between two instants (or comparing two
/// instants).
/// The size of an `Instant` struct may vary depending on the target operating
/// system.
/// Example:
/// ```no_run
/// use std::time::{Duration, Instant};
/// use std::thread::sleep;
/// fn main() {
///    let now = Instant::now();
///    // we sleep for 2 seconds
///    sleep(Duration::new(2, 0));
///    // it prints '2'
///    println!("{}", now.elapsed().as_secs());
/// }
/// ```
/// [platform bugs]: Instant#monotonicity
/// # OS-specific behaviors
/// An `Instant` is a wrapper around system-specific types and it may behave
/// differently depending on the underlying operating system. For example,
/// the following snippet is fine on Linux but panics on macOS:
/// ```no_run
/// use std::time::{Instant, Duration};
/// let now = Instant::now();
/// let max_seconds = u64::MAX / 1_000_000_000;
/// let duration = Duration::new(max_seconds, 0);
/// println!("{:?}", now + duration);
/// ```
/// # Underlying System calls
/// The following system calls are [currently] being used by `now()` to find out
/// the current time:
/// |  Platform |               System call                                            |
/// |-----------|----------------------------------------------------------------------|
/// | SGX       | [`insecure_time` usercall]. More information on [timekeeping in SGX] |
/// | UNIX      | [clock_gettime (Monotonic Clock)]                                    |
/// | Darwin    | [clock_gettime (Monotonic Clock)]                                    |
/// | VXWorks   | [clock_gettime (Monotonic Clock)]                                    |
/// | SOLID     | `get_tim`                                                            |
/// | WASI      | [__wasi_clock_time_get (Monotonic Clock)]                            |
/// | Windows   | [QueryPerformanceCounter]                                            |
/// [currently]: crate::io#platform-specific-behavior
/// [QueryPerformanceCounter]: https://docs.microsoft.com/en-us/windows/win32/api/profileapi/nf-profileapi-queryperformancecounter
/// [`insecure_time` usercall]: https://edp.fortanix.com/docs/api/fortanix_sgx_abi/struct.Usercalls.html#method.insecure_time
/// [timekeeping in SGX]: https://edp.fortanix.com/docs/concepts/rust-std/#codestdtimecode
/// [__wasi_clock_time_get (Monotonic Clock)]: https://github.com/WebAssembly/WASI/blob/main/legacy/preview1/docs.md#clock_time_get
/// [clock_gettime (Monotonic Clock)]: https://linux.die.net/man/3/clock_gettime
/// **Disclaimer:** These system calls might change over time.
/// > Note: mathematical operations like [`add`] may panic if the underlying
/// > structure cannot represent the new point in time.
/// [`add`]: Instant::add
/// ## Monotonicity
/// On all platforms `Instant` will try to use an OS API that guarantees monotonic behavior
/// if available, which is the case for all [tier 1] platforms.
/// In practice such guarantees are – under rare circumstances – broken by hardware, virtualization
/// or operating system bugs. To work around these bugs and platforms not offering monotonic clocks
/// [`duration_since`], [`elapsed`] and [`sub`] saturate to zero. In older Rust versions this
/// lead to a panic instead. [`checked_duration_since`] can be used to detect and handle situations
/// where monotonicity is violated, or `Instant`s are subtracted in the wrong order.
/// This workaround obscures programming errors where earlier and later instants are accidentally
/// swapped. For this reason future rust versions may reintroduce panics.
/// [tier 1]: https://doc.rust-lang.org/rustc/platform-support.html
/// [`duration_since`]: Instant::duration_since
/// [`elapsed`]: Instant::elapsed
/// [`sub`]: Instant::sub
/// [`checked_duration_since`]: Instant::checked_duration_since
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::utils::Instant",
    functions[r#"
/// # Panics
/// This function may panic if the resulting point in time cannot be represented by the
/// underlying data structure. See [`Instant::checked_add`] for a version without panic.

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] other: bevy::utils::Duration) -> bevy::utils::Instant;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_utils::Instant) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] other: bevy::utils::Duration) -> bevy::utils::Instant;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::utils::Instant;

"#,
    r#"
/// Returns an instant corresponding to "now".
/// # Examples
/// ```
/// use std::time::Instant;
/// let now = Instant::now();
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn now() -> bevy::utils::Instant;

"#,
    r#"
/// Returns the amount of time elapsed from another instant to this one,
/// or zero duration if that instant is later than this one.
/// # Panics
/// Previous rust versions panicked when `earlier` was later than `self`. Currently this
/// method saturates. Future versions may reintroduce the panic in some circumstances.
/// See [Monotonicity].
/// [Monotonicity]: Instant#monotonicity
/// # Examples
/// ```no_run
/// use std::time::{Duration, Instant};
/// use std::thread::sleep;
/// let now = Instant::now();
/// sleep(Duration::new(1, 0));
/// let new_now = Instant::now();
/// println!("{:?}", new_now.duration_since(now));
/// println!("{:?}", now.duration_since(new_now)); // 0ns
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn duration_since(
        &self,
        #[proxy]
        earlier: bevy::utils::Instant,
    ) -> bevy::utils::Duration;

"#,
    r#"
/// Returns the amount of time elapsed from another instant to this one,
/// or zero duration if that instant is later than this one.
/// # Examples
/// ```no_run
/// use std::time::{Duration, Instant};
/// use std::thread::sleep;
/// let now = Instant::now();
/// sleep(Duration::new(1, 0));
/// let new_now = Instant::now();
/// println!("{:?}", new_now.saturating_duration_since(now));
/// println!("{:?}", now.saturating_duration_since(new_now)); // 0ns
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_duration_since(
        &self,
        #[proxy]
        earlier: bevy::utils::Instant,
    ) -> bevy::utils::Duration;

"#,
    r#"
/// Returns the amount of time elapsed since this instant.
/// # Panics
/// Previous rust versions panicked when the current time was earlier than self. Currently this
/// method returns a Duration of zero in that case. Future versions may reintroduce the panic.
/// See [Monotonicity].
/// [Monotonicity]: Instant#monotonicity
/// # Examples
/// ```no_run
/// use std::thread::sleep;
/// use std::time::{Duration, Instant};
/// let instant = Instant::now();
/// let three_secs = Duration::from_secs(3);
/// sleep(three_secs);
/// assert!(instant.elapsed() >= three_secs);
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn elapsed(&self) -> bevy::utils::Duration;

"#,
    r#"
/// Returns the amount of time elapsed from another instant to this one,
/// or zero duration if that instant is later than this one.
/// # Panics
/// Previous rust versions panicked when `other` was later than `self`. Currently this
/// method saturates. Future versions may reintroduce the panic in some circumstances.
/// See [Monotonicity].
/// [Monotonicity]: Instant#monotonicity

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] other: bevy::utils::Instant) -> bevy::utils::Duration;

"#]
)]
pub struct Instant();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroI128>` is the same size as `i128`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroI128>>(), size_of::<i128>());
/// ```
/// # Layout
///`NonZeroI128` is guaranteed to have the same layout and bit validity as `i128`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroI128>` is guaranteed to be compatible with `i128`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroI128` and `Option<NonZeroI128>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroI128;
///assert_eq!(size_of::<NonZeroI128>(), size_of::<Option<NonZeroI128>>());
///assert_eq!(align_of::<NonZeroI128>(), align_of::<Option<NonZeroI128>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroI128",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroI128;

"#,
    r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: i128) -> std::num::NonZeroI128;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> i128;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroI128::new(-1i128).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroI128::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Computes the absolute value of self.
///See [`i128::abs`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroI128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI128::new(1)?;
///let neg = NonZeroI128::new(-1)?;
/// assert_eq!(pos, pos.abs());
/// assert_eq!(pos, neg.abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> std::num::NonZeroI128;

"#,
    r#"
/// Saturating absolute value, see
///[`i128::saturating_abs`].
/// # Example
/// ```
///# use std::num::NonZeroI128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI128::new(1)?;
///let neg = NonZeroI128::new(-1)?;
///let min = NonZeroI128::new(i128::MIN)?;
///let min_plus = NonZeroI128::new(i128::MIN + 1)?;
///let max = NonZeroI128::new(i128::MAX)?;
/// assert_eq!(pos, pos.saturating_abs());
/// assert_eq!(pos, neg.saturating_abs());
/// assert_eq!(max, min.saturating_abs());
/// assert_eq!(max, min_plus.saturating_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_abs(self) -> std::num::NonZeroI128;

"#,
    r#"
/// Wrapping absolute value, see
///[`i128::wrapping_abs`].
/// # Example
/// ```
///# use std::num::NonZeroI128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI128::new(1)?;
///let neg = NonZeroI128::new(-1)?;
///let min = NonZeroI128::new(i128::MIN)?;
///# let max = NonZeroI128::new(i128::MAX)?;
/// assert_eq!(pos, pos.wrapping_abs());
/// assert_eq!(pos, neg.wrapping_abs());
/// assert_eq!(min, min.wrapping_abs());
/// assert_eq!(max, (-max).wrapping_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_abs(self) -> std::num::NonZeroI128;

"#,
    r#"
/// Computes the absolute value of self
/// without any wrapping or panicking.
/// # Example
/// ```
///# use std::num::NonZeroI128;
///# use std::num::NonZeroU128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let u_pos = NonZeroU128::new(1)?;
///let i_pos = NonZeroI128::new(1)?;
///let i_neg = NonZeroI128::new(-1)?;
///let i_min = NonZeroI128::new(i128::MIN)?;
///let u_max = NonZeroU128::new(u128::MAX / 2 + 1)?;
/// assert_eq!(u_pos, i_pos.unsigned_abs());
/// assert_eq!(u_pos, i_neg.unsigned_abs());
/// assert_eq!(u_max, i_min.unsigned_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn unsigned_abs(self) -> std::num::NonZeroU128;

"#,
    r#"
/// Returns `true` if `self` is positive and `false` if the
/// number is negative.
/// # Example
/// ```
///# use std::num::NonZeroI128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI128::new(5)?;
///let neg_five = NonZeroI128::new(-5)?;
/// assert!(pos_five.is_positive());
/// assert!(!neg_five.is_positive());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_positive(self) -> bool;

"#,
    r#"
/// Returns `true` if `self` is negative and `false` if the
/// number is positive.
/// # Example
/// ```
///# use std::num::NonZeroI128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI128::new(5)?;
///let neg_five = NonZeroI128::new(-5)?;
/// assert!(neg_five.is_negative());
/// assert!(!pos_five.is_negative());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_negative(self) -> bool;

"#,
    r#"
/// Saturating negation. Computes `-self`,
///returning [`NonZeroI128::MAX`]
///if `self == NonZeroI128::MIN`
/// instead of overflowing.
/// # Example
/// ```
///# use std::num::NonZeroI128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI128::new(5)?;
///let neg_five = NonZeroI128::new(-5)?;
///let min = NonZeroI128::new(i128::MIN)?;
///let min_plus_one = NonZeroI128::new(i128::MIN + 1)?;
///let max = NonZeroI128::new(i128::MAX)?;
/// assert_eq!(pos_five.saturating_neg(), neg_five);
/// assert_eq!(min.saturating_neg(), max);
/// assert_eq!(max.saturating_neg(), min_plus_one);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_neg(self) -> std::num::NonZeroI128;

"#,
    r#"
/// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
/// of the type.
///See [`i128::wrapping_neg`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroI128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI128::new(5)?;
///let neg_five = NonZeroI128::new(-5)?;
///let min = NonZeroI128::new(i128::MIN)?;
/// assert_eq!(pos_five.wrapping_neg(), neg_five);
/// assert_eq!(min.wrapping_neg(), min);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_neg(self) -> std::num::NonZeroI128;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroI128::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroI128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroI128::new(2)?;
///let four = NonZeroI128::new(4)?;
///let max = NonZeroI128::new(i128::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(
        self,
        #[proxy]
        other: std::num::NonZeroI128,
    ) -> std::num::NonZeroI128;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroI128::MIN`] or [`NonZeroI128::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroI128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroI128::new(3)?;
///let twenty_seven = NonZeroI128::new(27)?;
///let max = NonZeroI128::new(i128::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroI128;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroI128) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> std::num::NonZeroI128;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct NonZeroI128();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroI16>` is the same size as `i16`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroI16>>(), size_of::<i16>());
/// ```
/// # Layout
///`NonZeroI16` is guaranteed to have the same layout and bit validity as `i16`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroI16>` is guaranteed to be compatible with `i16`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroI16` and `Option<NonZeroI16>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroI16;
///assert_eq!(size_of::<NonZeroI16>(), size_of::<Option<NonZeroI16>>());
///assert_eq!(align_of::<NonZeroI16>(), align_of::<Option<NonZeroI16>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroI16",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroI16) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroI16;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: i16) -> std::num::NonZeroI16;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> i16;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroI16::new(-1i16).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroI16::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Computes the absolute value of self.
///See [`i16::abs`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroI16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI16::new(1)?;
///let neg = NonZeroI16::new(-1)?;
/// assert_eq!(pos, pos.abs());
/// assert_eq!(pos, neg.abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> std::num::NonZeroI16;

"#,
    r#"
/// Saturating absolute value, see
///[`i16::saturating_abs`].
/// # Example
/// ```
///# use std::num::NonZeroI16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI16::new(1)?;
///let neg = NonZeroI16::new(-1)?;
///let min = NonZeroI16::new(i16::MIN)?;
///let min_plus = NonZeroI16::new(i16::MIN + 1)?;
///let max = NonZeroI16::new(i16::MAX)?;
/// assert_eq!(pos, pos.saturating_abs());
/// assert_eq!(pos, neg.saturating_abs());
/// assert_eq!(max, min.saturating_abs());
/// assert_eq!(max, min_plus.saturating_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_abs(self) -> std::num::NonZeroI16;

"#,
    r#"
/// Wrapping absolute value, see
///[`i16::wrapping_abs`].
/// # Example
/// ```
///# use std::num::NonZeroI16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI16::new(1)?;
///let neg = NonZeroI16::new(-1)?;
///let min = NonZeroI16::new(i16::MIN)?;
///# let max = NonZeroI16::new(i16::MAX)?;
/// assert_eq!(pos, pos.wrapping_abs());
/// assert_eq!(pos, neg.wrapping_abs());
/// assert_eq!(min, min.wrapping_abs());
/// assert_eq!(max, (-max).wrapping_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_abs(self) -> std::num::NonZeroI16;

"#,
    r#"
/// Computes the absolute value of self
/// without any wrapping or panicking.
/// # Example
/// ```
///# use std::num::NonZeroI16;
///# use std::num::NonZeroU16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let u_pos = NonZeroU16::new(1)?;
///let i_pos = NonZeroI16::new(1)?;
///let i_neg = NonZeroI16::new(-1)?;
///let i_min = NonZeroI16::new(i16::MIN)?;
///let u_max = NonZeroU16::new(u16::MAX / 2 + 1)?;
/// assert_eq!(u_pos, i_pos.unsigned_abs());
/// assert_eq!(u_pos, i_neg.unsigned_abs());
/// assert_eq!(u_max, i_min.unsigned_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn unsigned_abs(self) -> std::num::NonZeroU16;

"#,
    r#"
/// Returns `true` if `self` is positive and `false` if the
/// number is negative.
/// # Example
/// ```
///# use std::num::NonZeroI16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI16::new(5)?;
///let neg_five = NonZeroI16::new(-5)?;
/// assert!(pos_five.is_positive());
/// assert!(!neg_five.is_positive());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_positive(self) -> bool;

"#,
    r#"
/// Returns `true` if `self` is negative and `false` if the
/// number is positive.
/// # Example
/// ```
///# use std::num::NonZeroI16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI16::new(5)?;
///let neg_five = NonZeroI16::new(-5)?;
/// assert!(neg_five.is_negative());
/// assert!(!pos_five.is_negative());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_negative(self) -> bool;

"#,
    r#"
/// Saturating negation. Computes `-self`,
///returning [`NonZeroI16::MAX`]
///if `self == NonZeroI16::MIN`
/// instead of overflowing.
/// # Example
/// ```
///# use std::num::NonZeroI16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI16::new(5)?;
///let neg_five = NonZeroI16::new(-5)?;
///let min = NonZeroI16::new(i16::MIN)?;
///let min_plus_one = NonZeroI16::new(i16::MIN + 1)?;
///let max = NonZeroI16::new(i16::MAX)?;
/// assert_eq!(pos_five.saturating_neg(), neg_five);
/// assert_eq!(min.saturating_neg(), max);
/// assert_eq!(max.saturating_neg(), min_plus_one);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_neg(self) -> std::num::NonZeroI16;

"#,
    r#"
/// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
/// of the type.
///See [`i16::wrapping_neg`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroI16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI16::new(5)?;
///let neg_five = NonZeroI16::new(-5)?;
///let min = NonZeroI16::new(i16::MIN)?;
/// assert_eq!(pos_five.wrapping_neg(), neg_five);
/// assert_eq!(min.wrapping_neg(), min);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_neg(self) -> std::num::NonZeroI16;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroI16::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroI16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroI16::new(2)?;
///let four = NonZeroI16::new(4)?;
///let max = NonZeroI16::new(i16::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(
        self,
        #[proxy]
        other: std::num::NonZeroI16,
    ) -> std::num::NonZeroI16;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroI16::MIN`] or [`NonZeroI16::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroI16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroI16::new(3)?;
///let twenty_seven = NonZeroI16::new(27)?;
///let max = NonZeroI16::new(i16::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroI16;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> std::num::NonZeroI16;

"#]
)]
pub struct NonZeroI16();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroI32>` is the same size as `i32`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroI32>>(), size_of::<i32>());
/// ```
/// # Layout
///`NonZeroI32` is guaranteed to have the same layout and bit validity as `i32`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroI32>` is guaranteed to be compatible with `i32`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroI32` and `Option<NonZeroI32>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroI32;
///assert_eq!(size_of::<NonZeroI32>(), size_of::<Option<NonZeroI32>>());
///assert_eq!(align_of::<NonZeroI32>(), align_of::<Option<NonZeroI32>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroI32",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> std::num::NonZeroI32;

"#,
    r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: i32) -> std::num::NonZeroI32;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> i32;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroI32::new(-1i32).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroI32::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Computes the absolute value of self.
///See [`i32::abs`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroI32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI32::new(1)?;
///let neg = NonZeroI32::new(-1)?;
/// assert_eq!(pos, pos.abs());
/// assert_eq!(pos, neg.abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> std::num::NonZeroI32;

"#,
    r#"
/// Saturating absolute value, see
///[`i32::saturating_abs`].
/// # Example
/// ```
///# use std::num::NonZeroI32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI32::new(1)?;
///let neg = NonZeroI32::new(-1)?;
///let min = NonZeroI32::new(i32::MIN)?;
///let min_plus = NonZeroI32::new(i32::MIN + 1)?;
///let max = NonZeroI32::new(i32::MAX)?;
/// assert_eq!(pos, pos.saturating_abs());
/// assert_eq!(pos, neg.saturating_abs());
/// assert_eq!(max, min.saturating_abs());
/// assert_eq!(max, min_plus.saturating_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_abs(self) -> std::num::NonZeroI32;

"#,
    r#"
/// Wrapping absolute value, see
///[`i32::wrapping_abs`].
/// # Example
/// ```
///# use std::num::NonZeroI32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI32::new(1)?;
///let neg = NonZeroI32::new(-1)?;
///let min = NonZeroI32::new(i32::MIN)?;
///# let max = NonZeroI32::new(i32::MAX)?;
/// assert_eq!(pos, pos.wrapping_abs());
/// assert_eq!(pos, neg.wrapping_abs());
/// assert_eq!(min, min.wrapping_abs());
/// assert_eq!(max, (-max).wrapping_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_abs(self) -> std::num::NonZeroI32;

"#,
    r#"
/// Computes the absolute value of self
/// without any wrapping or panicking.
/// # Example
/// ```
///# use std::num::NonZeroI32;
///# use std::num::NonZeroU32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let u_pos = NonZeroU32::new(1)?;
///let i_pos = NonZeroI32::new(1)?;
///let i_neg = NonZeroI32::new(-1)?;
///let i_min = NonZeroI32::new(i32::MIN)?;
///let u_max = NonZeroU32::new(u32::MAX / 2 + 1)?;
/// assert_eq!(u_pos, i_pos.unsigned_abs());
/// assert_eq!(u_pos, i_neg.unsigned_abs());
/// assert_eq!(u_max, i_min.unsigned_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn unsigned_abs(self) -> std::num::NonZeroU32;

"#,
    r#"
/// Returns `true` if `self` is positive and `false` if the
/// number is negative.
/// # Example
/// ```
///# use std::num::NonZeroI32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI32::new(5)?;
///let neg_five = NonZeroI32::new(-5)?;
/// assert!(pos_five.is_positive());
/// assert!(!neg_five.is_positive());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_positive(self) -> bool;

"#,
    r#"
/// Returns `true` if `self` is negative and `false` if the
/// number is positive.
/// # Example
/// ```
///# use std::num::NonZeroI32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI32::new(5)?;
///let neg_five = NonZeroI32::new(-5)?;
/// assert!(neg_five.is_negative());
/// assert!(!pos_five.is_negative());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_negative(self) -> bool;

"#,
    r#"
/// Saturating negation. Computes `-self`,
///returning [`NonZeroI32::MAX`]
///if `self == NonZeroI32::MIN`
/// instead of overflowing.
/// # Example
/// ```
///# use std::num::NonZeroI32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI32::new(5)?;
///let neg_five = NonZeroI32::new(-5)?;
///let min = NonZeroI32::new(i32::MIN)?;
///let min_plus_one = NonZeroI32::new(i32::MIN + 1)?;
///let max = NonZeroI32::new(i32::MAX)?;
/// assert_eq!(pos_five.saturating_neg(), neg_five);
/// assert_eq!(min.saturating_neg(), max);
/// assert_eq!(max.saturating_neg(), min_plus_one);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_neg(self) -> std::num::NonZeroI32;

"#,
    r#"
/// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
/// of the type.
///See [`i32::wrapping_neg`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroI32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI32::new(5)?;
///let neg_five = NonZeroI32::new(-5)?;
///let min = NonZeroI32::new(i32::MIN)?;
/// assert_eq!(pos_five.wrapping_neg(), neg_five);
/// assert_eq!(min.wrapping_neg(), min);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_neg(self) -> std::num::NonZeroI32;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroI32::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroI32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroI32::new(2)?;
///let four = NonZeroI32::new(4)?;
///let max = NonZeroI32::new(i32::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(
        self,
        #[proxy]
        other: std::num::NonZeroI32,
    ) -> std::num::NonZeroI32;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroI32::MIN`] or [`NonZeroI32::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroI32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroI32::new(3)?;
///let twenty_seven = NonZeroI32::new(27)?;
///let max = NonZeroI32::new(i32::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroI32;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroI32;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroI32) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct NonZeroI32();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroI64>` is the same size as `i64`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroI64>>(), size_of::<i64>());
/// ```
/// # Layout
///`NonZeroI64` is guaranteed to have the same layout and bit validity as `i64`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroI64>` is guaranteed to be compatible with `i64`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroI64` and `Option<NonZeroI64>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroI64;
///assert_eq!(size_of::<NonZeroI64>(), size_of::<Option<NonZeroI64>>());
///assert_eq!(align_of::<NonZeroI64>(), align_of::<Option<NonZeroI64>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroI64",
    functions[r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: i64) -> std::num::NonZeroI64;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> i64;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroI64::new(-1i64).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroI64::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Computes the absolute value of self.
///See [`i64::abs`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroI64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI64::new(1)?;
///let neg = NonZeroI64::new(-1)?;
/// assert_eq!(pos, pos.abs());
/// assert_eq!(pos, neg.abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> std::num::NonZeroI64;

"#,
    r#"
/// Saturating absolute value, see
///[`i64::saturating_abs`].
/// # Example
/// ```
///# use std::num::NonZeroI64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI64::new(1)?;
///let neg = NonZeroI64::new(-1)?;
///let min = NonZeroI64::new(i64::MIN)?;
///let min_plus = NonZeroI64::new(i64::MIN + 1)?;
///let max = NonZeroI64::new(i64::MAX)?;
/// assert_eq!(pos, pos.saturating_abs());
/// assert_eq!(pos, neg.saturating_abs());
/// assert_eq!(max, min.saturating_abs());
/// assert_eq!(max, min_plus.saturating_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_abs(self) -> std::num::NonZeroI64;

"#,
    r#"
/// Wrapping absolute value, see
///[`i64::wrapping_abs`].
/// # Example
/// ```
///# use std::num::NonZeroI64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI64::new(1)?;
///let neg = NonZeroI64::new(-1)?;
///let min = NonZeroI64::new(i64::MIN)?;
///# let max = NonZeroI64::new(i64::MAX)?;
/// assert_eq!(pos, pos.wrapping_abs());
/// assert_eq!(pos, neg.wrapping_abs());
/// assert_eq!(min, min.wrapping_abs());
/// assert_eq!(max, (-max).wrapping_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_abs(self) -> std::num::NonZeroI64;

"#,
    r#"
/// Computes the absolute value of self
/// without any wrapping or panicking.
/// # Example
/// ```
///# use std::num::NonZeroI64;
///# use std::num::NonZeroU64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let u_pos = NonZeroU64::new(1)?;
///let i_pos = NonZeroI64::new(1)?;
///let i_neg = NonZeroI64::new(-1)?;
///let i_min = NonZeroI64::new(i64::MIN)?;
///let u_max = NonZeroU64::new(u64::MAX / 2 + 1)?;
/// assert_eq!(u_pos, i_pos.unsigned_abs());
/// assert_eq!(u_pos, i_neg.unsigned_abs());
/// assert_eq!(u_max, i_min.unsigned_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn unsigned_abs(self) -> std::num::NonZeroU64;

"#,
    r#"
/// Returns `true` if `self` is positive and `false` if the
/// number is negative.
/// # Example
/// ```
///# use std::num::NonZeroI64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI64::new(5)?;
///let neg_five = NonZeroI64::new(-5)?;
/// assert!(pos_five.is_positive());
/// assert!(!neg_five.is_positive());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_positive(self) -> bool;

"#,
    r#"
/// Returns `true` if `self` is negative and `false` if the
/// number is positive.
/// # Example
/// ```
///# use std::num::NonZeroI64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI64::new(5)?;
///let neg_five = NonZeroI64::new(-5)?;
/// assert!(neg_five.is_negative());
/// assert!(!pos_five.is_negative());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_negative(self) -> bool;

"#,
    r#"
/// Saturating negation. Computes `-self`,
///returning [`NonZeroI64::MAX`]
///if `self == NonZeroI64::MIN`
/// instead of overflowing.
/// # Example
/// ```
///# use std::num::NonZeroI64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI64::new(5)?;
///let neg_five = NonZeroI64::new(-5)?;
///let min = NonZeroI64::new(i64::MIN)?;
///let min_plus_one = NonZeroI64::new(i64::MIN + 1)?;
///let max = NonZeroI64::new(i64::MAX)?;
/// assert_eq!(pos_five.saturating_neg(), neg_five);
/// assert_eq!(min.saturating_neg(), max);
/// assert_eq!(max.saturating_neg(), min_plus_one);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_neg(self) -> std::num::NonZeroI64;

"#,
    r#"
/// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
/// of the type.
///See [`i64::wrapping_neg`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroI64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI64::new(5)?;
///let neg_five = NonZeroI64::new(-5)?;
///let min = NonZeroI64::new(i64::MIN)?;
/// assert_eq!(pos_five.wrapping_neg(), neg_five);
/// assert_eq!(min.wrapping_neg(), min);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_neg(self) -> std::num::NonZeroI64;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroI64::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroI64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroI64::new(2)?;
///let four = NonZeroI64::new(4)?;
///let max = NonZeroI64::new(i64::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(
        self,
        #[proxy]
        other: std::num::NonZeroI64,
    ) -> std::num::NonZeroI64;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroI64::MIN`] or [`NonZeroI64::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroI64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroI64::new(3)?;
///let twenty_seven = NonZeroI64::new(27)?;
///let max = NonZeroI64::new(i64::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroI64;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> std::num::NonZeroI64;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroI64) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroI64;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct NonZeroI64();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroI8>` is the same size as `i8`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroI8>>(), size_of::<i8>());
/// ```
/// # Layout
///`NonZeroI8` is guaranteed to have the same layout and bit validity as `i8`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroI8>` is guaranteed to be compatible with `i8`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroI8` and `Option<NonZeroI8>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroI8;
///assert_eq!(size_of::<NonZeroI8>(), size_of::<Option<NonZeroI8>>());
///assert_eq!(align_of::<NonZeroI8>(), align_of::<Option<NonZeroI8>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroI8",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> std::num::NonZeroI8;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroI8) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroI8;

"#,
    r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: i8) -> std::num::NonZeroI8;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> i8;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroI8::new(-1i8).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroI8::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Computes the absolute value of self.
///See [`i8::abs`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroI8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI8::new(1)?;
///let neg = NonZeroI8::new(-1)?;
/// assert_eq!(pos, pos.abs());
/// assert_eq!(pos, neg.abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> std::num::NonZeroI8;

"#,
    r#"
/// Saturating absolute value, see
///[`i8::saturating_abs`].
/// # Example
/// ```
///# use std::num::NonZeroI8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI8::new(1)?;
///let neg = NonZeroI8::new(-1)?;
///let min = NonZeroI8::new(i8::MIN)?;
///let min_plus = NonZeroI8::new(i8::MIN + 1)?;
///let max = NonZeroI8::new(i8::MAX)?;
/// assert_eq!(pos, pos.saturating_abs());
/// assert_eq!(pos, neg.saturating_abs());
/// assert_eq!(max, min.saturating_abs());
/// assert_eq!(max, min_plus.saturating_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_abs(self) -> std::num::NonZeroI8;

"#,
    r#"
/// Wrapping absolute value, see
///[`i8::wrapping_abs`].
/// # Example
/// ```
///# use std::num::NonZeroI8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroI8::new(1)?;
///let neg = NonZeroI8::new(-1)?;
///let min = NonZeroI8::new(i8::MIN)?;
///# let max = NonZeroI8::new(i8::MAX)?;
/// assert_eq!(pos, pos.wrapping_abs());
/// assert_eq!(pos, neg.wrapping_abs());
/// assert_eq!(min, min.wrapping_abs());
/// assert_eq!(max, (-max).wrapping_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_abs(self) -> std::num::NonZeroI8;

"#,
    r#"
/// Computes the absolute value of self
/// without any wrapping or panicking.
/// # Example
/// ```
///# use std::num::NonZeroI8;
///# use std::num::NonZeroU8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let u_pos = NonZeroU8::new(1)?;
///let i_pos = NonZeroI8::new(1)?;
///let i_neg = NonZeroI8::new(-1)?;
///let i_min = NonZeroI8::new(i8::MIN)?;
///let u_max = NonZeroU8::new(u8::MAX / 2 + 1)?;
/// assert_eq!(u_pos, i_pos.unsigned_abs());
/// assert_eq!(u_pos, i_neg.unsigned_abs());
/// assert_eq!(u_max, i_min.unsigned_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn unsigned_abs(self) -> std::num::NonZeroU8;

"#,
    r#"
/// Returns `true` if `self` is positive and `false` if the
/// number is negative.
/// # Example
/// ```
///# use std::num::NonZeroI8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI8::new(5)?;
///let neg_five = NonZeroI8::new(-5)?;
/// assert!(pos_five.is_positive());
/// assert!(!neg_five.is_positive());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_positive(self) -> bool;

"#,
    r#"
/// Returns `true` if `self` is negative and `false` if the
/// number is positive.
/// # Example
/// ```
///# use std::num::NonZeroI8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI8::new(5)?;
///let neg_five = NonZeroI8::new(-5)?;
/// assert!(neg_five.is_negative());
/// assert!(!pos_five.is_negative());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_negative(self) -> bool;

"#,
    r#"
/// Saturating negation. Computes `-self`,
///returning [`NonZeroI8::MAX`]
///if `self == NonZeroI8::MIN`
/// instead of overflowing.
/// # Example
/// ```
///# use std::num::NonZeroI8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI8::new(5)?;
///let neg_five = NonZeroI8::new(-5)?;
///let min = NonZeroI8::new(i8::MIN)?;
///let min_plus_one = NonZeroI8::new(i8::MIN + 1)?;
///let max = NonZeroI8::new(i8::MAX)?;
/// assert_eq!(pos_five.saturating_neg(), neg_five);
/// assert_eq!(min.saturating_neg(), max);
/// assert_eq!(max.saturating_neg(), min_plus_one);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_neg(self) -> std::num::NonZeroI8;

"#,
    r#"
/// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
/// of the type.
///See [`i8::wrapping_neg`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroI8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroI8::new(5)?;
///let neg_five = NonZeroI8::new(-5)?;
///let min = NonZeroI8::new(i8::MIN)?;
/// assert_eq!(pos_five.wrapping_neg(), neg_five);
/// assert_eq!(min.wrapping_neg(), min);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_neg(self) -> std::num::NonZeroI8;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroI8::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroI8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroI8::new(2)?;
///let four = NonZeroI8::new(4)?;
///let max = NonZeroI8::new(i8::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] other: std::num::NonZeroI8) -> std::num::NonZeroI8;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroI8::MIN`] or [`NonZeroI8::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroI8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroI8::new(3)?;
///let twenty_seven = NonZeroI8::new(27)?;
///let max = NonZeroI8::new(i8::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroI8;

"#]
)]
pub struct NonZeroI8();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroU128>` is the same size as `u128`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroU128>>(), size_of::<u128>());
/// ```
/// # Layout
///`NonZeroU128` is guaranteed to have the same layout and bit validity as `u128`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroU128>` is guaranteed to be compatible with `u128`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroU128` and `Option<NonZeroU128>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroU128;
///assert_eq!(size_of::<NonZeroU128>(), size_of::<Option<NonZeroU128>>());
///assert_eq!(align_of::<NonZeroU128>(), align_of::<Option<NonZeroU128>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroU128",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroU128;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroU128) -> bool;

"#,
    r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: u128) -> std::num::NonZeroU128;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> u128;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroU128::new(u128::MAX).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroU128::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Adds an unsigned integer to a non-zero value.
///Return [`NonZeroU128::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let one = NonZeroU128::new(1)?;
///let two = NonZeroU128::new(2)?;
///let max = NonZeroU128::new(u128::MAX)?;
/// assert_eq!(two, one.saturating_add(1));
/// assert_eq!(max, max.saturating_add(1));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, other: u128) -> std::num::NonZeroU128;

"#,
    r#"
/// Returns the base 2 logarithm of the number, rounded down.
/// This is the same operation as
///[`u128::ilog2`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroU128;
///assert_eq!(NonZeroU128::new(7).unwrap().ilog2(), 2);
///assert_eq!(NonZeroU128::new(8).unwrap().ilog2(), 3);
///assert_eq!(NonZeroU128::new(9).unwrap().ilog2(), 3);
/// ```

    #[lua(kind = "Method")]
    fn ilog2(self) -> u32;

"#,
    r#"
/// Returns the base 10 logarithm of the number, rounded down.
/// This is the same operation as
///[`u128::ilog10`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroU128;
///assert_eq!(NonZeroU128::new(99).unwrap().ilog10(), 1);
///assert_eq!(NonZeroU128::new(100).unwrap().ilog10(), 2);
///assert_eq!(NonZeroU128::new(101).unwrap().ilog10(), 2);
/// ```

    #[lua(kind = "Method")]
    fn ilog10(self) -> u32;

"#,
    r#"
/// Returns `true` if and only if `self == (1 << k)` for some `k`.
/// On many architectures, this function can perform better than `is_power_of_two()`
/// on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let eight = std::num::NonZeroU128::new(8).unwrap();
/// assert!(eight.is_power_of_two());
///let ten = std::num::NonZeroU128::new(10).unwrap();
/// assert!(!ten.is_power_of_two());
/// ```

    #[lua(kind = "Method")]
    fn is_power_of_two(self) -> bool;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroU128::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroU128::new(2)?;
///let four = NonZeroU128::new(4)?;
///let max = NonZeroU128::new(u128::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(
        self,
        #[proxy]
        other: std::num::NonZeroU128,
    ) -> std::num::NonZeroU128;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroU128::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU128;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroU128::new(3)?;
///let twenty_seven = NonZeroU128::new(27)?;
///let max = NonZeroU128::new(u128::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroU128;

"#]
)]
pub struct NonZeroU128();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroU16>` is the same size as `u16`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroU16>>(), size_of::<u16>());
/// ```
/// # Layout
///`NonZeroU16` is guaranteed to have the same layout and bit validity as `u16`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroU16>` is guaranteed to be compatible with `u16`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroU16` and `Option<NonZeroU16>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroU16;
///assert_eq!(size_of::<NonZeroU16>(), size_of::<Option<NonZeroU16>>());
///assert_eq!(align_of::<NonZeroU16>(), align_of::<Option<NonZeroU16>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroU16",
    functions[r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: u16) -> std::num::NonZeroU16;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> u16;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroU16::new(u16::MAX).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroU16::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Adds an unsigned integer to a non-zero value.
///Return [`NonZeroU16::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let one = NonZeroU16::new(1)?;
///let two = NonZeroU16::new(2)?;
///let max = NonZeroU16::new(u16::MAX)?;
/// assert_eq!(two, one.saturating_add(1));
/// assert_eq!(max, max.saturating_add(1));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, other: u16) -> std::num::NonZeroU16;

"#,
    r#"
/// Returns the base 2 logarithm of the number, rounded down.
/// This is the same operation as
///[`u16::ilog2`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroU16;
///assert_eq!(NonZeroU16::new(7).unwrap().ilog2(), 2);
///assert_eq!(NonZeroU16::new(8).unwrap().ilog2(), 3);
///assert_eq!(NonZeroU16::new(9).unwrap().ilog2(), 3);
/// ```

    #[lua(kind = "Method")]
    fn ilog2(self) -> u32;

"#,
    r#"
/// Returns the base 10 logarithm of the number, rounded down.
/// This is the same operation as
///[`u16::ilog10`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroU16;
///assert_eq!(NonZeroU16::new(99).unwrap().ilog10(), 1);
///assert_eq!(NonZeroU16::new(100).unwrap().ilog10(), 2);
///assert_eq!(NonZeroU16::new(101).unwrap().ilog10(), 2);
/// ```

    #[lua(kind = "Method")]
    fn ilog10(self) -> u32;

"#,
    r#"
/// Returns `true` if and only if `self == (1 << k)` for some `k`.
/// On many architectures, this function can perform better than `is_power_of_two()`
/// on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let eight = std::num::NonZeroU16::new(8).unwrap();
/// assert!(eight.is_power_of_two());
///let ten = std::num::NonZeroU16::new(10).unwrap();
/// assert!(!ten.is_power_of_two());
/// ```

    #[lua(kind = "Method")]
    fn is_power_of_two(self) -> bool;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroU16::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroU16::new(2)?;
///let four = NonZeroU16::new(4)?;
///let max = NonZeroU16::new(u16::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(
        self,
        #[proxy]
        other: std::num::NonZeroU16,
    ) -> std::num::NonZeroU16;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroU16::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU16;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroU16::new(3)?;
///let twenty_seven = NonZeroU16::new(27)?;
///let max = NonZeroU16::new(u16::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroU16;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroU16) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroU16;

"#]
)]
pub struct NonZeroU16();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroU32>` is the same size as `u32`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroU32>>(), size_of::<u32>());
/// ```
/// # Layout
///`NonZeroU32` is guaranteed to have the same layout and bit validity as `u32`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroU32>` is guaranteed to be compatible with `u32`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroU32` and `Option<NonZeroU32>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroU32;
///assert_eq!(size_of::<NonZeroU32>(), size_of::<Option<NonZeroU32>>());
///assert_eq!(align_of::<NonZeroU32>(), align_of::<Option<NonZeroU32>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroU32",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroU32) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: u32) -> std::num::NonZeroU32;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> u32;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroU32::new(u32::MAX).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroU32::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Adds an unsigned integer to a non-zero value.
///Return [`NonZeroU32::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let one = NonZeroU32::new(1)?;
///let two = NonZeroU32::new(2)?;
///let max = NonZeroU32::new(u32::MAX)?;
/// assert_eq!(two, one.saturating_add(1));
/// assert_eq!(max, max.saturating_add(1));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, other: u32) -> std::num::NonZeroU32;

"#,
    r#"
/// Returns the base 2 logarithm of the number, rounded down.
/// This is the same operation as
///[`u32::ilog2`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroU32;
///assert_eq!(NonZeroU32::new(7).unwrap().ilog2(), 2);
///assert_eq!(NonZeroU32::new(8).unwrap().ilog2(), 3);
///assert_eq!(NonZeroU32::new(9).unwrap().ilog2(), 3);
/// ```

    #[lua(kind = "Method")]
    fn ilog2(self) -> u32;

"#,
    r#"
/// Returns the base 10 logarithm of the number, rounded down.
/// This is the same operation as
///[`u32::ilog10`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroU32;
///assert_eq!(NonZeroU32::new(99).unwrap().ilog10(), 1);
///assert_eq!(NonZeroU32::new(100).unwrap().ilog10(), 2);
///assert_eq!(NonZeroU32::new(101).unwrap().ilog10(), 2);
/// ```

    #[lua(kind = "Method")]
    fn ilog10(self) -> u32;

"#,
    r#"
/// Returns `true` if and only if `self == (1 << k)` for some `k`.
/// On many architectures, this function can perform better than `is_power_of_two()`
/// on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let eight = std::num::NonZeroU32::new(8).unwrap();
/// assert!(eight.is_power_of_two());
///let ten = std::num::NonZeroU32::new(10).unwrap();
/// assert!(!ten.is_power_of_two());
/// ```

    #[lua(kind = "Method")]
    fn is_power_of_two(self) -> bool;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroU32::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroU32::new(2)?;
///let four = NonZeroU32::new(4)?;
///let max = NonZeroU32::new(u32::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(
        self,
        #[proxy]
        other: std::num::NonZeroU32,
    ) -> std::num::NonZeroU32;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroU32::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU32;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroU32::new(3)?;
///let twenty_seven = NonZeroU32::new(27)?;
///let max = NonZeroU32::new(u32::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroU32;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroU32;

"#]
)]
pub struct NonZeroU32();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroU64>` is the same size as `u64`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroU64>>(), size_of::<u64>());
/// ```
/// # Layout
///`NonZeroU64` is guaranteed to have the same layout and bit validity as `u64`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroU64>` is guaranteed to be compatible with `u64`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroU64` and `Option<NonZeroU64>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroU64;
///assert_eq!(size_of::<NonZeroU64>(), size_of::<Option<NonZeroU64>>());
///assert_eq!(align_of::<NonZeroU64>(), align_of::<Option<NonZeroU64>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroU64",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroU64) -> bool;

"#,
    r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: u64) -> std::num::NonZeroU64;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> u64;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroU64::new(u64::MAX).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroU64::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Adds an unsigned integer to a non-zero value.
///Return [`NonZeroU64::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let one = NonZeroU64::new(1)?;
///let two = NonZeroU64::new(2)?;
///let max = NonZeroU64::new(u64::MAX)?;
/// assert_eq!(two, one.saturating_add(1));
/// assert_eq!(max, max.saturating_add(1));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, other: u64) -> std::num::NonZeroU64;

"#,
    r#"
/// Returns the base 2 logarithm of the number, rounded down.
/// This is the same operation as
///[`u64::ilog2`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroU64;
///assert_eq!(NonZeroU64::new(7).unwrap().ilog2(), 2);
///assert_eq!(NonZeroU64::new(8).unwrap().ilog2(), 3);
///assert_eq!(NonZeroU64::new(9).unwrap().ilog2(), 3);
/// ```

    #[lua(kind = "Method")]
    fn ilog2(self) -> u32;

"#,
    r#"
/// Returns the base 10 logarithm of the number, rounded down.
/// This is the same operation as
///[`u64::ilog10`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroU64;
///assert_eq!(NonZeroU64::new(99).unwrap().ilog10(), 1);
///assert_eq!(NonZeroU64::new(100).unwrap().ilog10(), 2);
///assert_eq!(NonZeroU64::new(101).unwrap().ilog10(), 2);
/// ```

    #[lua(kind = "Method")]
    fn ilog10(self) -> u32;

"#,
    r#"
/// Returns `true` if and only if `self == (1 << k)` for some `k`.
/// On many architectures, this function can perform better than `is_power_of_two()`
/// on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let eight = std::num::NonZeroU64::new(8).unwrap();
/// assert!(eight.is_power_of_two());
///let ten = std::num::NonZeroU64::new(10).unwrap();
/// assert!(!ten.is_power_of_two());
/// ```

    #[lua(kind = "Method")]
    fn is_power_of_two(self) -> bool;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroU64::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroU64::new(2)?;
///let four = NonZeroU64::new(4)?;
///let max = NonZeroU64::new(u64::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(
        self,
        #[proxy]
        other: std::num::NonZeroU64,
    ) -> std::num::NonZeroU64;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroU64::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU64;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroU64::new(3)?;
///let twenty_seven = NonZeroU64::new(27)?;
///let max = NonZeroU64::new(u64::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroU64;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroU64;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct NonZeroU64();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroU8>` is the same size as `u8`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroU8>>(), size_of::<u8>());
/// ```
/// # Layout
///`NonZeroU8` is guaranteed to have the same layout and bit validity as `u8`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroU8>` is guaranteed to be compatible with `u8`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroU8` and `Option<NonZeroU8>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroU8;
///assert_eq!(size_of::<NonZeroU8>(), size_of::<Option<NonZeroU8>>());
///assert_eq!(align_of::<NonZeroU8>(), align_of::<Option<NonZeroU8>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroU8",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroU8;

"#,
    r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: u8) -> std::num::NonZeroU8;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> u8;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroU8::new(u8::MAX).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroU8::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Adds an unsigned integer to a non-zero value.
///Return [`NonZeroU8::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let one = NonZeroU8::new(1)?;
///let two = NonZeroU8::new(2)?;
///let max = NonZeroU8::new(u8::MAX)?;
/// assert_eq!(two, one.saturating_add(1));
/// assert_eq!(max, max.saturating_add(1));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, other: u8) -> std::num::NonZeroU8;

"#,
    r#"
/// Returns the base 2 logarithm of the number, rounded down.
/// This is the same operation as
///[`u8::ilog2`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroU8;
///assert_eq!(NonZeroU8::new(7).unwrap().ilog2(), 2);
///assert_eq!(NonZeroU8::new(8).unwrap().ilog2(), 3);
///assert_eq!(NonZeroU8::new(9).unwrap().ilog2(), 3);
/// ```

    #[lua(kind = "Method")]
    fn ilog2(self) -> u32;

"#,
    r#"
/// Returns the base 10 logarithm of the number, rounded down.
/// This is the same operation as
///[`u8::ilog10`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroU8;
///assert_eq!(NonZeroU8::new(99).unwrap().ilog10(), 1);
///assert_eq!(NonZeroU8::new(100).unwrap().ilog10(), 2);
///assert_eq!(NonZeroU8::new(101).unwrap().ilog10(), 2);
/// ```

    #[lua(kind = "Method")]
    fn ilog10(self) -> u32;

"#,
    r#"
/// Returns `true` if and only if `self == (1 << k)` for some `k`.
/// On many architectures, this function can perform better than `is_power_of_two()`
/// on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let eight = std::num::NonZeroU8::new(8).unwrap();
/// assert!(eight.is_power_of_two());
///let ten = std::num::NonZeroU8::new(10).unwrap();
/// assert!(!ten.is_power_of_two());
/// ```

    #[lua(kind = "Method")]
    fn is_power_of_two(self) -> bool;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroU8::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroU8::new(2)?;
///let four = NonZeroU8::new(4)?;
///let max = NonZeroU8::new(u8::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] other: std::num::NonZeroU8) -> std::num::NonZeroU8;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroU8::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroU8;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroU8::new(3)?;
///let twenty_seven = NonZeroU8::new(27)?;
///let max = NonZeroU8::new(u8::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroU8;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroU8) -> bool;

"#]
)]
pub struct NonZeroU8();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroUsize>` is the same size as `usize`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroUsize>>(), size_of::<usize>());
/// ```
/// # Layout
///`NonZeroUsize` is guaranteed to have the same layout and bit validity as `usize`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroUsize>` is guaranteed to be compatible with `usize`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroUsize` and `Option<NonZeroUsize>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroUsize;
///assert_eq!(size_of::<NonZeroUsize>(), size_of::<Option<NonZeroUsize>>());
///assert_eq!(align_of::<NonZeroUsize>(), align_of::<Option<NonZeroUsize>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroUsize",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: usize) -> std::num::NonZeroUsize;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> usize;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroUsize::new(usize::MAX).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroUsize::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Adds an unsigned integer to a non-zero value.
///Return [`NonZeroUsize::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroUsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let one = NonZeroUsize::new(1)?;
///let two = NonZeroUsize::new(2)?;
///let max = NonZeroUsize::new(usize::MAX)?;
/// assert_eq!(two, one.saturating_add(1));
/// assert_eq!(max, max.saturating_add(1));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, other: usize) -> std::num::NonZeroUsize;

"#,
    r#"
/// Returns the base 2 logarithm of the number, rounded down.
/// This is the same operation as
///[`usize::ilog2`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroUsize;
///assert_eq!(NonZeroUsize::new(7).unwrap().ilog2(), 2);
///assert_eq!(NonZeroUsize::new(8).unwrap().ilog2(), 3);
///assert_eq!(NonZeroUsize::new(9).unwrap().ilog2(), 3);
/// ```

    #[lua(kind = "Method")]
    fn ilog2(self) -> u32;

"#,
    r#"
/// Returns the base 10 logarithm of the number, rounded down.
/// This is the same operation as
///[`usize::ilog10`],
/// except that it has no failure cases to worry about
/// since this value can never be zero.
/// # Examples
/// ```
///# use std::num::NonZeroUsize;
///assert_eq!(NonZeroUsize::new(99).unwrap().ilog10(), 1);
///assert_eq!(NonZeroUsize::new(100).unwrap().ilog10(), 2);
///assert_eq!(NonZeroUsize::new(101).unwrap().ilog10(), 2);
/// ```

    #[lua(kind = "Method")]
    fn ilog10(self) -> u32;

"#,
    r#"
/// Returns `true` if and only if `self == (1 << k)` for some `k`.
/// On many architectures, this function can perform better than `is_power_of_two()`
/// on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let eight = std::num::NonZeroUsize::new(8).unwrap();
/// assert!(eight.is_power_of_two());
///let ten = std::num::NonZeroUsize::new(10).unwrap();
/// assert!(!ten.is_power_of_two());
/// ```

    #[lua(kind = "Method")]
    fn is_power_of_two(self) -> bool;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroUsize::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroUsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroUsize::new(2)?;
///let four = NonZeroUsize::new(4)?;
///let max = NonZeroUsize::new(usize::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(
        self,
        #[proxy]
        other: std::num::NonZeroUsize,
    ) -> std::num::NonZeroUsize;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroUsize::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroUsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroUsize::new(3)?;
///let twenty_seven = NonZeroUsize::new(27)?;
///let max = NonZeroUsize::new(usize::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroUsize;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroUsize) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroUsize;

"#]
)]
pub struct NonZeroUsize();
/// An owned, mutable path (akin to [`String`]).
/// This type provides methods like [`push`] and [`set_extension`] that mutate
/// the path in place. It also implements [`Deref`] to [`Path`], meaning that
/// all methods on [`Path`] slices are available on `PathBuf` values as well.
/// [`push`]: PathBuf::push
/// [`set_extension`]: PathBuf::set_extension
/// More details about the overall approach can be found in
/// the [module documentation](self).
/// # Examples
/// You can use [`push`] to build up a `PathBuf` from
/// components:
/// ```
/// use std::path::PathBuf;
/// let mut path = PathBuf::new();
/// path.push(r"C:\");
/// path.push("windows");
/// path.push("system32");
/// path.set_extension("dll");
/// ```
/// However, [`push`] is best used for dynamic situations. This is a better way
/// to do this when you know all of the components ahead of time:
/// ```
/// use std::path::PathBuf;
/// let path: PathBuf = [r"C:\", "windows", "system32.dll"].iter().collect();
/// ```
/// We can still do better than this! Since these are all strings, we can use
/// `From::from`:
/// ```
/// use std::path::PathBuf;
/// let path = PathBuf::from(r"C:\windows\system32.dll");
/// ```
/// Which method works best depends on what kind of situation you're in.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::path::PathBuf",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::path::PathBuf) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::path::PathBuf;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "MutatingMethod",
    )]
    fn clone_from(&mut self, #[proxy] source: &std::path::PathBuf) -> ();

"#,
    r#"
/// Allocates an empty `PathBuf`.
/// # Examples
/// ```
/// use std::path::PathBuf;
/// let path = PathBuf::new();
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn new() -> std::path::PathBuf;

"#,
    r#"
/// Creates a new `PathBuf` with a given capacity used to create the
/// internal [`OsString`]. See [`with_capacity`] defined on [`OsString`].
/// # Examples
/// ```
/// use std::path::PathBuf;
/// let mut path = PathBuf::with_capacity(10);
/// let capacity = path.capacity();
/// // This push is done without reallocating
/// path.push(r"C:\");
/// assert_eq!(capacity, path.capacity());
/// ```
/// [`with_capacity`]: OsString::with_capacity

    #[lua(kind = "Function", output(proxy))]
    fn with_capacity(capacity: usize) -> std::path::PathBuf;

"#,
    r#"
/// Truncates `self` to [`self.parent`].
/// Returns `false` and does nothing if [`self.parent`] is [`None`].
/// Otherwise, returns `true`.
/// [`self.parent`]: Path::parent
/// # Examples
/// ```
/// use std::path::{Path, PathBuf};
/// let mut p = PathBuf::from("/spirited/away.rs");
/// p.pop();
/// assert_eq!(Path::new("/spirited"), p);
/// p.pop();
/// assert_eq!(Path::new("/"), p);
/// ```

    #[lua(kind = "MutatingMethod")]
    fn pop(&mut self) -> bool;

"#,
    r#"
/// Invokes [`capacity`] on the underlying instance of [`OsString`].
/// [`capacity`]: OsString::capacity

    #[lua(kind = "Method")]
    fn capacity(&self) -> usize;

"#,
    r#"
/// Invokes [`clear`] on the underlying instance of [`OsString`].
/// [`clear`]: OsString::clear

    #[lua(kind = "MutatingMethod")]
    fn clear(&mut self) -> ();

"#,
    r#"
/// Invokes [`reserve`] on the underlying instance of [`OsString`].
/// [`reserve`]: OsString::reserve

    #[lua(kind = "MutatingMethod")]
    fn reserve(&mut self, additional: usize) -> ();

"#,
    r#"
/// Invokes [`reserve_exact`] on the underlying instance of [`OsString`].
/// [`reserve_exact`]: OsString::reserve_exact

    #[lua(kind = "MutatingMethod")]
    fn reserve_exact(&mut self, additional: usize) -> ();

"#,
    r#"
/// Invokes [`shrink_to_fit`] on the underlying instance of [`OsString`].
/// [`shrink_to_fit`]: OsString::shrink_to_fit

    #[lua(kind = "MutatingMethod")]
    fn shrink_to_fit(&mut self) -> ();

"#,
    r#"
/// Invokes [`shrink_to`] on the underlying instance of [`OsString`].
/// [`shrink_to`]: OsString::shrink_to

    #[lua(kind = "MutatingMethod")]
    fn shrink_to(&mut self, min_capacity: usize) -> ();

"#]
)]
pub struct PathBuf {}
/// An unbounded range (`..`).
/// `RangeFull` is primarily used as a [slicing index], its shorthand is `..`.
/// It cannot serve as an [`Iterator`] because it doesn't have a starting point.
/// # Examples
/// The `..` syntax is a `RangeFull`:
/// ```
/// assert_eq!(.., std::ops::RangeFull);
/// ```
/// It does not have an [`IntoIterator`] implementation, so you can't use it in
/// a `for` loop directly. This won't compile:
/// ```compile_fail,E0277
/// for i in .. {
///     // ...
/// }
/// ```
/// Used as a [slicing index], `RangeFull` produces the full array as a slice.
/// ```
/// let arr = [0, 1, 2, 3, 4];
/// assert_eq!(arr[ ..  ], [0, 1, 2, 3, 4]); // This is the `RangeFull`
/// assert_eq!(arr[ .. 3], [0, 1, 2      ]);
/// assert_eq!(arr[ ..=3], [0, 1, 2, 3   ]);
/// assert_eq!(arr[1..  ], [   1, 2, 3, 4]);
/// assert_eq!(arr[1.. 3], [   1, 2      ]);
/// assert_eq!(arr[1..=3], [   1, 2, 3   ]);
/// ```
/// [slicing index]: crate::slice::SliceIndex
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::ops::RangeFull",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::ops::RangeFull;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::ops::RangeFull) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct RangeFull {}
/// A quaternion representing an orientation.
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
/// SIMD vector types are used for storage on supported platforms.
/// This type is 16 byte aligned.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Quat",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::Quat) -> bool;

"#,
    r#"
/// Subtracts the `rhs` quaternion from `self`.
/// The difference is not guaranteed to be normalized.

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::Quat) -> bevy::math::Quat;

"#,
    r#"
/// Divides a quaternion by a scalar value.
/// The quotient is not guaranteed to be normalized.

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: f32) -> bevy::math::Quat;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Multiplies two quaternions. If they each represent a rotation, the result will
/// represent the combined rotation.
/// Note that due to floating point rounding the result may not be perfectly
/// normalized.
/// # Panics
/// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Quat) -> bevy::math::Quat;

"#,
    r#"
/// Adds two quaternions.
/// The sum is not guaranteed to be normalized.
/// Note that addition is not the same as combining the rotations represented by the
/// two quaternions! That corresponds to multiplication.

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::Quat) -> bevy::math::Quat;

"#,
    r#"
/// Multiplies a quaternion by a scalar value.
/// The product is not guaranteed to be normalized.

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f32) -> bevy::math::Quat;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Quat;

"#,
    r#"
/// Multiplies a quaternion and a 3D vector, returning the rotated vector.
/// # Panics
/// Will panic if `self` is not normalized when `glam_assert` is enabled.

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Creates a new rotation quaternion.
/// This should generally not be called manually unless you know what you are doing.
/// Use one of the other constructors instead such as `identity` or `from_axis_angle`.
/// `from_xyzw` is mostly used by unit tests and `serde` deserialization.
/// # Preconditions
/// This function does not check if the input is normalized, it is up to the user to
/// provide normalized input or to normalized the resulting quaternion.

    #[lua(kind = "Function", output(proxy))]
    fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> bevy::math::Quat;

"#,
    r#"
/// Creates a rotation quaternion from an array.
/// # Preconditions
/// This function does not check if the input is normalized, it is up to the user to
/// provide normalized input or to normalized the resulting quaternion.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [f32; 4]) -> bevy::math::Quat;

"#,
    r#"
/// Creates a new rotation quaternion from a 4D vector.
/// # Preconditions
/// This function does not check if the input is normalized, it is up to the user to
/// provide normalized input or to normalized the resulting quaternion.

    #[lua(kind = "Function", output(proxy))]
    fn from_vec4(#[proxy] v: bevy::math::Vec4) -> bevy::math::Quat;

"#,
    r#"
/// Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
/// The axis must be a unit vector.
/// # Panics
/// Will panic if `axis` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_axis_angle(#[proxy] axis: bevy::math::Vec3, angle: f32) -> bevy::math::Quat;

"#,
    r#"
/// Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
/// `from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.

    #[lua(kind = "Function", output(proxy))]
    fn from_scaled_axis(#[proxy] v: bevy::math::Vec3) -> bevy::math::Quat;

"#,
    r#"
/// Creates a quaternion from the `angle` (in radians) around the x axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_x(angle: f32) -> bevy::math::Quat;

"#,
    r#"
/// Creates a quaternion from the `angle` (in radians) around the y axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_y(angle: f32) -> bevy::math::Quat;

"#,
    r#"
/// Creates a quaternion from the `angle` (in radians) around the z axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_z(angle: f32) -> bevy::math::Quat;

"#,
    r#"
/// Creates a quaternion from the given Euler rotation sequence and the angles (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_euler(
        #[proxy]
        euler: bevy::math::EulerRot,
        a: f32,
        b: f32,
        c: f32,
    ) -> bevy::math::Quat;

"#,
    r#"
/// Creates a quaternion from a 3x3 rotation matrix.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3(#[proxy] mat: &glam::Mat3) -> bevy::math::Quat;

"#,
    r#"
/// Creates a quaternion from a 3x3 SIMD aligned rotation matrix.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3a(#[proxy] mat: &glam::Mat3A) -> bevy::math::Quat;

"#,
    r#"
/// Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat4(#[proxy] mat: &glam::Mat4) -> bevy::math::Quat;

"#,
    r#"
/// Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
/// plane spanned by the two vectors.  Will rotate at most 180 degrees.
/// The inputs must be unit vectors.
/// `from_rotation_arc(from, to) * from ≈ to`.
/// For near-singular cases (from≈to and from≈-to) the current implementation
/// is only accurate to about 0.001 (for `f32`).
/// # Panics
/// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_arc(
        #[proxy]
        from: bevy::math::Vec3,
        #[proxy]
        to: bevy::math::Vec3,
    ) -> bevy::math::Quat;

"#,
    r#"
/// Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
/// that the resulting quaternion will rotate `from` so that it is colinear with `to`.
/// The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
/// degrees.
/// The inputs must be unit vectors.
/// `to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
/// # Panics
/// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_arc_colinear(
        #[proxy]
        from: bevy::math::Vec3,
        #[proxy]
        to: bevy::math::Vec3,
    ) -> bevy::math::Quat;

"#,
    r#"
/// Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
/// around the z axis. Will rotate at most 180 degrees.
/// The inputs must be unit vectors.
/// `from_rotation_arc_2d(from, to) * from ≈ to`.
/// For near-singular cases (from≈to and from≈-to) the current implementation
/// is only accurate to about 0.001 (for `f32`).
/// # Panics
/// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_arc_2d(
        #[proxy]
        from: bevy::math::Vec2,
        #[proxy]
        to: bevy::math::Vec2,
    ) -> bevy::math::Quat;

"#,
    r#"
/// Returns the rotation axis scaled by the rotation in radians.

    #[lua(kind = "Method", output(proxy))]
    fn to_scaled_axis(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns the rotation angles for the given euler rotation sequence.

    #[lua(kind = "Method")]
    fn to_euler(self, #[proxy] euler: bevy::math::EulerRot) -> (f32, f32, f32);

"#,
    r#"
/// `[x, y, z, w]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [f32; 4];

"#,
    r#"
/// Returns the vector part of the quaternion.

    #[lua(kind = "Method", output(proxy))]
    fn xyz(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns the quaternion conjugate of `self`. For a unit quaternion the
/// conjugate is also the inverse.

    #[lua(kind = "Method", output(proxy))]
    fn conjugate(self) -> bevy::math::Quat;

"#,
    r#"
/// Returns the inverse of a normalized quaternion.
/// Typically quaternion inverse returns the conjugate of a normalized quaternion.
/// Because `self` is assumed to already be unit length this method *does not* normalize
/// before returning the conjugate.
/// # Panics
/// Will panic if `self` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(self) -> bevy::math::Quat;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`. The dot product is
/// equal to the cosine of the angle between two quaternion rotations.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::Quat) -> f32;

"#,
    r#"
/// Computes the length of `self`.

    #[lua(kind = "Method")]
    fn length(self) -> f32;

"#,
    r#"
/// Computes the squared length of `self`.
/// This is generally faster than `length()` as it avoids a square
/// root operation.

    #[lua(kind = "Method")]
    fn length_squared(self) -> f32;

"#,
    r#"
/// Computes `1.0 / length()`.
/// For valid results, `self` must _not_ be of length zero.

    #[lua(kind = "Method")]
    fn length_recip(self) -> f32;

"#,
    r#"
/// Returns `self` normalized to length 1.0.
/// For valid results, `self` must _not_ be of length zero.
/// Panics
/// Will panic if `self` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn normalize(self) -> bevy::math::Quat;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(self) -> bool;

"#,
    r#"

    #[lua(kind = "Method")]
    fn is_nan(self) -> bool;

"#,
    r#"
/// Returns whether `self` of length `1.0` or not.
/// Uses a precision threshold of `1e-6`.

    #[lua(kind = "Method")]
    fn is_normalized(self) -> bool;

"#,
    r#"

    #[lua(kind = "Method")]
    fn is_near_identity(self) -> bool;

"#,
    r#"
/// Returns the angle (in radians) for the minimal rotation
/// for transforming this quaternion into another.
/// Both quaternions must be normalized.
/// # Panics
/// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method")]
    fn angle_between(self, #[proxy] rhs: bevy::math::Quat) -> f32;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two quaternions contain similar elements. It works
/// best when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(self, #[proxy] rhs: bevy::math::Quat, max_abs_diff: f32) -> bool;

"#,
    r#"
/// Performs a linear interpolation between `self` and `rhs` based on
/// the value `s`.
/// When `s` is `0.0`, the result will be equal to `self`.  When `s`
/// is `1.0`, the result will be equal to `rhs`.
/// # Panics
/// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn lerp(self, #[proxy] end: bevy::math::Quat, s: f32) -> bevy::math::Quat;

"#,
    r#"
/// Performs a spherical linear interpolation between `self` and `end`
/// based on the value `s`.
/// When `s` is `0.0`, the result will be equal to `self`.  When `s`
/// is `1.0`, the result will be equal to `end`.
/// # Panics
/// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn slerp(self, #[proxy] end: bevy::math::Quat, s: f32) -> bevy::math::Quat;

"#,
    r#"
/// Multiplies a quaternion and a 3D vector, returning the rotated vector.
/// # Panics
/// Will panic if `self` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec3(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Multiplies two quaternions. If they each represent a rotation, the result will
/// represent the combined rotation.
/// Note that due to floating point rounding the result may not be perfectly normalized.
/// # Panics
/// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn mul_quat(self, #[proxy] rhs: bevy::math::Quat) -> bevy::math::Quat;

"#,
    r#"
/// Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.

    #[lua(kind = "Function", output(proxy))]
    fn from_affine3(#[proxy] a: &glam::Affine3A) -> bevy::math::Quat;

"#,
    r#"
/// Multiplies a quaternion and a 3D vector, returning the rotated vector.

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec3a(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_dquat(self) -> bevy::math::DQuat;

"#,
    r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_f64(self) -> bevy::math::DQuat;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::Quat;

"#,
    r#"
/// Rotates the [`Direction3d`] using a [`Quat`].

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(
        self,
        #[proxy]
        direction: bevy::math::primitives::Direction3d,
    ) -> bevy::math::primitives::Direction3d;

"#]
)]
pub struct Quat();
/// A 3-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Vec3",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f32) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::Vec3) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: f32) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: f32) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: f32) -> bevy::math::Vec3;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: f32, y: f32, z: f32) -> bevy::math::Vec3;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: f32) -> bevy::math::Vec3;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec3,
        #[proxy]
        if_true: bevy::math::Vec3,
        #[proxy]
        if_false: bevy::math::Vec3,
    ) -> bevy::math::Vec3;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [f32; 3]) -> bevy::math::Vec3;

"#,
    r#"
/// `[x, y, z]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [f32; 3];

"#,
    r#"
/// Creates a 4D vector from `self` and the given `w` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, w: f32) -> bevy::math::Vec4;

"#,
    r#"
/// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
/// Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::Vec2;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::Vec3) -> f32;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Computes the cross product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn cross(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Component-wise clamping of values, similar to [`f32::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::Vec3,
        #[proxy]
        max: bevy::math::Vec3,
    ) -> bevy::math::Vec3;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> f32;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> f32;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
/// - `1.0` if the number is positive, `+0.0` or `INFINITY`
/// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
/// - `NAN` if the number is `NAN`

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector with signs of `rhs` and the magnitudes of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn copysign(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.  If any element is either
/// `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(self) -> bool;

"#,
    r#"
/// Performs `is_nan` on each element of self, returning a vector mask of the results.
/// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

    #[lua(kind = "Method", output(proxy))]
    fn is_nan_mask(self) -> bevy::math::BVec3;

"#,
    r#"
/// Computes the length of `self`.

    #[lua(kind = "Method")]
    fn length(self) -> f32;

"#,
    r#"
/// Computes the squared length of `self`.
/// This is faster than `length()` as it avoids a square root operation.

    #[lua(kind = "Method")]
    fn length_squared(self) -> f32;

"#,
    r#"
/// Computes `1.0 / length()`.
/// For valid results, `self` must _not_ be of length zero.

    #[lua(kind = "Method")]
    fn length_recip(self) -> f32;

"#,
    r#"
/// Computes the Euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance(self, #[proxy] rhs: bevy::math::Vec3) -> f32;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::Vec3) -> f32;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// [Euclidean division]: f32::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Returns `self` normalized to length 1.0.
/// For valid results, `self` must _not_ be of length zero, nor very close to zero.
/// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
/// Panics
/// Will panic if `self` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn normalize(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns `self` normalized to length 1.0 if possible, else returns zero.
/// In particular, if the input is zero (or very close to zero), or non-finite,
/// the result of this operation will be zero.
/// See also [`Self::try_normalize()`].

    #[lua(kind = "Method", output(proxy))]
    fn normalize_or_zero(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns whether `self` is length `1.0` or not.
/// Uses a precision threshold of `1e-6`.

    #[lua(kind = "Method")]
    fn is_normalized(self) -> bool;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto_normalized(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from_normalized(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector containing the nearest integer to a number for each element of `self`.
/// Round half-way cases away from 0.0.

    #[lua(kind = "Method", output(proxy))]
    fn round(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector containing the largest integer less than or equal to a number for each
/// element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn floor(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector containing the smallest integer greater than or equal to a number for
/// each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn ceil(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector containing the integer part each element of `self`. This means numbers are
/// always truncated towards zero.

    #[lua(kind = "Method", output(proxy))]
    fn trunc(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector containing the fractional part of the vector, e.g. `self -
/// self.floor()`.
/// Note that this is fast but not precise for large numbers.

    #[lua(kind = "Method", output(proxy))]
    fn fract(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector containing `e^self` (the exponential function) for each element of
/// `self`.

    #[lua(kind = "Method", output(proxy))]
    fn exp(self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector containing each element of `self` raised to the power of `n`.

    #[lua(kind = "Method", output(proxy))]
    fn powf(self, n: f32) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn recip(self) -> bevy::math::Vec3;

"#,
    r#"
/// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
/// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
/// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
/// extrapolated.

    #[lua(kind = "Method", output(proxy))]
    fn lerp(self, #[proxy] rhs: bevy::math::Vec3, s: f32) -> bevy::math::Vec3;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs` is
/// less than or equal to `max_abs_diff`.
/// This can be used to compare if two vectors contain similar elements. It works best when
/// comparing with a known value. The `max_abs_diff` that should be used used depends on
/// the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(self, #[proxy] rhs: bevy::math::Vec3, max_abs_diff: f32) -> bool;

"#,
    r#"
/// Returns a vector with a length no less than `min` and no more than `max`
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length(self, min: f32, max: f32) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector with a length no more than `max`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_max(self, max: f32) -> bevy::math::Vec3;

"#,
    r#"
/// Returns a vector with a length no less than `min`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_min(self, min: f32) -> bevy::math::Vec3;

"#,
    r#"
/// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
/// error, yielding a more accurate result than an unfused multiply-add.
/// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
/// architecture has a dedicated fma CPU instruction. However, this is not always true,
/// and will be heavily dependant on designing algorithms with specific target hardware in
/// mind.

    #[lua(kind = "Method", output(proxy))]
    fn mul_add(
        self,
        #[proxy]
        a: bevy::math::Vec3,
        #[proxy]
        b: bevy::math::Vec3,
    ) -> bevy::math::Vec3;

"#,
    r#"
/// Returns the angle (in radians) between two vectors.
/// The inputs do not need to be unit vectors however they must be non-zero.

    #[lua(kind = "Method")]
    fn angle_between(self, #[proxy] rhs: bevy::math::Vec3) -> f32;

"#,
    r#"
/// Returns some vector that is orthogonal to the given one.
/// The input vector must be finite and non-zero.
/// The output vector is not necessarily unit length. For that use
/// [`Self::any_orthonormal_vector()`] instead.

    #[lua(kind = "Method", output(proxy))]
    fn any_orthogonal_vector(&self) -> bevy::math::Vec3;

"#,
    r#"
/// Returns any unit vector that is orthogonal to the given one.
/// The input vector must be unit length.
/// # Panics
/// Will panic if `self` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn any_orthonormal_vector(&self) -> bevy::math::Vec3;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec3(&self) -> bevy::math::DVec3;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec3(&self) -> bevy::math::IVec3;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec3(&self) -> bevy::math::UVec3;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec3(&self) -> bevy::math::I64Vec3;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec3(&self) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: f32) -> bevy::math::Vec3;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<f32,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: f32) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
/// A 2-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::IVec2",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::IVec2) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::IVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: i32) -> bevy::math::IVec2;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::IVec2;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: i32, y: i32) -> bevy::math::IVec2;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: i32) -> bevy::math::IVec2;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec2,
        #[proxy]
        if_true: bevy::math::IVec2,
        #[proxy]
        if_false: bevy::math::IVec2,
    ) -> bevy::math::IVec2;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [i32; 2]) -> bevy::math::IVec2;

"#,
    r#"
/// `[x, y]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [i32; 2];

"#,
    r#"
/// Creates a 3D vector from `self` and the given `z` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, z: i32) -> bevy::math::IVec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::IVec2) -> i32;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Component-wise clamping of values, similar to [`i32::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::IVec2,
        #[proxy]
        max: bevy::math::IVec2,
    ) -> bevy::math::IVec2;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> i32;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> i32;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
///  - `0` if the number is zero
///  - `1` if the number is positive
///  - `-1` if the number is negative

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> i32;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::IVec2) -> i32;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.
/// [Euclidean division]: i32::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a vector that is equal to `self` rotated by 90 degrees.

    #[lua(kind = "Method", output(proxy))]
    fn perp(self) -> bevy::math::IVec2;

"#,
    r#"
/// The perpendicular dot product of `self` and `rhs`.
/// Also known as the wedge product, 2D cross product, and determinant.

    #[lua(kind = "Method")]
    fn perp_dot(self, #[proxy] rhs: bevy::math::IVec2) -> i32;

"#,
    r#"
/// Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
/// then this just rotation. This is what you usually want. Otherwise,
/// it will be like a rotation with a multiplication by `self`'s length.

    #[lua(kind = "Method", output(proxy))]
    fn rotate(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec2(&self) -> bevy::math::Vec2;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec2(&self) -> bevy::math::DVec2;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec2(&self) -> bevy::math::UVec2;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec2(&self) -> bevy::math::I64Vec2;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec2(&self) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: i32) -> bevy::math::IVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: i32) -> bevy::math::IVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: i32) -> bevy::math::IVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: i32) -> bevy::math::IVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::IVec2) -> bevy::math::IVec2;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<i32,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: i32) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct IVec2 {
    x: i32,
    y: i32,
}
/// A 3-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::IVec3",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::IVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: i32) -> bevy::math::IVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: i32) -> bevy::math::IVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::IVec3) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: i32) -> bevy::math::IVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: i32) -> bevy::math::IVec3;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: i32, y: i32, z: i32) -> bevy::math::IVec3;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: i32) -> bevy::math::IVec3;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec3,
        #[proxy]
        if_true: bevy::math::IVec3,
        #[proxy]
        if_false: bevy::math::IVec3,
    ) -> bevy::math::IVec3;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [i32; 3]) -> bevy::math::IVec3;

"#,
    r#"
/// `[x, y, z]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [i32; 3];

"#,
    r#"
/// Creates a 4D vector from `self` and the given `w` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, w: i32) -> bevy::math::IVec4;

"#,
    r#"
/// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
/// Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::IVec2;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::IVec3) -> i32;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Computes the cross product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn cross(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Component-wise clamping of values, similar to [`i32::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::IVec3,
        #[proxy]
        max: bevy::math::IVec3,
    ) -> bevy::math::IVec3;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> i32;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> i32;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::IVec3;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
///  - `0` if the number is zero
///  - `1` if the number is positive
///  - `-1` if the number is negative

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::IVec3;

"#,
    r#"
/// Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> i32;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::IVec3) -> i32;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.
/// [Euclidean division]: i32::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec3(&self) -> bevy::math::Vec3;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec3a(&self) -> bevy::math::Vec3A;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec3(&self) -> bevy::math::DVec3;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec3(&self) -> bevy::math::UVec3;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec3(&self) -> bevy::math::I64Vec3;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec3(&self) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: i32) -> bevy::math::IVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::IVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::IVec3) -> bevy::math::IVec3;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<i32,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: i32) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct IVec3 {
    x: i32,
    y: i32,
    z: i32,
}
/// A 4-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::IVec4",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: i32, y: i32, z: i32, w: i32) -> bevy::math::IVec4;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: i32) -> bevy::math::IVec4;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec4,
        #[proxy]
        if_true: bevy::math::IVec4,
        #[proxy]
        if_false: bevy::math::IVec4,
    ) -> bevy::math::IVec4;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [i32; 4]) -> bevy::math::IVec4;

"#,
    r#"
/// `[x, y, z, w]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [i32; 4];

"#,
    r#"
/// Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
/// Truncation to [`IVec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::IVec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::IVec4) -> i32;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Component-wise clamping of values, similar to [`i32::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::IVec4,
        #[proxy]
        max: bevy::math::IVec4,
    ) -> bevy::math::IVec4;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> i32;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> i32;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::IVec4;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
///  - `0` if the number is zero
///  - `1` if the number is positive
///  - `-1` if the number is negative

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::IVec4;

"#,
    r#"
/// Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> i32;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::IVec4) -> i32;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.
/// [Euclidean division]: i32::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec4(&self) -> bevy::math::Vec4;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec4(&self) -> bevy::math::DVec4;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec4(&self) -> bevy::math::UVec4;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec4(&self) -> bevy::math::I64Vec4;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec4(&self) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: i32) -> bevy::math::IVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: i32) -> bevy::math::IVec4;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::IVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: i32) -> bevy::math::IVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: i32) -> bevy::math::IVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: i32) -> bevy::math::IVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::IVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::IVec4) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::IVec4) -> bevy::math::IVec4;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<i32,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: i32) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct IVec4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}
/// A 2-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::I64Vec2",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: i64) -> bevy::math::I64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::I64Vec2) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::I64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: i64) -> bevy::math::I64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::I64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: i64) -> bevy::math::I64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: i64) -> bevy::math::I64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: i64) -> bevy::math::I64Vec2;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: i64, y: i64) -> bevy::math::I64Vec2;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: i64) -> bevy::math::I64Vec2;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec2,
        #[proxy]
        if_true: bevy::math::I64Vec2,
        #[proxy]
        if_false: bevy::math::I64Vec2,
    ) -> bevy::math::I64Vec2;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [i64; 2]) -> bevy::math::I64Vec2;

"#,
    r#"
/// `[x, y]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [i64; 2];

"#,
    r#"
/// Creates a 3D vector from `self` and the given `z` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, z: i64) -> bevy::math::I64Vec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::I64Vec2) -> i64;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Component-wise clamping of values, similar to [`i64::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::I64Vec2,
        #[proxy]
        max: bevy::math::I64Vec2,
    ) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> i64;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> i64;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
///  - `0` if the number is zero
///  - `1` if the number is positive
///  - `-1` if the number is negative

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> i64;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::I64Vec2) -> i64;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.
/// [Euclidean division]: i64::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector that is equal to `self` rotated by 90 degrees.

    #[lua(kind = "Method", output(proxy))]
    fn perp(self) -> bevy::math::I64Vec2;

"#,
    r#"
/// The perpendicular dot product of `self` and `rhs`.
/// Also known as the wedge product, 2D cross product, and determinant.

    #[lua(kind = "Method")]
    fn perp_dot(self, #[proxy] rhs: bevy::math::I64Vec2) -> i64;

"#,
    r#"
/// Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
/// then this just rotation. This is what you usually want. Otherwise,
/// it will be like a rotation with a multiplication by `self`'s length.

    #[lua(kind = "Method", output(proxy))]
    fn rotate(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec2(&self) -> bevy::math::Vec2;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec2(&self) -> bevy::math::DVec2;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec2(&self) -> bevy::math::IVec2;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec2(&self) -> bevy::math::UVec2;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec2(&self) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::I64Vec2) -> bevy::math::I64Vec2;

"#]
)]
pub struct I64Vec2 {
    x: i64,
    y: i64,
}
/// A 3-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::I64Vec3",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: i64) -> bevy::math::I64Vec3;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: i64, y: i64, z: i64) -> bevy::math::I64Vec3;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: i64) -> bevy::math::I64Vec3;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec3,
        #[proxy]
        if_true: bevy::math::I64Vec3,
        #[proxy]
        if_false: bevy::math::I64Vec3,
    ) -> bevy::math::I64Vec3;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [i64; 3]) -> bevy::math::I64Vec3;

"#,
    r#"
/// `[x, y, z]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [i64; 3];

"#,
    r#"
/// Creates a 4D vector from `self` and the given `w` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, w: i64) -> bevy::math::I64Vec4;

"#,
    r#"
/// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
/// Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::I64Vec2;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::I64Vec3) -> i64;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Computes the cross product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn cross(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Component-wise clamping of values, similar to [`i64::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::I64Vec3,
        #[proxy]
        max: bevy::math::I64Vec3,
    ) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> i64;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> i64;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
///  - `0` if the number is zero
///  - `1` if the number is positive
///  - `-1` if the number is negative

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> i64;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::I64Vec3) -> i64;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.
/// [Euclidean division]: i64::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec3(&self) -> bevy::math::Vec3;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec3a(&self) -> bevy::math::Vec3A;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec3(&self) -> bevy::math::DVec3;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec3(&self) -> bevy::math::IVec3;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec3(&self) -> bevy::math::UVec3;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec3(&self) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: i64) -> bevy::math::I64Vec3;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::I64Vec3) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::I64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: i64) -> bevy::math::I64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::I64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: i64) -> bevy::math::I64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::I64Vec3) -> bevy::math::I64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: i64) -> bevy::math::I64Vec3;

"#]
)]
pub struct I64Vec3 {
    x: i64,
    y: i64,
    z: i64,
}
/// A 4-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::I64Vec4",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: i64) -> bevy::math::I64Vec4;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: i64) -> bevy::math::I64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: i64) -> bevy::math::I64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::I64Vec4) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::I64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: i64) -> bevy::math::I64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: i64) -> bevy::math::I64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::I64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: i64, y: i64, z: i64, w: i64) -> bevy::math::I64Vec4;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: i64) -> bevy::math::I64Vec4;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec4,
        #[proxy]
        if_true: bevy::math::I64Vec4,
        #[proxy]
        if_false: bevy::math::I64Vec4,
    ) -> bevy::math::I64Vec4;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [i64; 4]) -> bevy::math::I64Vec4;

"#,
    r#"
/// `[x, y, z, w]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [i64; 4];

"#,
    r#"
/// Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
/// Truncation to [`I64Vec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::I64Vec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::I64Vec4) -> i64;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Component-wise clamping of values, similar to [`i64::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::I64Vec4,
        #[proxy]
        max: bevy::math::I64Vec4,
    ) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> i64;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> i64;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
///  - `0` if the number is zero
///  - `1` if the number is positive
///  - `-1` if the number is negative

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> i64;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::I64Vec4) -> i64;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// # Panics
/// This function will panic if any `rhs` element is 0 or the division results in overflow.
/// [Euclidean division]: i64::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec4(&self) -> bevy::math::Vec4;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec4(&self) -> bevy::math::DVec4;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec4(&self) -> bevy::math::IVec4;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec4(&self) -> bevy::math::UVec4;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec4(&self) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::I64Vec4) -> bevy::math::I64Vec4;

"#]
)]
pub struct I64Vec4 {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}
/// A 2-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::UVec2",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: u32) -> bevy::math::UVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: u32) -> bevy::math::UVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::UVec2) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: u32, y: u32) -> bevy::math::UVec2;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: u32) -> bevy::math::UVec2;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec2,
        #[proxy]
        if_true: bevy::math::UVec2,
        #[proxy]
        if_false: bevy::math::UVec2,
    ) -> bevy::math::UVec2;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [u32; 2]) -> bevy::math::UVec2;

"#,
    r#"
/// `[x, y]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [u32; 2];

"#,
    r#"
/// Creates a 3D vector from `self` and the given `z` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, z: u32) -> bevy::math::UVec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::UVec2) -> u32;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"
/// Component-wise clamping of values, similar to [`u32::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::UVec2,
        #[proxy]
        max: bevy::math::UVec2,
    ) -> bevy::math::UVec2;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> u32;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> u32;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> u32;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec2(&self) -> bevy::math::Vec2;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec2(&self) -> bevy::math::DVec2;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec2(&self) -> bevy::math::IVec2;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec2(&self) -> bevy::math::I64Vec2;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec2(&self) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: u32) -> bevy::math::UVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: u32) -> bevy::math::UVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::UVec2) -> bevy::math::UVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: u32) -> bevy::math::UVec2;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::UVec2;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<u32,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: u32) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct UVec2 {
    x: u32,
    y: u32,
}
/// A 3-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::UVec3",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: u32) -> bevy::math::UVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::UVec3) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: u32) -> bevy::math::UVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: u32, y: u32, z: u32) -> bevy::math::UVec3;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: u32) -> bevy::math::UVec3;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec3,
        #[proxy]
        if_true: bevy::math::UVec3,
        #[proxy]
        if_false: bevy::math::UVec3,
    ) -> bevy::math::UVec3;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [u32; 3]) -> bevy::math::UVec3;

"#,
    r#"
/// `[x, y, z]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [u32; 3];

"#,
    r#"
/// Creates a 4D vector from `self` and the given `w` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, w: u32) -> bevy::math::UVec4;

"#,
    r#"
/// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
/// Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::UVec2;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::UVec3) -> u32;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Computes the cross product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn cross(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Component-wise clamping of values, similar to [`u32::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::UVec3,
        #[proxy]
        max: bevy::math::UVec3,
    ) -> bevy::math::UVec3;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> u32;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> u32;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> u32;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec3(&self) -> bevy::math::Vec3;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec3a(&self) -> bevy::math::Vec3A;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec3(&self) -> bevy::math::DVec3;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec3(&self) -> bevy::math::IVec3;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec3(&self) -> bevy::math::I64Vec3;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec3(&self) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::UVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: u32) -> bevy::math::UVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: u32) -> bevy::math::UVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::UVec3) -> bevy::math::UVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: u32) -> bevy::math::UVec3;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<u32,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: u32) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct UVec3 {
    x: u32,
    y: u32,
    z: u32,
}
/// A 4-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::UVec4",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: u32) -> bevy::math::UVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: u32) -> bevy::math::UVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: u32) -> bevy::math::UVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: u32, y: u32, z: u32, w: u32) -> bevy::math::UVec4;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: u32) -> bevy::math::UVec4;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec4,
        #[proxy]
        if_true: bevy::math::UVec4,
        #[proxy]
        if_false: bevy::math::UVec4,
    ) -> bevy::math::UVec4;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [u32; 4]) -> bevy::math::UVec4;

"#,
    r#"
/// `[x, y, z, w]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [u32; 4];

"#,
    r#"
/// Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
/// Truncation to [`UVec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::UVec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::UVec4) -> u32;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"
/// Component-wise clamping of values, similar to [`u32::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::UVec4,
        #[proxy]
        max: bevy::math::UVec4,
    ) -> bevy::math::UVec4;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> u32;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> u32;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> u32;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec4(&self) -> bevy::math::Vec4;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec4(&self) -> bevy::math::DVec4;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec4(&self) -> bevy::math::IVec4;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec4(&self) -> bevy::math::I64Vec4;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec4(&self) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::UVec4;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: u32) -> bevy::math::UVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::UVec4) -> bevy::math::UVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: u32) -> bevy::math::UVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::UVec4) -> bool;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<u32,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: u32) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct UVec4 {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}
/// A 2-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::U64Vec2",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: u64) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: u64) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: u64) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: u64) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::U64Vec2) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: u64) -> bevy::math::U64Vec2;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: u64, y: u64) -> bevy::math::U64Vec2;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: u64) -> bevy::math::U64Vec2;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec2,
        #[proxy]
        if_true: bevy::math::U64Vec2,
        #[proxy]
        if_false: bevy::math::U64Vec2,
    ) -> bevy::math::U64Vec2;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [u64; 2]) -> bevy::math::U64Vec2;

"#,
    r#"
/// `[x, y]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [u64; 2];

"#,
    r#"
/// Creates a 3D vector from `self` and the given `z` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, z: u64) -> bevy::math::U64Vec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::U64Vec2) -> u64;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"
/// Component-wise clamping of values, similar to [`u64::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::U64Vec2,
        #[proxy]
        max: bevy::math::U64Vec2,
    ) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> u64;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> u64;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> u64;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec2(&self) -> bevy::math::Vec2;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec2(&self) -> bevy::math::DVec2;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec2(&self) -> bevy::math::IVec2;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec2(&self) -> bevy::math::UVec2;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec2(&self) -> bevy::math::I64Vec2;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::U64Vec2) -> bevy::math::U64Vec2;

"#]
)]
pub struct U64Vec2 {
    x: u64,
    y: u64,
}
/// A 3-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::U64Vec3",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: u64) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::U64Vec3) -> bool;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: u64, y: u64, z: u64) -> bevy::math::U64Vec3;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: u64) -> bevy::math::U64Vec3;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec3,
        #[proxy]
        if_true: bevy::math::U64Vec3,
        #[proxy]
        if_false: bevy::math::U64Vec3,
    ) -> bevy::math::U64Vec3;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [u64; 3]) -> bevy::math::U64Vec3;

"#,
    r#"
/// `[x, y, z]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [u64; 3];

"#,
    r#"
/// Creates a 4D vector from `self` and the given `w` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, w: u64) -> bevy::math::U64Vec4;

"#,
    r#"
/// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
/// Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::U64Vec2;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::U64Vec3) -> u64;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"
/// Computes the cross product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn cross(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"
/// Component-wise clamping of values, similar to [`u64::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::U64Vec3,
        #[proxy]
        max: bevy::math::U64Vec3,
    ) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> u64;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> u64;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::BVec3;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> u64;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec3(&self) -> bevy::math::Vec3;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec3a(&self) -> bevy::math::Vec3A;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec3(&self) -> bevy::math::DVec3;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec3(&self) -> bevy::math::IVec3;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec3(&self) -> bevy::math::UVec3;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec3(&self) -> bevy::math::I64Vec3;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: u64) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: u64) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::U64Vec3) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: u64) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: u64) -> bevy::math::U64Vec3;

"#]
)]
pub struct U64Vec3 {
    x: u64,
    y: u64,
    z: u64,
}
/// A 4-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::U64Vec4",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: u64) -> bevy::math::U64Vec4;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: u64, y: u64, z: u64, w: u64) -> bevy::math::U64Vec4;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: u64) -> bevy::math::U64Vec4;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec4,
        #[proxy]
        if_true: bevy::math::U64Vec4,
        #[proxy]
        if_false: bevy::math::U64Vec4,
    ) -> bevy::math::U64Vec4;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [u64; 4]) -> bevy::math::U64Vec4;

"#,
    r#"
/// `[x, y, z, w]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [u64; 4];

"#,
    r#"
/// Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
/// Truncation to [`U64Vec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::U64Vec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::U64Vec4) -> u64;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"
/// Component-wise clamping of values, similar to [`u64::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::U64Vec4,
        #[proxy]
        max: bevy::math::U64Vec4,
    ) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> u64;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> u64;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::BVec4;

"#,
    r#"
/// Computes the squared length of `self`.

    #[lua(kind = "Method")]
    fn length_squared(self) -> u64;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec4(&self) -> bevy::math::Vec4;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec4(&self) -> bevy::math::DVec4;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec4(&self) -> bevy::math::IVec4;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec4(&self) -> bevy::math::UVec4;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec4(&self) -> bevy::math::I64Vec4;

"#,
    r#"
/// Returns a vector containing the wrapping addition of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_add(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the wrapping subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_sub(rhs.x), self.y.wrapping_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_sub(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the wrapping multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_mul(rhs.x), self.y.wrapping_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_mul(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the wrapping division of `self` and `rhs`.
/// In other words this computes `[self.x.wrapping_div(rhs.x), self.y.wrapping_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_div(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the saturating addition of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_add(rhs.x), self.y.saturating_add(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_add(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the saturating subtraction of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_sub(rhs.x), self.y.saturating_sub(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_sub(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the saturating multiplication of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_mul(rhs.x), self.y.saturating_mul(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"
/// Returns a vector containing the saturating division of `self` and `rhs`.
/// In other words this computes `[self.x.saturating_div(rhs.x), self.y.saturating_div(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn saturating_div(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::U64Vec4) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: u64) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: u64) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: u64) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: u64) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::U64Vec4) -> bevy::math::U64Vec4;

"#]
)]
pub struct U64Vec4 {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}
/// A 2-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Vec2",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: f32) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f32) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: f32) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: f32) -> bevy::math::Vec2;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: f32, y: f32) -> bevy::math::Vec2;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: f32) -> bevy::math::Vec2;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec2,
        #[proxy]
        if_true: bevy::math::Vec2,
        #[proxy]
        if_false: bevy::math::Vec2,
    ) -> bevy::math::Vec2;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [f32; 2]) -> bevy::math::Vec2;

"#,
    r#"
/// `[x, y]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [f32; 2];

"#,
    r#"
/// Creates a 3D vector from `self` and the given `z` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, z: f32) -> bevy::math::Vec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::Vec2) -> f32;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Component-wise clamping of values, similar to [`f32::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::Vec2,
        #[proxy]
        max: bevy::math::Vec2,
    ) -> bevy::math::Vec2;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> f32;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> f32;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
/// - `1.0` if the number is positive, `+0.0` or `INFINITY`
/// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
/// - `NAN` if the number is `NAN`

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector with signs of `rhs` and the magnitudes of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn copysign(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.  If any element is either
/// `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(self) -> bool;

"#,
    r#"
/// Performs `is_nan` on each element of self, returning a vector mask of the results.
/// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

    #[lua(kind = "Method", output(proxy))]
    fn is_nan_mask(self) -> bevy::math::BVec2;

"#,
    r#"
/// Computes the length of `self`.

    #[lua(kind = "Method")]
    fn length(self) -> f32;

"#,
    r#"
/// Computes the squared length of `self`.
/// This is faster than `length()` as it avoids a square root operation.

    #[lua(kind = "Method")]
    fn length_squared(self) -> f32;

"#,
    r#"
/// Computes `1.0 / length()`.
/// For valid results, `self` must _not_ be of length zero.

    #[lua(kind = "Method")]
    fn length_recip(self) -> f32;

"#,
    r#"
/// Computes the Euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance(self, #[proxy] rhs: bevy::math::Vec2) -> f32;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::Vec2) -> f32;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// [Euclidean division]: f32::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Returns `self` normalized to length 1.0.
/// For valid results, `self` must _not_ be of length zero, nor very close to zero.
/// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
/// Panics
/// Will panic if `self` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn normalize(self) -> bevy::math::Vec2;

"#,
    r#"
/// Returns `self` normalized to length 1.0 if possible, else returns zero.
/// In particular, if the input is zero (or very close to zero), or non-finite,
/// the result of this operation will be zero.
/// See also [`Self::try_normalize()`].

    #[lua(kind = "Method", output(proxy))]
    fn normalize_or_zero(self) -> bevy::math::Vec2;

"#,
    r#"
/// Returns whether `self` is length `1.0` or not.
/// Uses a precision threshold of `1e-6`.

    #[lua(kind = "Method")]
    fn is_normalized(self) -> bool;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto_normalized(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from_normalized(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector containing the nearest integer to a number for each element of `self`.
/// Round half-way cases away from 0.0.

    #[lua(kind = "Method", output(proxy))]
    fn round(self) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector containing the largest integer less than or equal to a number for each
/// element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn floor(self) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector containing the smallest integer greater than or equal to a number for
/// each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn ceil(self) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector containing the integer part each element of `self`. This means numbers are
/// always truncated towards zero.

    #[lua(kind = "Method", output(proxy))]
    fn trunc(self) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector containing the fractional part of the vector, e.g. `self -
/// self.floor()`.
/// Note that this is fast but not precise for large numbers.

    #[lua(kind = "Method", output(proxy))]
    fn fract(self) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector containing `e^self` (the exponential function) for each element of
/// `self`.

    #[lua(kind = "Method", output(proxy))]
    fn exp(self) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector containing each element of `self` raised to the power of `n`.

    #[lua(kind = "Method", output(proxy))]
    fn powf(self, n: f32) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn recip(self) -> bevy::math::Vec2;

"#,
    r#"
/// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
/// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
/// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
/// extrapolated.

    #[lua(kind = "Method", output(proxy))]
    fn lerp(self, #[proxy] rhs: bevy::math::Vec2, s: f32) -> bevy::math::Vec2;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs` is
/// less than or equal to `max_abs_diff`.
/// This can be used to compare if two vectors contain similar elements. It works best when
/// comparing with a known value. The `max_abs_diff` that should be used used depends on
/// the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(self, #[proxy] rhs: bevy::math::Vec2, max_abs_diff: f32) -> bool;

"#,
    r#"
/// Returns a vector with a length no less than `min` and no more than `max`
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length(self, min: f32, max: f32) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector with a length no more than `max`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_max(self, max: f32) -> bevy::math::Vec2;

"#,
    r#"
/// Returns a vector with a length no less than `min`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_min(self, min: f32) -> bevy::math::Vec2;

"#,
    r#"
/// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
/// error, yielding a more accurate result than an unfused multiply-add.
/// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
/// architecture has a dedicated fma CPU instruction. However, this is not always true,
/// and will be heavily dependant on designing algorithms with specific target hardware in
/// mind.

    #[lua(kind = "Method", output(proxy))]
    fn mul_add(
        self,
        #[proxy]
        a: bevy::math::Vec2,
        #[proxy]
        b: bevy::math::Vec2,
    ) -> bevy::math::Vec2;

"#,
    r#"
/// Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
/// conjunction with the [`rotate()`][Self::rotate()] method, e.g.
/// `Vec2::from_angle(PI).rotate(Vec2::Y)` will create the vector `[-1, 0]`
/// and rotate [`Vec2::Y`] around it returning `-Vec2::Y`.

    #[lua(kind = "Function", output(proxy))]
    fn from_angle(angle: f32) -> bevy::math::Vec2;

"#,
    r#"
/// Returns the angle (in radians) of this vector in the range `[-π, +π]`.
/// The input does not need to be a unit vector however it must be non-zero.

    #[lua(kind = "Method")]
    fn to_angle(self) -> f32;

"#,
    r#"
/// Returns the angle (in radians) between `self` and `rhs` in the range `[-π, +π]`.
/// The inputs do not need to be unit vectors however they must be non-zero.

    #[lua(kind = "Method")]
    fn angle_between(self, #[proxy] rhs: bevy::math::Vec2) -> f32;

"#,
    r#"
/// Returns a vector that is equal to `self` rotated by 90 degrees.

    #[lua(kind = "Method", output(proxy))]
    fn perp(self) -> bevy::math::Vec2;

"#,
    r#"
/// The perpendicular dot product of `self` and `rhs`.
/// Also known as the wedge product, 2D cross product, and determinant.

    #[lua(kind = "Method")]
    fn perp_dot(self, #[proxy] rhs: bevy::math::Vec2) -> f32;

"#,
    r#"
/// Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
/// then this just rotation. This is what you usually want. Otherwise,
/// it will be like a rotation with a multiplication by `self`'s length.

    #[lua(kind = "Method", output(proxy))]
    fn rotate(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec2(&self) -> bevy::math::DVec2;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec2(&self) -> bevy::math::IVec2;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec2(&self) -> bevy::math::UVec2;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec2(&self) -> bevy::math::I64Vec2;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec2(&self) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: f32) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::Vec2) -> bool;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<f32,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: f32) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct Vec2 {
    x: f32,
    y: f32,
}
/// A 3-dimensional vector.
/// SIMD vector types are used for storage on supported platforms for better
/// performance than the [`Vec3`] type.
/// It is possible to convert between [`Vec3`] and [`Vec3A`] types using [`From`]
/// or [`Into`] trait implementations.
/// This type is 16 byte aligned.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Vec3A",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: f32) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::Vec3A) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: f32) -> bevy::math::Vec3A;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: f32, y: f32, z: f32) -> bevy::math::Vec3A;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: f32) -> bevy::math::Vec3A;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec3A,
        #[proxy]
        if_true: bevy::math::Vec3A,
        #[proxy]
        if_false: bevy::math::Vec3A,
    ) -> bevy::math::Vec3A;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [f32; 3]) -> bevy::math::Vec3A;

"#,
    r#"
/// `[x, y, z]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [f32; 3];

"#,
    r#"
/// Creates a 4D vector from `self` and the given `w` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, w: f32) -> bevy::math::Vec4;

"#,
    r#"
/// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
/// Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::Vec2;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::Vec3A) -> f32;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Computes the cross product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn cross(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Component-wise clamping of values, similar to [`f32::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::Vec3A,
        #[proxy]
        max: bevy::math::Vec3A,
    ) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> f32;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> f32;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::BVec3A;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::BVec3A;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::BVec3A;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::BVec3A;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::BVec3A;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::BVec3A;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
/// - `1.0` if the number is positive, `+0.0` or `INFINITY`
/// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
/// - `NAN` if the number is `NAN`

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector with signs of `rhs` and the magnitudes of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn copysign(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.  If any element is either
/// `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(self) -> bool;

"#,
    r#"
/// Performs `is_nan` on each element of self, returning a vector mask of the results.
/// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

    #[lua(kind = "Method", output(proxy))]
    fn is_nan_mask(self) -> bevy::math::BVec3A;

"#,
    r#"
/// Computes the length of `self`.

    #[lua(kind = "Method")]
    fn length(self) -> f32;

"#,
    r#"
/// Computes the squared length of `self`.
/// This is faster than `length()` as it avoids a square root operation.

    #[lua(kind = "Method")]
    fn length_squared(self) -> f32;

"#,
    r#"
/// Computes `1.0 / length()`.
/// For valid results, `self` must _not_ be of length zero.

    #[lua(kind = "Method")]
    fn length_recip(self) -> f32;

"#,
    r#"
/// Computes the Euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance(self, #[proxy] rhs: bevy::math::Vec3A) -> f32;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::Vec3A) -> f32;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// [Euclidean division]: f32::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns `self` normalized to length 1.0.
/// For valid results, `self` must _not_ be of length zero, nor very close to zero.
/// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
/// Panics
/// Will panic if `self` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn normalize(self) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns `self` normalized to length 1.0 if possible, else returns zero.
/// In particular, if the input is zero (or very close to zero), or non-finite,
/// the result of this operation will be zero.
/// See also [`Self::try_normalize()`].

    #[lua(kind = "Method", output(proxy))]
    fn normalize_or_zero(self) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns whether `self` is length `1.0` or not.
/// Uses a precision threshold of `1e-6`.

    #[lua(kind = "Method")]
    fn is_normalized(self) -> bool;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto_normalized(
        self,
        #[proxy]
        rhs: bevy::math::Vec3A,
    ) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from_normalized(
        self,
        #[proxy]
        rhs: bevy::math::Vec3A,
    ) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector containing the nearest integer to a number for each element of `self`.
/// Round half-way cases away from 0.0.

    #[lua(kind = "Method", output(proxy))]
    fn round(self) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector containing the largest integer less than or equal to a number for each
/// element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn floor(self) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector containing the smallest integer greater than or equal to a number for
/// each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn ceil(self) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector containing the integer part each element of `self`. This means numbers are
/// always truncated towards zero.

    #[lua(kind = "Method", output(proxy))]
    fn trunc(self) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector containing the fractional part of the vector, e.g. `self -
/// self.floor()`.
/// Note that this is fast but not precise for large numbers.

    #[lua(kind = "Method", output(proxy))]
    fn fract(self) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector containing `e^self` (the exponential function) for each element of
/// `self`.

    #[lua(kind = "Method", output(proxy))]
    fn exp(self) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector containing each element of `self` raised to the power of `n`.

    #[lua(kind = "Method", output(proxy))]
    fn powf(self, n: f32) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn recip(self) -> bevy::math::Vec3A;

"#,
    r#"
/// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
/// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
/// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
/// extrapolated.

    #[lua(kind = "Method", output(proxy))]
    fn lerp(self, #[proxy] rhs: bevy::math::Vec3A, s: f32) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs` is
/// less than or equal to `max_abs_diff`.
/// This can be used to compare if two vectors contain similar elements. It works best when
/// comparing with a known value. The `max_abs_diff` that should be used used depends on
/// the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(self, #[proxy] rhs: bevy::math::Vec3A, max_abs_diff: f32) -> bool;

"#,
    r#"
/// Returns a vector with a length no less than `min` and no more than `max`
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length(self, min: f32, max: f32) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector with a length no more than `max`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_max(self, max: f32) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns a vector with a length no less than `min`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_min(self, min: f32) -> bevy::math::Vec3A;

"#,
    r#"
/// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
/// error, yielding a more accurate result than an unfused multiply-add.
/// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
/// architecture has a dedicated fma CPU instruction. However, this is not always true,
/// and will be heavily dependant on designing algorithms with specific target hardware in
/// mind.

    #[lua(kind = "Method", output(proxy))]
    fn mul_add(
        self,
        #[proxy]
        a: bevy::math::Vec3A,
        #[proxy]
        b: bevy::math::Vec3A,
    ) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns the angle (in radians) between two vectors.
/// The inputs do not need to be unit vectors however they must be non-zero.

    #[lua(kind = "Method")]
    fn angle_between(self, #[proxy] rhs: bevy::math::Vec3A) -> f32;

"#,
    r#"
/// Returns some vector that is orthogonal to the given one.
/// The input vector must be finite and non-zero.
/// The output vector is not necessarily unit length. For that use
/// [`Self::any_orthonormal_vector()`] instead.

    #[lua(kind = "Method", output(proxy))]
    fn any_orthogonal_vector(&self) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns any unit vector that is orthogonal to the given one.
/// The input vector must be unit length.
/// # Panics
/// Will panic if `self` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn any_orthonormal_vector(&self) -> bevy::math::Vec3A;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec3(&self) -> bevy::math::DVec3;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec3(&self) -> bevy::math::IVec3;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec3(&self) -> bevy::math::UVec3;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec3(&self) -> bevy::math::I64Vec3;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec3(&self) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: f32) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f32) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: f32) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<f32,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: f32) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct Vec3A();
/// A 4-dimensional vector.
/// SIMD vector types are used for storage on supported platforms.
/// This type is 16 byte aligned.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Vec4",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::Vec4) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: f32) -> bevy::math::Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: f32) -> bevy::math::Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: f32) -> bevy::math::Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: f32) -> bevy::math::Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: f32, y: f32, z: f32, w: f32) -> bevy::math::Vec4;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: f32) -> bevy::math::Vec4;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec4A,
        #[proxy]
        if_true: bevy::math::Vec4,
        #[proxy]
        if_false: bevy::math::Vec4,
    ) -> bevy::math::Vec4;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [f32; 4]) -> bevy::math::Vec4;

"#,
    r#"
/// `[x, y, z, w]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [f32; 4];

"#,
    r#"
/// Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
/// Truncation to [`Vec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].
/// To truncate to [`Vec3A`] use [`Vec3A::from()`].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::Vec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::Vec4) -> f32;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Component-wise clamping of values, similar to [`f32::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::Vec4,
        #[proxy]
        max: bevy::math::Vec4,
    ) -> bevy::math::Vec4;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> f32;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> f32;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::BVec4A;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::BVec4A;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::BVec4A;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::BVec4A;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::BVec4A;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::BVec4A;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
/// - `1.0` if the number is positive, `+0.0` or `INFINITY`
/// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
/// - `NAN` if the number is `NAN`

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector with signs of `rhs` and the magnitudes of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn copysign(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.  If any element is either
/// `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(self) -> bool;

"#,
    r#"
/// Performs `is_nan` on each element of self, returning a vector mask of the results.
/// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

    #[lua(kind = "Method", output(proxy))]
    fn is_nan_mask(self) -> bevy::math::BVec4A;

"#,
    r#"
/// Computes the length of `self`.

    #[lua(kind = "Method")]
    fn length(self) -> f32;

"#,
    r#"
/// Computes the squared length of `self`.
/// This is faster than `length()` as it avoids a square root operation.

    #[lua(kind = "Method")]
    fn length_squared(self) -> f32;

"#,
    r#"
/// Computes `1.0 / length()`.
/// For valid results, `self` must _not_ be of length zero.

    #[lua(kind = "Method")]
    fn length_recip(self) -> f32;

"#,
    r#"
/// Computes the Euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance(self, #[proxy] rhs: bevy::math::Vec4) -> f32;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::Vec4) -> f32;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// [Euclidean division]: f32::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Returns `self` normalized to length 1.0.
/// For valid results, `self` must _not_ be of length zero, nor very close to zero.
/// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
/// Panics
/// Will panic if `self` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn normalize(self) -> bevy::math::Vec4;

"#,
    r#"
/// Returns `self` normalized to length 1.0 if possible, else returns zero.
/// In particular, if the input is zero (or very close to zero), or non-finite,
/// the result of this operation will be zero.
/// See also [`Self::try_normalize()`].

    #[lua(kind = "Method", output(proxy))]
    fn normalize_or_zero(self) -> bevy::math::Vec4;

"#,
    r#"
/// Returns whether `self` is length `1.0` or not.
/// Uses a precision threshold of `1e-6`.

    #[lua(kind = "Method")]
    fn is_normalized(self) -> bool;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto_normalized(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from_normalized(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector containing the nearest integer to a number for each element of `self`.
/// Round half-way cases away from 0.0.

    #[lua(kind = "Method", output(proxy))]
    fn round(self) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector containing the largest integer less than or equal to a number for each
/// element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn floor(self) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector containing the smallest integer greater than or equal to a number for
/// each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn ceil(self) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector containing the integer part each element of `self`. This means numbers are
/// always truncated towards zero.

    #[lua(kind = "Method", output(proxy))]
    fn trunc(self) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector containing the fractional part of the vector, e.g. `self -
/// self.floor()`.
/// Note that this is fast but not precise for large numbers.

    #[lua(kind = "Method", output(proxy))]
    fn fract(self) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector containing `e^self` (the exponential function) for each element of
/// `self`.

    #[lua(kind = "Method", output(proxy))]
    fn exp(self) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector containing each element of `self` raised to the power of `n`.

    #[lua(kind = "Method", output(proxy))]
    fn powf(self, n: f32) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn recip(self) -> bevy::math::Vec4;

"#,
    r#"
/// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
/// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
/// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
/// extrapolated.

    #[lua(kind = "Method", output(proxy))]
    fn lerp(self, #[proxy] rhs: bevy::math::Vec4, s: f32) -> bevy::math::Vec4;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs` is
/// less than or equal to `max_abs_diff`.
/// This can be used to compare if two vectors contain similar elements. It works best when
/// comparing with a known value. The `max_abs_diff` that should be used used depends on
/// the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(self, #[proxy] rhs: bevy::math::Vec4, max_abs_diff: f32) -> bool;

"#,
    r#"
/// Returns a vector with a length no less than `min` and no more than `max`
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length(self, min: f32, max: f32) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector with a length no more than `max`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_max(self, max: f32) -> bevy::math::Vec4;

"#,
    r#"
/// Returns a vector with a length no less than `min`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_min(self, min: f32) -> bevy::math::Vec4;

"#,
    r#"
/// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
/// error, yielding a more accurate result than an unfused multiply-add.
/// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
/// architecture has a dedicated fma CPU instruction. However, this is not always true,
/// and will be heavily dependant on designing algorithms with specific target hardware in
/// mind.

    #[lua(kind = "Method", output(proxy))]
    fn mul_add(
        self,
        #[proxy]
        a: bevy::math::Vec4,
        #[proxy]
        b: bevy::math::Vec4,
    ) -> bevy::math::Vec4;

"#,
    r#"
/// Casts all elements of `self` to `f64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_dvec4(&self) -> bevy::math::DVec4;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec4(&self) -> bevy::math::IVec4;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec4(&self) -> bevy::math::UVec4;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec4(&self) -> bevy::math::I64Vec4;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec4(&self) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f32) -> bevy::math::Vec4;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<f32,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: f32) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct Vec4();
/// A 2-dimensional `bool` vector mask.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::BVec2",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"
/// Creates a new vector mask.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: bool, y: bool) -> bevy::math::BVec2;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: bool) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a bitmask with the lowest 2 bits set from the elements of `self`.
/// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn bitmask(self) -> u32;

"#,
    r#"
/// Returns true if any of the elements are true, false otherwise.

    #[lua(kind = "Method")]
    fn any(self) -> bool;

"#,
    r#"
/// Returns true if all the elements are true, false otherwise.

    #[lua(kind = "Method")]
    fn all(self) -> bool;

"#,
    r#"
/// Tests the value at `index`.
/// Panics if `index` is greater than 1.

    #[lua(kind = "Method")]
    fn test(&self, index: usize) -> bool;

"#,
    r#"
/// Sets the element at `index`.
/// Panics if `index` is greater than 1.

    #[lua(kind = "MutatingMethod")]
    fn set(&mut self, index: usize, value: bool) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::BVec2) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::BVec2;

"#]
)]
pub struct BVec2 {
    x: bool,
    y: bool,
}
/// A 3-dimensional `bool` vector mask.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::BVec3",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::BVec3) -> bool;

"#,
    r#"
/// Creates a new vector mask.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: bool, y: bool, z: bool) -> bevy::math::BVec3;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: bool) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a bitmask with the lowest 3 bits set from the elements of `self`.
/// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn bitmask(self) -> u32;

"#,
    r#"
/// Returns true if any of the elements are true, false otherwise.

    #[lua(kind = "Method")]
    fn any(self) -> bool;

"#,
    r#"
/// Returns true if all the elements are true, false otherwise.

    #[lua(kind = "Method")]
    fn all(self) -> bool;

"#,
    r#"
/// Tests the value at `index`.
/// Panics if `index` is greater than 2.

    #[lua(kind = "Method")]
    fn test(&self, index: usize) -> bool;

"#,
    r#"
/// Sets the element at `index`.
/// Panics if `index` is greater than 2.

    #[lua(kind = "MutatingMethod")]
    fn set(&mut self, index: usize, value: bool) -> ();

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::BVec3;

"#]
)]
pub struct BVec3 {
    x: bool,
    y: bool,
    z: bool,
}
/// A 4-dimensional `bool` vector mask.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::BVec4",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::BVec4;

"#,
    r#"
/// Creates a new vector mask.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: bool, y: bool, z: bool, w: bool) -> bevy::math::BVec4;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: bool) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a bitmask with the lowest 4 bits set from the elements of `self`.
/// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn bitmask(self) -> u32;

"#,
    r#"
/// Returns true if any of the elements are true, false otherwise.

    #[lua(kind = "Method")]
    fn any(self) -> bool;

"#,
    r#"
/// Returns true if all the elements are true, false otherwise.

    #[lua(kind = "Method")]
    fn all(self) -> bool;

"#,
    r#"
/// Tests the value at `index`.
/// Panics if `index` is greater than 3.

    #[lua(kind = "Method")]
    fn test(&self, index: usize) -> bool;

"#,
    r#"
/// Sets the element at `index`.
/// Panics if `index` is greater than 3.

    #[lua(kind = "MutatingMethod")]
    fn set(&mut self, index: usize, value: bool) -> ();

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::BVec4) -> bool;

"#]
)]
pub struct BVec4 {
    x: bool,
    y: bool,
    z: bool,
    w: bool,
}
/// A 2-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DVec2",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::DVec2) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: f64) -> bevy::math::DVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: f64, y: f64) -> bevy::math::DVec2;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: f64) -> bevy::math::DVec2;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec2,
        #[proxy]
        if_true: bevy::math::DVec2,
        #[proxy]
        if_false: bevy::math::DVec2,
    ) -> bevy::math::DVec2;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [f64; 2]) -> bevy::math::DVec2;

"#,
    r#"
/// `[x, y]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [f64; 2];

"#,
    r#"
/// Creates a 3D vector from `self` and the given `z` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, z: f64) -> bevy::math::DVec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::DVec2) -> f64;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Component-wise clamping of values, similar to [`f64::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::DVec2,
        #[proxy]
        max: bevy::math::DVec2,
    ) -> bevy::math::DVec2;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> f64;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> f64;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::BVec2;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
/// - `1.0` if the number is positive, `+0.0` or `INFINITY`
/// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
/// - `NAN` if the number is `NAN`

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector with signs of `rhs` and the magnitudes of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn copysign(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a bitmask with the lowest 2 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.  If any element is either
/// `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(self) -> bool;

"#,
    r#"
/// Performs `is_nan` on each element of self, returning a vector mask of the results.
/// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

    #[lua(kind = "Method", output(proxy))]
    fn is_nan_mask(self) -> bevy::math::BVec2;

"#,
    r#"
/// Computes the length of `self`.

    #[lua(kind = "Method")]
    fn length(self) -> f64;

"#,
    r#"
/// Computes the squared length of `self`.
/// This is faster than `length()` as it avoids a square root operation.

    #[lua(kind = "Method")]
    fn length_squared(self) -> f64;

"#,
    r#"
/// Computes `1.0 / length()`.
/// For valid results, `self` must _not_ be of length zero.

    #[lua(kind = "Method")]
    fn length_recip(self) -> f64;

"#,
    r#"
/// Computes the Euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance(self, #[proxy] rhs: bevy::math::DVec2) -> f64;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::DVec2) -> f64;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// [Euclidean division]: f64::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Returns `self` normalized to length 1.0.
/// For valid results, `self` must _not_ be of length zero, nor very close to zero.
/// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
/// Panics
/// Will panic if `self` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn normalize(self) -> bevy::math::DVec2;

"#,
    r#"
/// Returns `self` normalized to length 1.0 if possible, else returns zero.
/// In particular, if the input is zero (or very close to zero), or non-finite,
/// the result of this operation will be zero.
/// See also [`Self::try_normalize()`].

    #[lua(kind = "Method", output(proxy))]
    fn normalize_or_zero(self) -> bevy::math::DVec2;

"#,
    r#"
/// Returns whether `self` is length `1.0` or not.
/// Uses a precision threshold of `1e-6`.

    #[lua(kind = "Method")]
    fn is_normalized(self) -> bool;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto_normalized(
        self,
        #[proxy]
        rhs: bevy::math::DVec2,
    ) -> bevy::math::DVec2;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from_normalized(
        self,
        #[proxy]
        rhs: bevy::math::DVec2,
    ) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector containing the nearest integer to a number for each element of `self`.
/// Round half-way cases away from 0.0.

    #[lua(kind = "Method", output(proxy))]
    fn round(self) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector containing the largest integer less than or equal to a number for each
/// element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn floor(self) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector containing the smallest integer greater than or equal to a number for
/// each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn ceil(self) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector containing the integer part each element of `self`. This means numbers are
/// always truncated towards zero.

    #[lua(kind = "Method", output(proxy))]
    fn trunc(self) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector containing the fractional part of the vector, e.g. `self -
/// self.floor()`.
/// Note that this is fast but not precise for large numbers.

    #[lua(kind = "Method", output(proxy))]
    fn fract(self) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector containing `e^self` (the exponential function) for each element of
/// `self`.

    #[lua(kind = "Method", output(proxy))]
    fn exp(self) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector containing each element of `self` raised to the power of `n`.

    #[lua(kind = "Method", output(proxy))]
    fn powf(self, n: f64) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn recip(self) -> bevy::math::DVec2;

"#,
    r#"
/// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
/// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
/// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
/// extrapolated.

    #[lua(kind = "Method", output(proxy))]
    fn lerp(self, #[proxy] rhs: bevy::math::DVec2, s: f64) -> bevy::math::DVec2;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs` is
/// less than or equal to `max_abs_diff`.
/// This can be used to compare if two vectors contain similar elements. It works best when
/// comparing with a known value. The `max_abs_diff` that should be used used depends on
/// the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(self, #[proxy] rhs: bevy::math::DVec2, max_abs_diff: f64) -> bool;

"#,
    r#"
/// Returns a vector with a length no less than `min` and no more than `max`
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length(self, min: f64, max: f64) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector with a length no more than `max`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_max(self, max: f64) -> bevy::math::DVec2;

"#,
    r#"
/// Returns a vector with a length no less than `min`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_min(self, min: f64) -> bevy::math::DVec2;

"#,
    r#"
/// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
/// error, yielding a more accurate result than an unfused multiply-add.
/// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
/// architecture has a dedicated fma CPU instruction. However, this is not always true,
/// and will be heavily dependant on designing algorithms with specific target hardware in
/// mind.

    #[lua(kind = "Method", output(proxy))]
    fn mul_add(
        self,
        #[proxy]
        a: bevy::math::DVec2,
        #[proxy]
        b: bevy::math::DVec2,
    ) -> bevy::math::DVec2;

"#,
    r#"
/// Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
/// conjunction with the [`rotate()`][Self::rotate()] method, e.g.
/// `DVec2::from_angle(PI).rotate(DVec2::Y)` will create the vector `[-1, 0]`
/// and rotate [`DVec2::Y`] around it returning `-DVec2::Y`.

    #[lua(kind = "Function", output(proxy))]
    fn from_angle(angle: f64) -> bevy::math::DVec2;

"#,
    r#"
/// Returns the angle (in radians) of this vector in the range `[-π, +π]`.
/// The input does not need to be a unit vector however it must be non-zero.

    #[lua(kind = "Method")]
    fn to_angle(self) -> f64;

"#,
    r#"
/// Returns the angle (in radians) between `self` and `rhs` in the range `[-π, +π]`.
/// The inputs do not need to be unit vectors however they must be non-zero.

    #[lua(kind = "Method")]
    fn angle_between(self, #[proxy] rhs: bevy::math::DVec2) -> f64;

"#,
    r#"
/// Returns a vector that is equal to `self` rotated by 90 degrees.

    #[lua(kind = "Method", output(proxy))]
    fn perp(self) -> bevy::math::DVec2;

"#,
    r#"
/// The perpendicular dot product of `self` and `rhs`.
/// Also known as the wedge product, 2D cross product, and determinant.

    #[lua(kind = "Method")]
    fn perp_dot(self, #[proxy] rhs: bevy::math::DVec2) -> f64;

"#,
    r#"
/// Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
/// then this just rotation. This is what you usually want. Otherwise,
/// it will be like a rotation with a multiplication by `self`'s length.

    #[lua(kind = "Method", output(proxy))]
    fn rotate(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec2(&self) -> bevy::math::Vec2;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec2(&self) -> bevy::math::IVec2;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec2(&self) -> bevy::math::UVec2;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec2(&self) -> bevy::math::I64Vec2;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec2(&self) -> bevy::math::U64Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: f64) -> bevy::math::DVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f64) -> bevy::math::DVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: f64) -> bevy::math::DVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: f64) -> bevy::math::DVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::DVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::DVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<f64,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: f64) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct DVec2 {
    x: f64,
    y: f64,
}
/// A 3-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DVec3",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: f64, y: f64, z: f64) -> bevy::math::DVec3;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: f64) -> bevy::math::DVec3;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec3,
        #[proxy]
        if_true: bevy::math::DVec3,
        #[proxy]
        if_false: bevy::math::DVec3,
    ) -> bevy::math::DVec3;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [f64; 3]) -> bevy::math::DVec3;

"#,
    r#"
/// `[x, y, z]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [f64; 3];

"#,
    r#"
/// Creates a 4D vector from `self` and the given `w` value.

    #[lua(kind = "Method", output(proxy))]
    fn extend(self, w: f64) -> bevy::math::DVec4;

"#,
    r#"
/// Creates a 2D vector from the `x` and `y` elements of `self`, discarding `z`.
/// Truncation may also be performed by using [`self.xy()`][crate::swizzles::Vec3Swizzles::xy()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::DVec2;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::DVec3) -> f64;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Computes the cross product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn cross(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Component-wise clamping of values, similar to [`f64::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::DVec3,
        #[proxy]
        max: bevy::math::DVec3,
    ) -> bevy::math::DVec3;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> f64;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> f64;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::BVec3;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
/// - `1.0` if the number is positive, `+0.0` or `INFINITY`
/// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
/// - `NAN` if the number is `NAN`

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector with signs of `rhs` and the magnitudes of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn copysign(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a bitmask with the lowest 3 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.  If any element is either
/// `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(self) -> bool;

"#,
    r#"
/// Performs `is_nan` on each element of self, returning a vector mask of the results.
/// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

    #[lua(kind = "Method", output(proxy))]
    fn is_nan_mask(self) -> bevy::math::BVec3;

"#,
    r#"
/// Computes the length of `self`.

    #[lua(kind = "Method")]
    fn length(self) -> f64;

"#,
    r#"
/// Computes the squared length of `self`.
/// This is faster than `length()` as it avoids a square root operation.

    #[lua(kind = "Method")]
    fn length_squared(self) -> f64;

"#,
    r#"
/// Computes `1.0 / length()`.
/// For valid results, `self` must _not_ be of length zero.

    #[lua(kind = "Method")]
    fn length_recip(self) -> f64;

"#,
    r#"
/// Computes the Euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance(self, #[proxy] rhs: bevy::math::DVec3) -> f64;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::DVec3) -> f64;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// [Euclidean division]: f64::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Returns `self` normalized to length 1.0.
/// For valid results, `self` must _not_ be of length zero, nor very close to zero.
/// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
/// Panics
/// Will panic if `self` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn normalize(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns `self` normalized to length 1.0 if possible, else returns zero.
/// In particular, if the input is zero (or very close to zero), or non-finite,
/// the result of this operation will be zero.
/// See also [`Self::try_normalize()`].

    #[lua(kind = "Method", output(proxy))]
    fn normalize_or_zero(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns whether `self` is length `1.0` or not.
/// Uses a precision threshold of `1e-6`.

    #[lua(kind = "Method")]
    fn is_normalized(self) -> bool;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto_normalized(
        self,
        #[proxy]
        rhs: bevy::math::DVec3,
    ) -> bevy::math::DVec3;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from_normalized(
        self,
        #[proxy]
        rhs: bevy::math::DVec3,
    ) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector containing the nearest integer to a number for each element of `self`.
/// Round half-way cases away from 0.0.

    #[lua(kind = "Method", output(proxy))]
    fn round(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector containing the largest integer less than or equal to a number for each
/// element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn floor(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector containing the smallest integer greater than or equal to a number for
/// each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn ceil(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector containing the integer part each element of `self`. This means numbers are
/// always truncated towards zero.

    #[lua(kind = "Method", output(proxy))]
    fn trunc(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector containing the fractional part of the vector, e.g. `self -
/// self.floor()`.
/// Note that this is fast but not precise for large numbers.

    #[lua(kind = "Method", output(proxy))]
    fn fract(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector containing `e^self` (the exponential function) for each element of
/// `self`.

    #[lua(kind = "Method", output(proxy))]
    fn exp(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector containing each element of `self` raised to the power of `n`.

    #[lua(kind = "Method", output(proxy))]
    fn powf(self, n: f64) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn recip(self) -> bevy::math::DVec3;

"#,
    r#"
/// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
/// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
/// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
/// extrapolated.

    #[lua(kind = "Method", output(proxy))]
    fn lerp(self, #[proxy] rhs: bevy::math::DVec3, s: f64) -> bevy::math::DVec3;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs` is
/// less than or equal to `max_abs_diff`.
/// This can be used to compare if two vectors contain similar elements. It works best when
/// comparing with a known value. The `max_abs_diff` that should be used used depends on
/// the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(self, #[proxy] rhs: bevy::math::DVec3, max_abs_diff: f64) -> bool;

"#,
    r#"
/// Returns a vector with a length no less than `min` and no more than `max`
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length(self, min: f64, max: f64) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector with a length no more than `max`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_max(self, max: f64) -> bevy::math::DVec3;

"#,
    r#"
/// Returns a vector with a length no less than `min`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_min(self, min: f64) -> bevy::math::DVec3;

"#,
    r#"
/// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
/// error, yielding a more accurate result than an unfused multiply-add.
/// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
/// architecture has a dedicated fma CPU instruction. However, this is not always true,
/// and will be heavily dependant on designing algorithms with specific target hardware in
/// mind.

    #[lua(kind = "Method", output(proxy))]
    fn mul_add(
        self,
        #[proxy]
        a: bevy::math::DVec3,
        #[proxy]
        b: bevy::math::DVec3,
    ) -> bevy::math::DVec3;

"#,
    r#"
/// Returns the angle (in radians) between two vectors.
/// The inputs do not need to be unit vectors however they must be non-zero.

    #[lua(kind = "Method")]
    fn angle_between(self, #[proxy] rhs: bevy::math::DVec3) -> f64;

"#,
    r#"
/// Returns some vector that is orthogonal to the given one.
/// The input vector must be finite and non-zero.
/// The output vector is not necessarily unit length. For that use
/// [`Self::any_orthonormal_vector()`] instead.

    #[lua(kind = "Method", output(proxy))]
    fn any_orthogonal_vector(&self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns any unit vector that is orthogonal to the given one.
/// The input vector must be unit length.
/// # Panics
/// Will panic if `self` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn any_orthonormal_vector(&self) -> bevy::math::DVec3;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec3(&self) -> bevy::math::Vec3;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec3a(&self) -> bevy::math::Vec3A;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec3(&self) -> bevy::math::IVec3;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec3(&self) -> bevy::math::UVec3;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec3(&self) -> bevy::math::I64Vec3;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec3(&self) -> bevy::math::U64Vec3;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::DVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::DVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::DVec3) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f64) -> bevy::math::DVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: f64) -> bevy::math::DVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: f64) -> bevy::math::DVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: f64) -> bevy::math::DVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: f64) -> bevy::math::DVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<f64,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: f64) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct DVec3 {
    x: f64,
    y: f64,
    z: f64,
}
/// A 4-dimensional vector.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DVec4",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, rhs: f64) -> bevy::math::DVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: f64) -> bevy::math::DVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, rhs: f64) -> bevy::math::DVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, rhs: f64) -> bevy::math::DVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::DVec4) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"
/// Creates a new vector.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: f64, y: f64, z: f64, w: f64) -> bevy::math::DVec4;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: f64) -> bevy::math::DVec4;

"#,
    r#"
/// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
/// for each element of `self`.
/// A true element in the mask uses the corresponding element from `if_true`, and false
/// uses the element from `if_false`.

    #[lua(kind = "Function", output(proxy))]
    fn select(
        #[proxy]
        mask: bevy::math::BVec4,
        #[proxy]
        if_true: bevy::math::DVec4,
        #[proxy]
        if_false: bevy::math::DVec4,
    ) -> bevy::math::DVec4;

"#,
    r#"
/// Creates a new vector from an array.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [f64; 4]) -> bevy::math::DVec4;

"#,
    r#"
/// `[x, y, z, w]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [f64; 4];

"#,
    r#"
/// Creates a 3D vector from the `x`, `y` and `z` elements of `self`, discarding `w`.
/// Truncation to [`DVec3`] may also be performed by using [`self.xyz()`][crate::swizzles::Vec4Swizzles::xyz()].

    #[lua(kind = "Method", output(proxy))]
    fn truncate(self) -> bevy::math::DVec3;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::DVec4) -> f64;

"#,
    r#"
/// Returns a vector where every component is the dot product of `self` and `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn dot_into_vec(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector containing the minimum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn min(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector containing the maximum values for each element of `self` and `rhs`.
/// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.

    #[lua(kind = "Method", output(proxy))]
    fn max(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"
/// Component-wise clamping of values, similar to [`f64::clamp`].
/// Each element in `min` must be less-or-equal to the corresponding element in `max`.
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp(
        self,
        #[proxy]
        min: bevy::math::DVec4,
        #[proxy]
        max: bevy::math::DVec4,
    ) -> bevy::math::DVec4;

"#,
    r#"
/// Returns the horizontal minimum of `self`.
/// In other words this computes `min(x, y, ..)`.

    #[lua(kind = "Method")]
    fn min_element(self) -> f64;

"#,
    r#"
/// Returns the horizontal maximum of `self`.
/// In other words this computes `max(x, y, ..)`.

    #[lua(kind = "Method")]
    fn max_element(self) -> f64;

"#,
    r#"
/// Returns a vector mask containing the result of a `==` comparison for each element of
/// `self` and `rhs`.
/// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpeq(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `!=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpne(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `>=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpge(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `>` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmpgt(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `<=` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmple(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector mask containing the result of a `<` comparison for each element of
/// `self` and `rhs`.
/// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
/// elements.

    #[lua(kind = "Method", output(proxy))]
    fn cmplt(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::BVec4;

"#,
    r#"
/// Returns a vector containing the absolute value of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector with elements representing the sign of `self`.
/// - `1.0` if the number is positive, `+0.0` or `INFINITY`
/// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
/// - `NAN` if the number is `NAN`

    #[lua(kind = "Method", output(proxy))]
    fn signum(self) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector with signs of `rhs` and the magnitudes of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn copysign(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a bitmask with the lowest 4 bits set to the sign bits from the elements of `self`.
/// A negative element results in a `1` bit and a positive element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn is_negative_bitmask(self) -> u32;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.  If any element is either
/// `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(self) -> bool;

"#,
    r#"
/// Performs `is_nan` on each element of self, returning a vector mask of the results.
/// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.

    #[lua(kind = "Method", output(proxy))]
    fn is_nan_mask(self) -> bevy::math::BVec4;

"#,
    r#"
/// Computes the length of `self`.

    #[lua(kind = "Method")]
    fn length(self) -> f64;

"#,
    r#"
/// Computes the squared length of `self`.
/// This is faster than `length()` as it avoids a square root operation.

    #[lua(kind = "Method")]
    fn length_squared(self) -> f64;

"#,
    r#"
/// Computes `1.0 / length()`.
/// For valid results, `self` must _not_ be of length zero.

    #[lua(kind = "Method")]
    fn length_recip(self) -> f64;

"#,
    r#"
/// Computes the Euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance(self, #[proxy] rhs: bevy::math::DVec4) -> f64;

"#,
    r#"
/// Compute the squared euclidean distance between two points in space.

    #[lua(kind = "Method")]
    fn distance_squared(self, #[proxy] rhs: bevy::math::DVec4) -> f64;

"#,
    r#"
/// Returns the element-wise quotient of [Euclidean division] of `self` by `rhs`.

    #[lua(kind = "Method", output(proxy))]
    fn div_euclid(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"
/// Returns the element-wise remainder of [Euclidean division] of `self` by `rhs`.
/// [Euclidean division]: f64::rem_euclid

    #[lua(kind = "Method", output(proxy))]
    fn rem_euclid(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"
/// Returns `self` normalized to length 1.0.
/// For valid results, `self` must _not_ be of length zero, nor very close to zero.
/// See also [`Self::try_normalize()`] and [`Self::normalize_or_zero()`].
/// Panics
/// Will panic if `self` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn normalize(self) -> bevy::math::DVec4;

"#,
    r#"
/// Returns `self` normalized to length 1.0 if possible, else returns zero.
/// In particular, if the input is zero (or very close to zero), or non-finite,
/// the result of this operation will be zero.
/// See also [`Self::try_normalize()`].

    #[lua(kind = "Method", output(proxy))]
    fn normalize_or_zero(self) -> bevy::math::DVec4;

"#,
    r#"
/// Returns whether `self` is length `1.0` or not.
/// Uses a precision threshold of `1e-6`.

    #[lua(kind = "Method")]
    fn is_normalized(self) -> bool;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be of non-zero length.
/// # Panics
/// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"
/// Returns the vector projection of `self` onto `rhs`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn project_onto_normalized(
        self,
        #[proxy]
        rhs: bevy::math::DVec4,
    ) -> bevy::math::DVec4;

"#,
    r#"
/// Returns the vector rejection of `self` from `rhs`.
/// The vector rejection is the vector perpendicular to the projection of `self` onto
/// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
/// `rhs` must be normalized.
/// # Panics
/// Will panic if `rhs` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn reject_from_normalized(
        self,
        #[proxy]
        rhs: bevy::math::DVec4,
    ) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector containing the nearest integer to a number for each element of `self`.
/// Round half-way cases away from 0.0.

    #[lua(kind = "Method", output(proxy))]
    fn round(self) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector containing the largest integer less than or equal to a number for each
/// element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn floor(self) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector containing the smallest integer greater than or equal to a number for
/// each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn ceil(self) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector containing the integer part each element of `self`. This means numbers are
/// always truncated towards zero.

    #[lua(kind = "Method", output(proxy))]
    fn trunc(self) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector containing the fractional part of the vector, e.g. `self -
/// self.floor()`.
/// Note that this is fast but not precise for large numbers.

    #[lua(kind = "Method", output(proxy))]
    fn fract(self) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector containing `e^self` (the exponential function) for each element of
/// `self`.

    #[lua(kind = "Method", output(proxy))]
    fn exp(self) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector containing each element of `self` raised to the power of `n`.

    #[lua(kind = "Method", output(proxy))]
    fn powf(self, n: f64) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn recip(self) -> bevy::math::DVec4;

"#,
    r#"
/// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
/// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
/// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
/// extrapolated.

    #[lua(kind = "Method", output(proxy))]
    fn lerp(self, #[proxy] rhs: bevy::math::DVec4, s: f64) -> bevy::math::DVec4;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs` is
/// less than or equal to `max_abs_diff`.
/// This can be used to compare if two vectors contain similar elements. It works best when
/// comparing with a known value. The `max_abs_diff` that should be used used depends on
/// the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(self, #[proxy] rhs: bevy::math::DVec4, max_abs_diff: f64) -> bool;

"#,
    r#"
/// Returns a vector with a length no less than `min` and no more than `max`
/// # Panics
/// Will panic if `min` is greater than `max` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length(self, min: f64, max: f64) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector with a length no more than `max`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_max(self, max: f64) -> bevy::math::DVec4;

"#,
    r#"
/// Returns a vector with a length no less than `min`

    #[lua(kind = "Method", output(proxy))]
    fn clamp_length_min(self, min: f64) -> bevy::math::DVec4;

"#,
    r#"
/// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
/// error, yielding a more accurate result than an unfused multiply-add.
/// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
/// architecture has a dedicated fma CPU instruction. However, this is not always true,
/// and will be heavily dependant on designing algorithms with specific target hardware in
/// mind.

    #[lua(kind = "Method", output(proxy))]
    fn mul_add(
        self,
        #[proxy]
        a: bevy::math::DVec4,
        #[proxy]
        b: bevy::math::DVec4,
    ) -> bevy::math::DVec4;

"#,
    r#"
/// Casts all elements of `self` to `f32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_vec4(&self) -> bevy::math::Vec4;

"#,
    r#"
/// Casts all elements of `self` to `i32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_ivec4(&self) -> bevy::math::IVec4;

"#,
    r#"
/// Casts all elements of `self` to `u32`.

    #[lua(kind = "Method", output(proxy))]
    fn as_uvec4(&self) -> bevy::math::UVec4;

"#,
    r#"
/// Casts all elements of `self` to `i64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_i64vec4(&self) -> bevy::math::I64Vec4;

"#,
    r#"
/// Casts all elements of `self` to `u64`.

    #[lua(kind = "Method", output(proxy))]
    fn as_u64vec4(&self) -> bevy::math::U64Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::DVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f64) -> bevy::math::DVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Rem",
        kind = "MetaFunction",
        output(proxy),
        composite = "rem",
        metamethod = "Mod",
    )]
    fn rem(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::DVec4;

"#,
    r#"
#[lua(kind="MetaMethod", raw , metamethod="Index")]
fn index(&self, lua: &Lua, idx: crate::lua::util::LuaIndex) -> Result<f64,_> {
    Ok(self.inner()?[*idx])
}
"#,
    r#"
#[lua(kind="MutatingMetaMethod", raw , metamethod="NewIndex")]
fn index(&mut self, lua: &Lua, idx: crate::lua::util::LuaIndex, val: f64) -> Result<(),_> {
    self.val_mut(|s| Ok(s[*idx] = val))?
}
"#]
)]
pub struct DVec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
/// A 2x2 column major matrix.
/// SIMD vector types are used for storage on supported platforms.
/// This type is 16 byte aligned.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Mat2",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::Mat2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::Mat2) -> bevy::math::Mat2;

"#,
    r#"
/// Creates a 2x2 matrix from two column vectors.

    #[lua(kind = "Function", output(proxy))]
    fn from_cols(
        #[proxy]
        x_axis: bevy::math::Vec2,
        #[proxy]
        y_axis: bevy::math::Vec2,
    ) -> bevy::math::Mat2;

"#,
    r#"
/// Creates a `[f32; 4]` array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array(&self) -> [f32; 4];

"#,
    r#"
/// Creates a `[[f32; 2]; 2]` 2D array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array_2d(&self) -> [[f32; 2]; 2];

"#,
    r#"
/// Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.

    #[lua(kind = "Function", output(proxy))]
    fn from_diagonal(#[proxy] diagonal: bevy::math::Vec2) -> bevy::math::Mat2;

"#,
    r#"
/// Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
/// `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_scale_angle(
        #[proxy]
        scale: bevy::math::Vec2,
        angle: f32,
    ) -> bevy::math::Mat2;

"#,
    r#"
/// Creates a 2x2 matrix containing a rotation of `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_angle(angle: f32) -> bevy::math::Mat2;

"#,
    r#"
/// Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3(#[proxy] m: bevy::math::Mat3) -> bevy::math::Mat2;

"#,
    r#"
/// Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3a(#[proxy] m: bevy::math::Mat3A) -> bevy::math::Mat2;

"#,
    r#"
/// Returns the matrix column for the given `index`.
/// # Panics
/// Panics if `index` is greater than 1.

    #[lua(kind = "Method", output(proxy))]
    fn col(&self, index: usize) -> bevy::math::Vec2;

"#,
    r#"
/// Returns the matrix row for the given `index`.
/// # Panics
/// Panics if `index` is greater than 1.

    #[lua(kind = "Method", output(proxy))]
    fn row(&self, index: usize) -> bevy::math::Vec2;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(&self) -> bool;

"#,
    r#"
/// Returns the transpose of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn transpose(&self) -> bevy::math::Mat2;

"#,
    r#"
/// Returns the determinant of `self`.

    #[lua(kind = "Method")]
    fn determinant(&self) -> f32;

"#,
    r#"
/// Returns the inverse of `self`.
/// If the matrix is not invertible the returned matrix will be invalid.
/// # Panics
/// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(&self) -> bevy::math::Mat2;

"#,
    r#"
/// Transforms a 2D vector.

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec2(&self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Multiplies two 2x2 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn mul_mat2(&self, #[proxy] rhs: &glam::Mat2) -> bevy::math::Mat2;

"#,
    r#"
/// Adds two 2x2 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn add_mat2(&self, #[proxy] rhs: &glam::Mat2) -> bevy::math::Mat2;

"#,
    r#"
/// Subtracts two 2x2 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn sub_mat2(&self, #[proxy] rhs: &glam::Mat2) -> bevy::math::Mat2;

"#,
    r#"
/// Multiplies a 2x2 matrix by a scalar.

    #[lua(kind = "Method", output(proxy))]
    fn mul_scalar(&self, rhs: f32) -> bevy::math::Mat2;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two matrices contain similar elements. It works best
/// when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(&self, #[proxy] rhs: bevy::math::Mat2, max_abs_diff: f32) -> bool;

"#,
    r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_dmat2(&self) -> bevy::math::DMat2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::Mat2) -> bevy::math::Mat2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f32) -> bevy::math::Mat2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Mat2) -> bevy::math::Mat2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::Mat2) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Mat2;

"#]
)]
pub struct Mat2();
/// A 3x3 column major matrix.
/// This 3x3 matrix type features convenience methods for creating and using linear and
/// affine transformations. If you are primarily dealing with 2D affine transformations the
/// [`Affine2`](crate::Affine2) type is much faster and more space efficient than
/// using a 3x3 matrix.
/// Linear transformations including 3D rotation and scale can be created using methods
/// such as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],
/// [`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or
/// [`Self::from_rotation_z()`].
/// The resulting matrices can be use to transform 3D vectors using regular vector
/// multiplication.
/// Affine transformations including 2D translation, rotation and scale can be created
/// using methods such as [`Self::from_translation()`], [`Self::from_angle()`],
/// [`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].
/// The [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods
/// are provided for performing affine transforms on 2D vectors and points. These multiply
/// 2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for
/// vectors respectively. These methods assume that `Self` contains a valid affine
/// transform.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Mat3",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::Mat3) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Mat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Affine2) -> bevy::math::Mat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::Mat3) -> bevy::math::Mat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Mat3) -> bevy::math::Mat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f32) -> bevy::math::Mat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::Mat3) -> bevy::math::Mat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::Mat3;

"#,
    r#"
/// Creates a 3x3 matrix from three column vectors.

    #[lua(kind = "Function", output(proxy))]
    fn from_cols(
        #[proxy]
        x_axis: bevy::math::Vec3,
        #[proxy]
        y_axis: bevy::math::Vec3,
        #[proxy]
        z_axis: bevy::math::Vec3,
    ) -> bevy::math::Mat3;

"#,
    r#"
/// Creates a `[f32; 9]` array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array(&self) -> [f32; 9];

"#,
    r#"
/// Creates a `[[f32; 3]; 3]` 3D array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array_2d(&self) -> [[f32; 3]; 3];

"#,
    r#"
/// Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.

    #[lua(kind = "Function", output(proxy))]
    fn from_diagonal(#[proxy] diagonal: bevy::math::Vec3) -> bevy::math::Mat3;

"#,
    r#"
/// Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat4(#[proxy] m: bevy::math::Mat4) -> bevy::math::Mat3;

"#,
    r#"
/// Creates a 3D rotation matrix from the given quaternion.
/// # Panics
/// Will panic if `rotation` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_quat(#[proxy] rotation: bevy::math::Quat) -> bevy::math::Mat3;

"#,
    r#"
/// Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
/// radians).
/// # Panics
/// Will panic if `axis` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_axis_angle(#[proxy] axis: bevy::math::Vec3, angle: f32) -> bevy::math::Mat3;

"#,
    r#"
/// Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
/// radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_euler(
        #[proxy]
        order: bevy::math::EulerRot,
        a: f32,
        b: f32,
        c: f32,
    ) -> bevy::math::Mat3;

"#,
    r#"
/// Creates a 3D rotation matrix from `angle` (in radians) around the x axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_x(angle: f32) -> bevy::math::Mat3;

"#,
    r#"
/// Creates a 3D rotation matrix from `angle` (in radians) around the y axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_y(angle: f32) -> bevy::math::Mat3;

"#,
    r#"
/// Creates a 3D rotation matrix from `angle` (in radians) around the z axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_z(angle: f32) -> bevy::math::Mat3;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2D `translation`.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_translation(#[proxy] translation: bevy::math::Vec2) -> bevy::math::Mat3;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2D rotation `angle` (in
/// radians).
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_angle(angle: f32) -> bevy::math::Mat3;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
/// radians) and `translation`.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_scale_angle_translation(
        #[proxy]
        scale: bevy::math::Vec2,
        angle: f32,
        #[proxy]
        translation: bevy::math::Vec2,
    ) -> bevy::math::Mat3;

"#,
    r#"
/// Creates an affine transformation matrix from the given non-uniform 2D `scale`.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
/// # Panics
/// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale(#[proxy] scale: bevy::math::Vec2) -> bevy::math::Mat3;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2x2 matrix.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_mat2(#[proxy] m: bevy::math::Mat2) -> bevy::math::Mat3;

"#,
    r#"
/// Returns the matrix column for the given `index`.
/// # Panics
/// Panics if `index` is greater than 2.

    #[lua(kind = "Method", output(proxy))]
    fn col(&self, index: usize) -> bevy::math::Vec3;

"#,
    r#"
/// Returns the matrix row for the given `index`.
/// # Panics
/// Panics if `index` is greater than 2.

    #[lua(kind = "Method", output(proxy))]
    fn row(&self, index: usize) -> bevy::math::Vec3;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(&self) -> bool;

"#,
    r#"
/// Returns the transpose of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn transpose(&self) -> bevy::math::Mat3;

"#,
    r#"
/// Returns the determinant of `self`.

    #[lua(kind = "Method")]
    fn determinant(&self) -> f32;

"#,
    r#"
/// Returns the inverse of `self`.
/// If the matrix is not invertible the returned matrix will be invalid.
/// # Panics
/// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(&self) -> bevy::math::Mat3;

"#,
    r#"
/// Transforms the given 2D vector as a point.
/// This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
/// This method assumes that `self` contains a valid affine transform.
/// # Panics
/// Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point2(&self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Rotates the given 2D vector.
/// This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
/// This method assumes that `self` contains a valid affine transform.
/// # Panics
/// Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn transform_vector2(&self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Transforms a 3D vector.

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec3(&self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Transforms a [`Vec3A`].

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec3a(&self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Multiplies two 3x3 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn mul_mat3(&self, #[proxy] rhs: &glam::Mat3) -> bevy::math::Mat3;

"#,
    r#"
/// Adds two 3x3 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn add_mat3(&self, #[proxy] rhs: &glam::Mat3) -> bevy::math::Mat3;

"#,
    r#"
/// Subtracts two 3x3 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn sub_mat3(&self, #[proxy] rhs: &glam::Mat3) -> bevy::math::Mat3;

"#,
    r#"
/// Multiplies a 3x3 matrix by a scalar.

    #[lua(kind = "Method", output(proxy))]
    fn mul_scalar(&self, rhs: f32) -> bevy::math::Mat3;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two matrices contain similar elements. It works best
/// when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(&self, #[proxy] rhs: bevy::math::Mat3, max_abs_diff: f32) -> bool;

"#,
    r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_dmat3(&self) -> bevy::math::DMat3;

"#]
)]
pub struct Mat3 {
    #[lua(output(proxy))]
    x_axis: bevy::math::Vec3,
    #[lua(output(proxy))]
    y_axis: bevy::math::Vec3,
    #[lua(output(proxy))]
    z_axis: bevy::math::Vec3,
}
/// A 3x3 column major matrix.
/// This 3x3 matrix type features convenience methods for creating and using linear and
/// affine transformations. If you are primarily dealing with 2D affine transformations the
/// [`Affine2`](crate::Affine2) type is much faster and more space efficient than
/// using a 3x3 matrix.
/// Linear transformations including 3D rotation and scale can be created using methods
/// such as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],
/// [`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or
/// [`Self::from_rotation_z()`].
/// The resulting matrices can be use to transform 3D vectors using regular vector
/// multiplication.
/// Affine transformations including 2D translation, rotation and scale can be created
/// using methods such as [`Self::from_translation()`], [`Self::from_angle()`],
/// [`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].
/// The [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods
/// are provided for performing affine transforms on 2D vectors and points. These multiply
/// 2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for
/// vectors respectively. These methods assume that `Self` contains a valid affine
/// transform.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Mat3A",
    functions[r#"
/// Creates a 3x3 matrix from three column vectors.

    #[lua(kind = "Function", output(proxy))]
    fn from_cols(
        #[proxy]
        x_axis: bevy::math::Vec3A,
        #[proxy]
        y_axis: bevy::math::Vec3A,
        #[proxy]
        z_axis: bevy::math::Vec3A,
    ) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates a `[f32; 9]` array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array(&self) -> [f32; 9];

"#,
    r#"
/// Creates a `[[f32; 3]; 3]` 3D array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array_2d(&self) -> [[f32; 3]; 3];

"#,
    r#"
/// Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.

    #[lua(kind = "Function", output(proxy))]
    fn from_diagonal(#[proxy] diagonal: bevy::math::Vec3) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat4(#[proxy] m: bevy::math::Mat4) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates a 3D rotation matrix from the given quaternion.
/// # Panics
/// Will panic if `rotation` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_quat(#[proxy] rotation: bevy::math::Quat) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
/// radians).
/// # Panics
/// Will panic if `axis` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_axis_angle(#[proxy] axis: bevy::math::Vec3, angle: f32) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
/// radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_euler(
        #[proxy]
        order: bevy::math::EulerRot,
        a: f32,
        b: f32,
        c: f32,
    ) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates a 3D rotation matrix from `angle` (in radians) around the x axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_x(angle: f32) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates a 3D rotation matrix from `angle` (in radians) around the y axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_y(angle: f32) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates a 3D rotation matrix from `angle` (in radians) around the z axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_z(angle: f32) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2D `translation`.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_translation(#[proxy] translation: bevy::math::Vec2) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2D rotation `angle` (in
/// radians).
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_angle(angle: f32) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
/// radians) and `translation`.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_scale_angle_translation(
        #[proxy]
        scale: bevy::math::Vec2,
        angle: f32,
        #[proxy]
        translation: bevy::math::Vec2,
    ) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates an affine transformation matrix from the given non-uniform 2D `scale`.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
/// # Panics
/// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale(#[proxy] scale: bevy::math::Vec2) -> bevy::math::Mat3A;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2x2 matrix.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_mat2(#[proxy] m: bevy::math::Mat2) -> bevy::math::Mat3A;

"#,
    r#"
/// Returns the matrix column for the given `index`.
/// # Panics
/// Panics if `index` is greater than 2.

    #[lua(kind = "Method", output(proxy))]
    fn col(&self, index: usize) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns the matrix row for the given `index`.
/// # Panics
/// Panics if `index` is greater than 2.

    #[lua(kind = "Method", output(proxy))]
    fn row(&self, index: usize) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(&self) -> bool;

"#,
    r#"
/// Returns the transpose of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn transpose(&self) -> bevy::math::Mat3A;

"#,
    r#"
/// Returns the determinant of `self`.

    #[lua(kind = "Method")]
    fn determinant(&self) -> f32;

"#,
    r#"
/// Returns the inverse of `self`.
/// If the matrix is not invertible the returned matrix will be invalid.
/// # Panics
/// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(&self) -> bevy::math::Mat3A;

"#,
    r#"
/// Transforms the given 2D vector as a point.
/// This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
/// This method assumes that `self` contains a valid affine transform.
/// # Panics
/// Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point2(&self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Rotates the given 2D vector.
/// This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
/// This method assumes that `self` contains a valid affine transform.
/// # Panics
/// Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn transform_vector2(&self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Transforms a 3D vector.

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec3(&self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Transforms a [`Vec3A`].

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec3a(&self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Multiplies two 3x3 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn mul_mat3(&self, #[proxy] rhs: &glam::Mat3A) -> bevy::math::Mat3A;

"#,
    r#"
/// Adds two 3x3 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn add_mat3(&self, #[proxy] rhs: &glam::Mat3A) -> bevy::math::Mat3A;

"#,
    r#"
/// Subtracts two 3x3 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn sub_mat3(&self, #[proxy] rhs: &glam::Mat3A) -> bevy::math::Mat3A;

"#,
    r#"
/// Multiplies a 3x3 matrix by a scalar.

    #[lua(kind = "Method", output(proxy))]
    fn mul_scalar(&self, rhs: f32) -> bevy::math::Mat3A;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two matrices contain similar elements. It works best
/// when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(&self, #[proxy] rhs: bevy::math::Mat3A, max_abs_diff: f32) -> bool;

"#,
    r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_dmat3(&self) -> bevy::math::DMat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f32) -> bevy::math::Mat3A;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::Mat3A) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::Mat3A) -> bevy::math::Mat3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::Mat3A) -> bevy::math::Mat3A;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Mat3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::Mat3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Mat3A) -> bevy::math::Mat3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Affine2) -> bevy::math::Mat3A;

"#]
)]
pub struct Mat3A {
    #[lua(output(proxy))]
    x_axis: bevy::math::Vec3A,
    #[lua(output(proxy))]
    y_axis: bevy::math::Vec3A,
    #[lua(output(proxy))]
    z_axis: bevy::math::Vec3A,
}
/// A 4x4 column major matrix.
/// This 4x4 matrix type features convenience methods for creating and using affine transforms and
/// perspective projections. If you are primarily dealing with 3D affine transformations
/// considering using [`Affine3A`](crate::Affine3A) which is faster than a 4x4 matrix
/// for some affine operations.
/// Affine transformations including 3D translation, rotation and scale can be created
/// using methods such as [`Self::from_translation()`], [`Self::from_quat()`],
/// [`Self::from_scale()`] and [`Self::from_scale_rotation_translation()`].
/// Orthographic projections can be created using the methods [`Self::orthographic_lh()`] for
/// left-handed coordinate systems and [`Self::orthographic_rh()`] for right-handed
/// systems. The resulting matrix is also an affine transformation.
/// The [`Self::transform_point3()`] and [`Self::transform_vector3()`] convenience methods
/// are provided for performing affine transformations on 3D vectors and points. These
/// multiply 3D inputs as 4D vectors with an implicit `w` value of `1` for points and `0`
/// for vectors respectively. These methods assume that `Self` contains a valid affine
/// transform.
/// Perspective projections can be created using methods such as
/// [`Self::perspective_lh()`], [`Self::perspective_infinite_lh()`] and
/// [`Self::perspective_infinite_reverse_lh()`] for left-handed co-ordinate systems and
/// [`Self::perspective_rh()`], [`Self::perspective_infinite_rh()`] and
/// [`Self::perspective_infinite_reverse_rh()`] for right-handed co-ordinate systems.
/// The resulting perspective project can be use to transform 3D vectors as points with
/// perspective correction using the [`Self::project_point3()`] convenience method.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Mat4",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::Mat4) -> bevy::math::Mat4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f32) -> bevy::math::Mat4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::Mat4;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Mat4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::Mat4) -> bevy::math::Mat4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::Mat4) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Mat4) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a 4x4 matrix from four column vectors.

    #[lua(kind = "Function", output(proxy))]
    fn from_cols(
        #[proxy]
        x_axis: bevy::math::Vec4,
        #[proxy]
        y_axis: bevy::math::Vec4,
        #[proxy]
        z_axis: bevy::math::Vec4,
        #[proxy]
        w_axis: bevy::math::Vec4,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a `[f32; 16]` array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array(&self) -> [f32; 16];

"#,
    r#"
/// Creates a `[[f32; 4]; 4]` 4D array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array_2d(&self) -> [[f32; 4]; 4];

"#,
    r#"
/// Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.

    #[lua(kind = "Function", output(proxy))]
    fn from_diagonal(#[proxy] diagonal: bevy::math::Vec4) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
/// `translation`.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
/// # Panics
/// Will panic if `rotation` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale_rotation_translation(
        #[proxy]
        scale: bevy::math::Vec3,
        #[proxy]
        rotation: bevy::math::Quat,
        #[proxy]
        translation: bevy::math::Vec3,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an affine transformation matrix from the given 3D `translation`.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
/// # Panics
/// Will panic if `rotation` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_translation(
        #[proxy]
        rotation: bevy::math::Quat,
        #[proxy]
        translation: bevy::math::Vec3,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an affine transformation matrix from the given `rotation` quaternion.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
/// # Panics
/// Will panic if `rotation` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_quat(#[proxy] rotation: bevy::math::Quat) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an affine transformation matrix from the given 3x3 linear transformation
/// matrix.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3(#[proxy] m: bevy::math::Mat3) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an affine transformation matrix from the given 3x3 linear transformation
/// matrix.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3a(#[proxy] m: bevy::math::Mat3A) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an affine transformation matrix from the given 3D `translation`.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_translation(#[proxy] translation: bevy::math::Vec3) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an affine transformation matrix containing a 3D rotation around a normalized
/// rotation `axis` of `angle` (in radians).
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
/// # Panics
/// Will panic if `axis` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_axis_angle(#[proxy] axis: bevy::math::Vec3, angle: f32) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a affine transformation matrix containing a rotation from the given euler
/// rotation sequence and angles (in radians).
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_euler(
        #[proxy]
        order: bevy::math::EulerRot,
        a: f32,
        b: f32,
        c: f32,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an affine transformation matrix containing a 3D rotation around the x axis of
/// `angle` (in radians).
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_x(angle: f32) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an affine transformation matrix containing a 3D rotation around the y axis of
/// `angle` (in radians).
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_y(angle: f32) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an affine transformation matrix containing a 3D rotation around the z axis of
/// `angle` (in radians).
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_z(angle: f32) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
/// # Panics
/// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale(#[proxy] scale: bevy::math::Vec3) -> bevy::math::Mat4;

"#,
    r#"
/// Returns the matrix column for the given `index`.
/// # Panics
/// Panics if `index` is greater than 3.

    #[lua(kind = "Method", output(proxy))]
    fn col(&self, index: usize) -> bevy::math::Vec4;

"#,
    r#"
/// Returns the matrix row for the given `index`.
/// # Panics
/// Panics if `index` is greater than 3.

    #[lua(kind = "Method", output(proxy))]
    fn row(&self, index: usize) -> bevy::math::Vec4;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(&self) -> bool;

"#,
    r#"
/// Returns the transpose of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn transpose(&self) -> bevy::math::Mat4;

"#,
    r#"
/// Returns the determinant of `self`.

    #[lua(kind = "Method")]
    fn determinant(&self) -> f32;

"#,
    r#"
/// Returns the inverse of `self`.
/// If the matrix is not invertible the returned matrix will be invalid.
/// # Panics
/// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(&self) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a left-handed view matrix using a camera position, an up direction, and a facing
/// direction.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

    #[lua(kind = "Function", output(proxy))]
    fn look_to_lh(
        #[proxy]
        eye: bevy::math::Vec3,
        #[proxy]
        dir: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a right-handed view matrix using a camera position, an up direction, and a facing
/// direction.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

    #[lua(kind = "Function", output(proxy))]
    fn look_to_rh(
        #[proxy]
        eye: bevy::math::Vec3,
        #[proxy]
        dir: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a left-handed view matrix using a camera position, an up direction, and a focal
/// point.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
/// # Panics
/// Will panic if `up` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn look_at_lh(
        #[proxy]
        eye: bevy::math::Vec3,
        #[proxy]
        center: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a right-handed view matrix using a camera position, an up direction, and a focal
/// point.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
/// # Panics
/// Will panic if `up` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn look_at_rh(
        #[proxy]
        eye: bevy::math::Vec3,
        #[proxy]
        center: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a right-handed perspective projection matrix with [-1,1] depth range.
/// This is the same as the OpenGL `gluPerspective` function.
/// See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>

    #[lua(kind = "Function", output(proxy))]
    fn perspective_rh_gl(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a left-handed perspective projection matrix with `[0,1]` depth range.
/// # Panics
/// Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
/// enabled.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_lh(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a right-handed perspective projection matrix with `[0,1]` depth range.
/// # Panics
/// Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
/// enabled.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_rh(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
/// # Panics
/// Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_infinite_lh(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
/// # Panics
/// Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_infinite_reverse_lh(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an infinite right-handed perspective projection matrix with
/// `[0,1]` depth range.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_infinite_rh(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates an infinite reverse right-handed perspective projection matrix
/// with `[0,1]` depth range.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_infinite_reverse_rh(
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a right-handed orthographic projection matrix with `[-1,1]` depth
/// range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
/// See
/// <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>

    #[lua(kind = "Function", output(proxy))]
    fn orthographic_rh_gl(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a left-handed orthographic projection matrix with `[0,1]` depth range.

    #[lua(kind = "Function", output(proxy))]
    fn orthographic_lh(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Creates a right-handed orthographic projection matrix with `[0,1]` depth range.

    #[lua(kind = "Function", output(proxy))]
    fn orthographic_rh(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> bevy::math::Mat4;

"#,
    r#"
/// Transforms the given 3D vector as a point, applying perspective correction.
/// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
/// The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
/// This method assumes that `self` contains a projective transform.

    #[lua(kind = "Method", output(proxy))]
    fn project_point3(&self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Transforms the given 3D vector as a point.
/// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
/// `1.0`.
/// This method assumes that `self` contains a valid affine transform. It does not perform
/// a persective divide, if `self` contains a perspective transform, or if you are unsure,
/// the [`Self::project_point3()`] method should be used instead.
/// # Panics
/// Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point3(&self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Transforms the give 3D vector as a direction.
/// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
/// `0.0`.
/// This method assumes that `self` contains a valid affine transform.
/// # Panics
/// Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn transform_vector3(&self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Transforms the given [`Vec3A`] as 3D point.
/// This is the equivalent of multiplying the [`Vec3A`] as a 4D vector where `w` is `1.0`.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point3a(&self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Transforms the give [`Vec3A`] as 3D vector.
/// This is the equivalent of multiplying the [`Vec3A`] as a 4D vector where `w` is `0.0`.

    #[lua(kind = "Method", output(proxy))]
    fn transform_vector3a(&self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Transforms a 4D vector.

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec4(&self, #[proxy] rhs: bevy::math::Vec4) -> bevy::math::Vec4;

"#,
    r#"
/// Multiplies two 4x4 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn mul_mat4(&self, #[proxy] rhs: &glam::Mat4) -> bevy::math::Mat4;

"#,
    r#"
/// Adds two 4x4 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn add_mat4(&self, #[proxy] rhs: &glam::Mat4) -> bevy::math::Mat4;

"#,
    r#"
/// Subtracts two 4x4 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn sub_mat4(&self, #[proxy] rhs: &glam::Mat4) -> bevy::math::Mat4;

"#,
    r#"
/// Multiplies a 4x4 matrix by a scalar.

    #[lua(kind = "Method", output(proxy))]
    fn mul_scalar(&self, rhs: f32) -> bevy::math::Mat4;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two matrices contain similar elements. It works best
/// when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(&self, #[proxy] rhs: bevy::math::Mat4, max_abs_diff: f32) -> bool;

"#,
    r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_dmat4(&self) -> bevy::math::DMat4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Affine3A) -> bevy::math::Mat4;

"#]
)]
pub struct Mat4 {
    #[lua(output(proxy))]
    x_axis: bevy::math::Vec4,
    #[lua(output(proxy))]
    y_axis: bevy::math::Vec4,
    #[lua(output(proxy))]
    z_axis: bevy::math::Vec4,
    #[lua(output(proxy))]
    w_axis: bevy::math::Vec4,
}
/// A 2x2 column major matrix.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DMat2",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::DMat2) -> bevy::math::DMat2;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::DMat2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::DMat2;

"#,
    r#"
/// Creates a 2x2 matrix from two column vectors.

    #[lua(kind = "Function", output(proxy))]
    fn from_cols(
        #[proxy]
        x_axis: bevy::math::DVec2,
        #[proxy]
        y_axis: bevy::math::DVec2,
    ) -> bevy::math::DMat2;

"#,
    r#"
/// Creates a `[f64; 4]` array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array(&self) -> [f64; 4];

"#,
    r#"
/// Creates a `[[f64; 2]; 2]` 2D array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array_2d(&self) -> [[f64; 2]; 2];

"#,
    r#"
/// Creates a 2x2 matrix with its diagonal set to `diagonal` and all other entries set to 0.

    #[lua(kind = "Function", output(proxy))]
    fn from_diagonal(#[proxy] diagonal: bevy::math::DVec2) -> bevy::math::DMat2;

"#,
    r#"
/// Creates a 2x2 matrix containing the combining non-uniform `scale` and rotation of
/// `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_scale_angle(
        #[proxy]
        scale: bevy::math::DVec2,
        angle: f64,
    ) -> bevy::math::DMat2;

"#,
    r#"
/// Creates a 2x2 matrix containing a rotation of `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_angle(angle: f64) -> bevy::math::DMat2;

"#,
    r#"
/// Creates a 2x2 matrix from a 3x3 matrix, discarding the 2nd row and column.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3(#[proxy] m: bevy::math::DMat3) -> bevy::math::DMat2;

"#,
    r#"
/// Returns the matrix column for the given `index`.
/// # Panics
/// Panics if `index` is greater than 1.

    #[lua(kind = "Method", output(proxy))]
    fn col(&self, index: usize) -> bevy::math::DVec2;

"#,
    r#"
/// Returns the matrix row for the given `index`.
/// # Panics
/// Panics if `index` is greater than 1.

    #[lua(kind = "Method", output(proxy))]
    fn row(&self, index: usize) -> bevy::math::DVec2;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(&self) -> bool;

"#,
    r#"
/// Returns the transpose of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn transpose(&self) -> bevy::math::DMat2;

"#,
    r#"
/// Returns the determinant of `self`.

    #[lua(kind = "Method")]
    fn determinant(&self) -> f64;

"#,
    r#"
/// Returns the inverse of `self`.
/// If the matrix is not invertible the returned matrix will be invalid.
/// # Panics
/// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(&self) -> bevy::math::DMat2;

"#,
    r#"
/// Transforms a 2D vector.

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec2(&self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Multiplies two 2x2 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn mul_mat2(&self, #[proxy] rhs: &glam::DMat2) -> bevy::math::DMat2;

"#,
    r#"
/// Adds two 2x2 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn add_mat2(&self, #[proxy] rhs: &glam::DMat2) -> bevy::math::DMat2;

"#,
    r#"
/// Subtracts two 2x2 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn sub_mat2(&self, #[proxy] rhs: &glam::DMat2) -> bevy::math::DMat2;

"#,
    r#"
/// Multiplies a 2x2 matrix by a scalar.

    #[lua(kind = "Method", output(proxy))]
    fn mul_scalar(&self, rhs: f64) -> bevy::math::DMat2;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two matrices contain similar elements. It works best
/// when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(&self, #[proxy] rhs: bevy::math::DMat2, max_abs_diff: f64) -> bool;

"#,
    r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_mat2(&self) -> bevy::math::Mat2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DMat2) -> bevy::math::DMat2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::DMat2) -> bevy::math::DMat2;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::DMat2) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f64) -> bevy::math::DMat2;

"#]
)]
pub struct DMat2 {
    #[lua(output(proxy))]
    x_axis: bevy::math::DVec2,
    #[lua(output(proxy))]
    y_axis: bevy::math::DVec2,
}
/// A 3x3 column major matrix.
/// This 3x3 matrix type features convenience methods for creating and using linear and
/// affine transformations. If you are primarily dealing with 2D affine transformations the
/// [`DAffine2`](crate::DAffine2) type is much faster and more space efficient than
/// using a 3x3 matrix.
/// Linear transformations including 3D rotation and scale can be created using methods
/// such as [`Self::from_diagonal()`], [`Self::from_quat()`], [`Self::from_axis_angle()`],
/// [`Self::from_rotation_x()`], [`Self::from_rotation_y()`], or
/// [`Self::from_rotation_z()`].
/// The resulting matrices can be use to transform 3D vectors using regular vector
/// multiplication.
/// Affine transformations including 2D translation, rotation and scale can be created
/// using methods such as [`Self::from_translation()`], [`Self::from_angle()`],
/// [`Self::from_scale()`] and [`Self::from_scale_angle_translation()`].
/// The [`Self::transform_point2()`] and [`Self::transform_vector2()`] convenience methods
/// are provided for performing affine transforms on 2D vectors and points. These multiply
/// 2D inputs as 3D vectors with an implicit `z` value of `1` for points and `0` for
/// vectors respectively. These methods assume that `Self` contains a valid affine
/// transform.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DMat3",
    functions[r#"
/// Creates a 3x3 matrix from three column vectors.

    #[lua(kind = "Function", output(proxy))]
    fn from_cols(
        #[proxy]
        x_axis: bevy::math::DVec3,
        #[proxy]
        y_axis: bevy::math::DVec3,
        #[proxy]
        z_axis: bevy::math::DVec3,
    ) -> bevy::math::DMat3;

"#,
    r#"
/// Creates a `[f64; 9]` array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array(&self) -> [f64; 9];

"#,
    r#"
/// Creates a `[[f64; 3]; 3]` 3D array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array_2d(&self) -> [[f64; 3]; 3];

"#,
    r#"
/// Creates a 3x3 matrix with its diagonal set to `diagonal` and all other entries set to 0.

    #[lua(kind = "Function", output(proxy))]
    fn from_diagonal(#[proxy] diagonal: bevy::math::DVec3) -> bevy::math::DMat3;

"#,
    r#"
/// Creates a 3x3 matrix from a 4x4 matrix, discarding the 4th row and column.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat4(#[proxy] m: bevy::math::DMat4) -> bevy::math::DMat3;

"#,
    r#"
/// Creates a 3D rotation matrix from the given quaternion.
/// # Panics
/// Will panic if `rotation` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_quat(#[proxy] rotation: bevy::math::DQuat) -> bevy::math::DMat3;

"#,
    r#"
/// Creates a 3D rotation matrix from a normalized rotation `axis` and `angle` (in
/// radians).
/// # Panics
/// Will panic if `axis` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_axis_angle(
        #[proxy]
        axis: bevy::math::DVec3,
        angle: f64,
    ) -> bevy::math::DMat3;

"#,
    r#"
/// Creates a 3D rotation matrix from the given euler rotation sequence and the angles (in
/// radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_euler(
        #[proxy]
        order: bevy::math::EulerRot,
        a: f64,
        b: f64,
        c: f64,
    ) -> bevy::math::DMat3;

"#,
    r#"
/// Creates a 3D rotation matrix from `angle` (in radians) around the x axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_x(angle: f64) -> bevy::math::DMat3;

"#,
    r#"
/// Creates a 3D rotation matrix from `angle` (in radians) around the y axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_y(angle: f64) -> bevy::math::DMat3;

"#,
    r#"
/// Creates a 3D rotation matrix from `angle` (in radians) around the z axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_z(angle: f64) -> bevy::math::DMat3;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2D `translation`.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_translation(#[proxy] translation: bevy::math::DVec2) -> bevy::math::DMat3;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2D rotation `angle` (in
/// radians).
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_angle(angle: f64) -> bevy::math::DMat3;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2D `scale`, rotation `angle` (in
/// radians) and `translation`.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_scale_angle_translation(
        #[proxy]
        scale: bevy::math::DVec2,
        angle: f64,
        #[proxy]
        translation: bevy::math::DVec2,
    ) -> bevy::math::DMat3;

"#,
    r#"
/// Creates an affine transformation matrix from the given non-uniform 2D `scale`.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].
/// # Panics
/// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale(#[proxy] scale: bevy::math::DVec2) -> bevy::math::DMat3;

"#,
    r#"
/// Creates an affine transformation matrix from the given 2x2 matrix.
/// The resulting matrix can be used to transform 2D points and vectors. See
/// [`Self::transform_point2()`] and [`Self::transform_vector2()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_mat2(#[proxy] m: bevy::math::DMat2) -> bevy::math::DMat3;

"#,
    r#"
/// Returns the matrix column for the given `index`.
/// # Panics
/// Panics if `index` is greater than 2.

    #[lua(kind = "Method", output(proxy))]
    fn col(&self, index: usize) -> bevy::math::DVec3;

"#,
    r#"
/// Returns the matrix row for the given `index`.
/// # Panics
/// Panics if `index` is greater than 2.

    #[lua(kind = "Method", output(proxy))]
    fn row(&self, index: usize) -> bevy::math::DVec3;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(&self) -> bool;

"#,
    r#"
/// Returns the transpose of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn transpose(&self) -> bevy::math::DMat3;

"#,
    r#"
/// Returns the determinant of `self`.

    #[lua(kind = "Method")]
    fn determinant(&self) -> f64;

"#,
    r#"
/// Returns the inverse of `self`.
/// If the matrix is not invertible the returned matrix will be invalid.
/// # Panics
/// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(&self) -> bevy::math::DMat3;

"#,
    r#"
/// Transforms the given 2D vector as a point.
/// This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `1`.
/// This method assumes that `self` contains a valid affine transform.
/// # Panics
/// Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point2(&self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Rotates the given 2D vector.
/// This is the equivalent of multiplying `rhs` as a 3D vector where `z` is `0`.
/// This method assumes that `self` contains a valid affine transform.
/// # Panics
/// Will panic if the 2nd row of `self` is not `(0, 0, 1)` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn transform_vector2(&self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Transforms a 3D vector.

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec3(&self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Multiplies two 3x3 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn mul_mat3(&self, #[proxy] rhs: &glam::DMat3) -> bevy::math::DMat3;

"#,
    r#"
/// Adds two 3x3 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn add_mat3(&self, #[proxy] rhs: &glam::DMat3) -> bevy::math::DMat3;

"#,
    r#"
/// Subtracts two 3x3 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn sub_mat3(&self, #[proxy] rhs: &glam::DMat3) -> bevy::math::DMat3;

"#,
    r#"
/// Multiplies a 3x3 matrix by a scalar.

    #[lua(kind = "Method", output(proxy))]
    fn mul_scalar(&self, rhs: f64) -> bevy::math::DMat3;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two matrices contain similar elements. It works best
/// when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(&self, #[proxy] rhs: bevy::math::DMat3, max_abs_diff: f64) -> bool;

"#,
    r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_mat3(&self) -> bevy::math::Mat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::DMat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f64) -> bevy::math::DMat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::DMat3) -> bevy::math::DMat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::DMat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::DMat3) -> bevy::math::DMat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DAffine2) -> bevy::math::DMat3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DMat3) -> bevy::math::DMat3;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::DMat3) -> bool;

"#]
)]
pub struct DMat3 {
    #[lua(output(proxy))]
    x_axis: bevy::math::DVec3,
    #[lua(output(proxy))]
    y_axis: bevy::math::DVec3,
    #[lua(output(proxy))]
    z_axis: bevy::math::DVec3,
}
/// A 4x4 column major matrix.
/// This 4x4 matrix type features convenience methods for creating and using affine transforms and
/// perspective projections. If you are primarily dealing with 3D affine transformations
/// considering using [`DAffine3`](crate::DAffine3) which is faster than a 4x4 matrix
/// for some affine operations.
/// Affine transformations including 3D translation, rotation and scale can be created
/// using methods such as [`Self::from_translation()`], [`Self::from_quat()`],
/// [`Self::from_scale()`] and [`Self::from_scale_rotation_translation()`].
/// Orthographic projections can be created using the methods [`Self::orthographic_lh()`] for
/// left-handed coordinate systems and [`Self::orthographic_rh()`] for right-handed
/// systems. The resulting matrix is also an affine transformation.
/// The [`Self::transform_point3()`] and [`Self::transform_vector3()`] convenience methods
/// are provided for performing affine transformations on 3D vectors and points. These
/// multiply 3D inputs as 4D vectors with an implicit `w` value of `1` for points and `0`
/// for vectors respectively. These methods assume that `Self` contains a valid affine
/// transform.
/// Perspective projections can be created using methods such as
/// [`Self::perspective_lh()`], [`Self::perspective_infinite_lh()`] and
/// [`Self::perspective_infinite_reverse_lh()`] for left-handed co-ordinate systems and
/// [`Self::perspective_rh()`], [`Self::perspective_infinite_rh()`] and
/// [`Self::perspective_infinite_reverse_rh()`] for right-handed co-ordinate systems.
/// The resulting perspective project can be use to transform 3D vectors as points with
/// perspective correction using the [`Self::project_point3()`] convenience method.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DMat4",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DAffine3) -> bevy::math::DMat4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::DMat4) -> bevy::math::DMat4;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::DMat4) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::DMat4) -> bevy::math::DMat4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f64) -> bevy::math::DMat4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DMat4) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a 4x4 matrix from four column vectors.

    #[lua(kind = "Function", output(proxy))]
    fn from_cols(
        #[proxy]
        x_axis: bevy::math::DVec4,
        #[proxy]
        y_axis: bevy::math::DVec4,
        #[proxy]
        z_axis: bevy::math::DVec4,
        #[proxy]
        w_axis: bevy::math::DVec4,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a `[f64; 16]` array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array(&self) -> [f64; 16];

"#,
    r#"
/// Creates a `[[f64; 4]; 4]` 4D array storing data in column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array_2d(&self) -> [[f64; 4]; 4];

"#,
    r#"
/// Creates a 4x4 matrix with its diagonal set to `diagonal` and all other entries set to 0.

    #[lua(kind = "Function", output(proxy))]
    fn from_diagonal(#[proxy] diagonal: bevy::math::DVec4) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an affine transformation matrix from the given 3D `scale`, `rotation` and
/// `translation`.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
/// # Panics
/// Will panic if `rotation` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale_rotation_translation(
        #[proxy]
        scale: bevy::math::DVec3,
        #[proxy]
        rotation: bevy::math::DQuat,
        #[proxy]
        translation: bevy::math::DVec3,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an affine transformation matrix from the given 3D `translation`.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
/// # Panics
/// Will panic if `rotation` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_translation(
        #[proxy]
        rotation: bevy::math::DQuat,
        #[proxy]
        translation: bevy::math::DVec3,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an affine transformation matrix from the given `rotation` quaternion.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
/// # Panics
/// Will panic if `rotation` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_quat(#[proxy] rotation: bevy::math::DQuat) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an affine transformation matrix from the given 3x3 linear transformation
/// matrix.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3(#[proxy] m: bevy::math::DMat3) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an affine transformation matrix from the given 3D `translation`.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_translation(#[proxy] translation: bevy::math::DVec3) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an affine transformation matrix containing a 3D rotation around a normalized
/// rotation `axis` of `angle` (in radians).
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
/// # Panics
/// Will panic if `axis` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_axis_angle(
        #[proxy]
        axis: bevy::math::DVec3,
        angle: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a affine transformation matrix containing a rotation from the given euler
/// rotation sequence and angles (in radians).
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_euler(
        #[proxy]
        order: bevy::math::EulerRot,
        a: f64,
        b: f64,
        c: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an affine transformation matrix containing a 3D rotation around the x axis of
/// `angle` (in radians).
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_x(angle: f64) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an affine transformation matrix containing a 3D rotation around the y axis of
/// `angle` (in radians).
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_y(angle: f64) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an affine transformation matrix containing a 3D rotation around the z axis of
/// `angle` (in radians).
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_z(angle: f64) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an affine transformation matrix containing the given 3D non-uniform `scale`.
/// The resulting matrix can be used to transform 3D points and vectors. See
/// [`Self::transform_point3()`] and [`Self::transform_vector3()`].
/// # Panics
/// Will panic if all elements of `scale` are zero when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale(#[proxy] scale: bevy::math::DVec3) -> bevy::math::DMat4;

"#,
    r#"
/// Returns the matrix column for the given `index`.
/// # Panics
/// Panics if `index` is greater than 3.

    #[lua(kind = "Method", output(proxy))]
    fn col(&self, index: usize) -> bevy::math::DVec4;

"#,
    r#"
/// Returns the matrix row for the given `index`.
/// # Panics
/// Panics if `index` is greater than 3.

    #[lua(kind = "Method", output(proxy))]
    fn row(&self, index: usize) -> bevy::math::DVec4;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(&self) -> bool;

"#,
    r#"
/// Returns the transpose of `self`.

    #[lua(kind = "Method", output(proxy))]
    fn transpose(&self) -> bevy::math::DMat4;

"#,
    r#"
/// Returns the determinant of `self`.

    #[lua(kind = "Method")]
    fn determinant(&self) -> f64;

"#,
    r#"
/// Returns the inverse of `self`.
/// If the matrix is not invertible the returned matrix will be invalid.
/// # Panics
/// Will panic if the determinant of `self` is zero when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(&self) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a left-handed view matrix using a camera position, an up direction, and a facing
/// direction.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

    #[lua(kind = "Function", output(proxy))]
    fn look_to_lh(
        #[proxy]
        eye: bevy::math::DVec3,
        #[proxy]
        dir: bevy::math::DVec3,
        #[proxy]
        up: bevy::math::DVec3,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a right-handed view matrix using a camera position, an up direction, and a facing
/// direction.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

    #[lua(kind = "Function", output(proxy))]
    fn look_to_rh(
        #[proxy]
        eye: bevy::math::DVec3,
        #[proxy]
        dir: bevy::math::DVec3,
        #[proxy]
        up: bevy::math::DVec3,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a left-handed view matrix using a camera position, an up direction, and a focal
/// point.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
/// # Panics
/// Will panic if `up` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn look_at_lh(
        #[proxy]
        eye: bevy::math::DVec3,
        #[proxy]
        center: bevy::math::DVec3,
        #[proxy]
        up: bevy::math::DVec3,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a right-handed view matrix using a camera position, an up direction, and a focal
/// point.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
/// # Panics
/// Will panic if `up` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn look_at_rh(
        #[proxy]
        eye: bevy::math::DVec3,
        #[proxy]
        center: bevy::math::DVec3,
        #[proxy]
        up: bevy::math::DVec3,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a right-handed perspective projection matrix with [-1,1] depth range.
/// This is the same as the OpenGL `gluPerspective` function.
/// See <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml>

    #[lua(kind = "Function", output(proxy))]
    fn perspective_rh_gl(
        fov_y_radians: f64,
        aspect_ratio: f64,
        z_near: f64,
        z_far: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a left-handed perspective projection matrix with `[0,1]` depth range.
/// # Panics
/// Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
/// enabled.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_lh(
        fov_y_radians: f64,
        aspect_ratio: f64,
        z_near: f64,
        z_far: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a right-handed perspective projection matrix with `[0,1]` depth range.
/// # Panics
/// Will panic if `z_near` or `z_far` are less than or equal to zero when `glam_assert` is
/// enabled.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_rh(
        fov_y_radians: f64,
        aspect_ratio: f64,
        z_near: f64,
        z_far: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
/// # Panics
/// Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_infinite_lh(
        fov_y_radians: f64,
        aspect_ratio: f64,
        z_near: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an infinite left-handed perspective projection matrix with `[0,1]` depth range.
/// # Panics
/// Will panic if `z_near` is less than or equal to zero when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_infinite_reverse_lh(
        fov_y_radians: f64,
        aspect_ratio: f64,
        z_near: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an infinite right-handed perspective projection matrix with
/// `[0,1]` depth range.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_infinite_rh(
        fov_y_radians: f64,
        aspect_ratio: f64,
        z_near: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an infinite reverse right-handed perspective projection matrix
/// with `[0,1]` depth range.

    #[lua(kind = "Function", output(proxy))]
    fn perspective_infinite_reverse_rh(
        fov_y_radians: f64,
        aspect_ratio: f64,
        z_near: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a right-handed orthographic projection matrix with `[-1,1]` depth
/// range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
/// See
/// <https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml>

    #[lua(kind = "Function", output(proxy))]
    fn orthographic_rh_gl(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a left-handed orthographic projection matrix with `[0,1]` depth range.

    #[lua(kind = "Function", output(proxy))]
    fn orthographic_lh(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Creates a right-handed orthographic projection matrix with `[0,1]` depth range.

    #[lua(kind = "Function", output(proxy))]
    fn orthographic_rh(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) -> bevy::math::DMat4;

"#,
    r#"
/// Transforms the given 3D vector as a point, applying perspective correction.
/// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is `1.0`.
/// The perspective divide is performed meaning the resulting 3D vector is divided by `w`.
/// This method assumes that `self` contains a projective transform.

    #[lua(kind = "Method", output(proxy))]
    fn project_point3(&self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Transforms the given 3D vector as a point.
/// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
/// `1.0`.
/// This method assumes that `self` contains a valid affine transform. It does not perform
/// a persective divide, if `self` contains a perspective transform, or if you are unsure,
/// the [`Self::project_point3()`] method should be used instead.
/// # Panics
/// Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point3(&self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Transforms the give 3D vector as a direction.
/// This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
/// `0.0`.
/// This method assumes that `self` contains a valid affine transform.
/// # Panics
/// Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn transform_vector3(&self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Transforms a 4D vector.

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec4(&self, #[proxy] rhs: bevy::math::DVec4) -> bevy::math::DVec4;

"#,
    r#"
/// Multiplies two 4x4 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn mul_mat4(&self, #[proxy] rhs: &glam::DMat4) -> bevy::math::DMat4;

"#,
    r#"
/// Adds two 4x4 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn add_mat4(&self, #[proxy] rhs: &glam::DMat4) -> bevy::math::DMat4;

"#,
    r#"
/// Subtracts two 4x4 matrices.

    #[lua(kind = "Method", output(proxy))]
    fn sub_mat4(&self, #[proxy] rhs: &glam::DMat4) -> bevy::math::DMat4;

"#,
    r#"
/// Multiplies a 4x4 matrix by a scalar.

    #[lua(kind = "Method", output(proxy))]
    fn mul_scalar(&self, rhs: f64) -> bevy::math::DMat4;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two matrices contain similar elements. It works best
/// when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(&self, #[proxy] rhs: bevy::math::DMat4, max_abs_diff: f64) -> bool;

"#,
    r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_mat4(&self) -> bevy::math::Mat4;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::DMat4;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::DMat4;

"#]
)]
pub struct DMat4 {
    #[lua(output(proxy))]
    x_axis: bevy::math::DVec4,
    #[lua(output(proxy))]
    y_axis: bevy::math::DVec4,
    #[lua(output(proxy))]
    z_axis: bevy::math::DVec4,
    #[lua(output(proxy))]
    w_axis: bevy::math::DVec4,
}
/// A 2D affine transform, which can represent translation, rotation, scaling and shear.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Affine2",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Affine2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Mat3A) -> bevy::math::Mat3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Mat3) -> bevy::math::Mat3;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::Affine2) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Affine2) -> bevy::math::Affine2;

"#,
    r#"
/// Creates an affine transform from three column vectors.

    #[lua(kind = "Function", output(proxy))]
    fn from_cols(
        #[proxy]
        x_axis: bevy::math::Vec2,
        #[proxy]
        y_axis: bevy::math::Vec2,
        #[proxy]
        z_axis: bevy::math::Vec2,
    ) -> bevy::math::Affine2;

"#,
    r#"
/// Creates a `[f32; 6]` array storing data in column major order.

    #[lua(kind = "Method")]
    fn to_cols_array(&self) -> [f32; 6];

"#,
    r#"
/// Creates a `[[f32; 2]; 3]` 2D array storing data in
/// column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array_2d(&self) -> [[f32; 2]; 3];

"#,
    r#"
/// Creates an affine transform that changes scale.
/// Note that if any scale is zero the transform will be non-invertible.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale(#[proxy] scale: bevy::math::Vec2) -> bevy::math::Affine2;

"#,
    r#"
/// Creates an affine transform from the given rotation `angle`.

    #[lua(kind = "Function", output(proxy))]
    fn from_angle(angle: f32) -> bevy::math::Affine2;

"#,
    r#"
/// Creates an affine transformation from the given 2D `translation`.

    #[lua(kind = "Function", output(proxy))]
    fn from_translation(#[proxy] translation: bevy::math::Vec2) -> bevy::math::Affine2;

"#,
    r#"
/// Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)

    #[lua(kind = "Function", output(proxy))]
    fn from_mat2(#[proxy] matrix2: bevy::math::Mat2) -> bevy::math::Affine2;

"#,
    r#"
/// Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
/// translation vector.
/// Equivalent to
/// `Affine2::from_translation(translation) * Affine2::from_mat2(mat2)`

    #[lua(kind = "Function", output(proxy))]
    fn from_mat2_translation(
        #[proxy]
        matrix2: bevy::math::Mat2,
        #[proxy]
        translation: bevy::math::Vec2,
    ) -> bevy::math::Affine2;

"#,
    r#"
/// Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
/// `translation`.
/// Equivalent to `Affine2::from_translation(translation) *
/// Affine2::from_angle(angle) * Affine2::from_scale(scale)`

    #[lua(kind = "Function", output(proxy))]
    fn from_scale_angle_translation(
        #[proxy]
        scale: bevy::math::Vec2,
        angle: f32,
        #[proxy]
        translation: bevy::math::Vec2,
    ) -> bevy::math::Affine2;

"#,
    r#"
/// Creates an affine transform from the given 2D rotation `angle` (in radians) and
/// `translation`.
/// Equivalent to `Affine2::from_translation(translation) * Affine2::from_angle(angle)`

    #[lua(kind = "Function", output(proxy))]
    fn from_angle_translation(
        angle: f32,
        #[proxy]
        translation: bevy::math::Vec2,
    ) -> bevy::math::Affine2;

"#,
    r#"
/// The given `Mat3` must be an affine transform,

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3(#[proxy] m: bevy::math::Mat3) -> bevy::math::Affine2;

"#,
    r#"
/// The given [`Mat3A`] must be an affine transform,

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3a(#[proxy] m: bevy::math::Mat3A) -> bevy::math::Affine2;

"#,
    r#"
/// Transforms the given 2D point, applying shear, scale, rotation and translation.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point2(&self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Transforms the given 2D vector, applying shear, scale and rotation (but NOT
/// translation).
/// To also apply translation, use [`Self::transform_point2()`] instead.

    #[lua(kind = "Method", output(proxy))]
    fn transform_vector2(&self, #[proxy] rhs: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return
/// `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(&self) -> bool;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two 3x4 matrices contain similar elements. It works
/// best when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(&self, #[proxy] rhs: bevy::math::Affine2, max_abs_diff: f32) -> bool;

"#,
    r#"
/// Return the inverse of this transform.
/// Note that if the transform is not invertible the result will be invalid.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(&self) -> bevy::math::Affine2;

"#]
)]
pub struct Affine2 {
    #[lua(output(proxy))]
    matrix2: bevy::math::Mat2,
    #[lua(output(proxy))]
    translation: bevy::math::Vec2,
}
/// A 3D affine transform, which can represent translation, rotation, scaling and shear.
/// This type is 16 byte aligned.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Affine3A",
    functions[r#"
/// Creates an affine transform from three column vectors.

    #[lua(kind = "Function", output(proxy))]
    fn from_cols(
        #[proxy]
        x_axis: bevy::math::Vec3A,
        #[proxy]
        y_axis: bevy::math::Vec3A,
        #[proxy]
        z_axis: bevy::math::Vec3A,
        #[proxy]
        w_axis: bevy::math::Vec3A,
    ) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates a `[f32; 12]` array storing data in column major order.

    #[lua(kind = "Method")]
    fn to_cols_array(&self) -> [f32; 12];

"#,
    r#"
/// Creates a `[[f32; 3]; 4]` 3D array storing data in
/// column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array_2d(&self) -> [[f32; 3]; 4];

"#,
    r#"
/// Creates an affine transform that changes scale.
/// Note that if any scale is zero the transform will be non-invertible.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale(#[proxy] scale: bevy::math::Vec3) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates an affine transform from the given `rotation` quaternion.

    #[lua(kind = "Function", output(proxy))]
    fn from_quat(#[proxy] rotation: bevy::math::Quat) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates an affine transform containing a 3D rotation around a normalized
/// rotation `axis` of `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_axis_angle(
        #[proxy]
        axis: bevy::math::Vec3,
        angle: f32,
    ) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates an affine transform containing a 3D rotation around the x axis of
/// `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_x(angle: f32) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates an affine transform containing a 3D rotation around the y axis of
/// `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_y(angle: f32) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates an affine transform containing a 3D rotation around the z axis of
/// `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_z(angle: f32) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates an affine transformation from the given 3D `translation`.

    #[lua(kind = "Function", output(proxy))]
    fn from_translation(#[proxy] translation: bevy::math::Vec3) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates an affine transform from a 3x3 matrix (expressing scale, shear and
/// rotation)

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3(#[proxy] mat3: bevy::math::Mat3) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
/// and a translation vector.
/// Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_mat3(mat3)`

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3_translation(
        #[proxy]
        mat3: bevy::math::Mat3,
        #[proxy]
        translation: bevy::math::Vec3,
    ) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates an affine transform from the given 3D `scale`, `rotation` and
/// `translation`.
/// Equivalent to `Affine3A::from_translation(translation) *
/// Affine3A::from_quat(rotation) * Affine3A::from_scale(scale)`

    #[lua(kind = "Function", output(proxy))]
    fn from_scale_rotation_translation(
        #[proxy]
        scale: bevy::math::Vec3,
        #[proxy]
        rotation: bevy::math::Quat,
        #[proxy]
        translation: bevy::math::Vec3,
    ) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates an affine transform from the given 3D `rotation` and `translation`.
/// Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_quat(rotation)`

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_translation(
        #[proxy]
        rotation: bevy::math::Quat,
        #[proxy]
        translation: bevy::math::Vec3,
    ) -> bevy::math::Affine3A;

"#,
    r#"
/// The given `Mat4` must be an affine transform,
/// i.e. contain no perspective transform.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat4(#[proxy] m: bevy::math::Mat4) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates a left-handed view transform using a camera position, an up direction, and a facing
/// direction.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

    #[lua(kind = "Function", output(proxy))]
    fn look_to_lh(
        #[proxy]
        eye: bevy::math::Vec3,
        #[proxy]
        dir: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates a right-handed view transform using a camera position, an up direction, and a facing
/// direction.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

    #[lua(kind = "Function", output(proxy))]
    fn look_to_rh(
        #[proxy]
        eye: bevy::math::Vec3,
        #[proxy]
        dir: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates a left-handed view transform using a camera position, an up direction, and a focal
/// point.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
/// # Panics
/// Will panic if `up` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn look_at_lh(
        #[proxy]
        eye: bevy::math::Vec3,
        #[proxy]
        center: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> bevy::math::Affine3A;

"#,
    r#"
/// Creates a right-handed view transform using a camera position, an up direction, and a focal
/// point.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
/// # Panics
/// Will panic if `up` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn look_at_rh(
        #[proxy]
        eye: bevy::math::Vec3,
        #[proxy]
        center: bevy::math::Vec3,
        #[proxy]
        up: bevy::math::Vec3,
    ) -> bevy::math::Affine3A;

"#,
    r#"
/// Transforms the given 3D points, applying shear, scale, rotation and translation.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point3(&self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Transforms the given 3D vector, applying shear, scale and rotation (but NOT
/// translation).
/// To also apply translation, use [`Self::transform_point3()`] instead.

    #[lua(kind = "Method", output(proxy))]
    fn transform_vector3(&self, #[proxy] rhs: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"
/// Transforms the given [`Vec3A`], applying shear, scale, rotation and translation.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point3a(&self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Transforms the given [`Vec3A`], applying shear, scale and rotation (but NOT
/// translation).
/// To also apply translation, use [`Self::transform_point3a()`] instead.

    #[lua(kind = "Method", output(proxy))]
    fn transform_vector3a(&self, #[proxy] rhs: bevy::math::Vec3A) -> bevy::math::Vec3A;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return
/// `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(&self) -> bool;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two 3x4 matrices contain similar elements. It works
/// best when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(&self, #[proxy] rhs: bevy::math::Affine3A, max_abs_diff: f32) -> bool;

"#,
    r#"
/// Return the inverse of this transform.
/// Note that if the transform is not invertible the result will be invalid.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(&self) -> bevy::math::Affine3A;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::Affine3A) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Affine3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Affine3A) -> bevy::math::Affine3A;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::Mat4) -> bevy::math::Mat4;

"#]
)]
pub struct Affine3A {
    #[lua(output(proxy))]
    matrix3: bevy::math::Mat3A,
    #[lua(output(proxy))]
    translation: bevy::math::Vec3A,
}
/// A 2D affine transform, which can represent translation, rotation, scaling and shear.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DAffine2",
    functions[r#"
/// Creates an affine transform from three column vectors.

    #[lua(kind = "Function", output(proxy))]
    fn from_cols(
        #[proxy]
        x_axis: bevy::math::DVec2,
        #[proxy]
        y_axis: bevy::math::DVec2,
        #[proxy]
        z_axis: bevy::math::DVec2,
    ) -> bevy::math::DAffine2;

"#,
    r#"
/// Creates a `[f64; 6]` array storing data in column major order.

    #[lua(kind = "Method")]
    fn to_cols_array(&self) -> [f64; 6];

"#,
    r#"
/// Creates a `[[f64; 2]; 3]` 2D array storing data in
/// column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array_2d(&self) -> [[f64; 2]; 3];

"#,
    r#"
/// Creates an affine transform that changes scale.
/// Note that if any scale is zero the transform will be non-invertible.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale(#[proxy] scale: bevy::math::DVec2) -> bevy::math::DAffine2;

"#,
    r#"
/// Creates an affine transform from the given rotation `angle`.

    #[lua(kind = "Function", output(proxy))]
    fn from_angle(angle: f64) -> bevy::math::DAffine2;

"#,
    r#"
/// Creates an affine transformation from the given 2D `translation`.

    #[lua(kind = "Function", output(proxy))]
    fn from_translation(#[proxy] translation: bevy::math::DVec2) -> bevy::math::DAffine2;

"#,
    r#"
/// Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)

    #[lua(kind = "Function", output(proxy))]
    fn from_mat2(#[proxy] matrix2: bevy::math::DMat2) -> bevy::math::DAffine2;

"#,
    r#"
/// Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
/// translation vector.
/// Equivalent to
/// `DAffine2::from_translation(translation) * DAffine2::from_mat2(mat2)`

    #[lua(kind = "Function", output(proxy))]
    fn from_mat2_translation(
        #[proxy]
        matrix2: bevy::math::DMat2,
        #[proxy]
        translation: bevy::math::DVec2,
    ) -> bevy::math::DAffine2;

"#,
    r#"
/// Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
/// `translation`.
/// Equivalent to `DAffine2::from_translation(translation) *
/// DAffine2::from_angle(angle) * DAffine2::from_scale(scale)`

    #[lua(kind = "Function", output(proxy))]
    fn from_scale_angle_translation(
        #[proxy]
        scale: bevy::math::DVec2,
        angle: f64,
        #[proxy]
        translation: bevy::math::DVec2,
    ) -> bevy::math::DAffine2;

"#,
    r#"
/// Creates an affine transform from the given 2D rotation `angle` (in radians) and
/// `translation`.
/// Equivalent to `DAffine2::from_translation(translation) * DAffine2::from_angle(angle)`

    #[lua(kind = "Function", output(proxy))]
    fn from_angle_translation(
        angle: f64,
        #[proxy]
        translation: bevy::math::DVec2,
    ) -> bevy::math::DAffine2;

"#,
    r#"
/// The given `DMat3` must be an affine transform,

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3(#[proxy] m: bevy::math::DMat3) -> bevy::math::DAffine2;

"#,
    r#"
/// Transforms the given 2D point, applying shear, scale, rotation and translation.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point2(&self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Transforms the given 2D vector, applying shear, scale and rotation (but NOT
/// translation).
/// To also apply translation, use [`Self::transform_point2()`] instead.

    #[lua(kind = "Method", output(proxy))]
    fn transform_vector2(&self, #[proxy] rhs: bevy::math::DVec2) -> bevy::math::DVec2;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return
/// `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(&self) -> bool;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two 3x4 matrices contain similar elements. It works
/// best when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(&self, #[proxy] rhs: bevy::math::DAffine2, max_abs_diff: f64) -> bool;

"#,
    r#"
/// Return the inverse of this transform.
/// Note that if the transform is not invertible the result will be invalid.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(&self) -> bevy::math::DAffine2;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::DAffine2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DAffine2) -> bevy::math::DAffine2;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DMat3) -> bevy::math::DMat3;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::DAffine2) -> bool;

"#]
)]
pub struct DAffine2 {
    #[lua(output(proxy))]
    matrix2: bevy::math::DMat2,
    #[lua(output(proxy))]
    translation: bevy::math::DVec2,
}
/// A 3D affine transform, which can represent translation, rotation, scaling and shear.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DAffine3",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DMat4) -> bevy::math::DMat4;

"#,
    r#"
/// Creates an affine transform from three column vectors.

    #[lua(kind = "Function", output(proxy))]
    fn from_cols(
        #[proxy]
        x_axis: bevy::math::DVec3,
        #[proxy]
        y_axis: bevy::math::DVec3,
        #[proxy]
        z_axis: bevy::math::DVec3,
        #[proxy]
        w_axis: bevy::math::DVec3,
    ) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates a `[f64; 12]` array storing data in column major order.

    #[lua(kind = "Method")]
    fn to_cols_array(&self) -> [f64; 12];

"#,
    r#"
/// Creates a `[[f64; 3]; 4]` 3D array storing data in
/// column major order.
/// If you require data in row major order `transpose` the matrix first.

    #[lua(kind = "Method")]
    fn to_cols_array_2d(&self) -> [[f64; 3]; 4];

"#,
    r#"
/// Creates an affine transform that changes scale.
/// Note that if any scale is zero the transform will be non-invertible.

    #[lua(kind = "Function", output(proxy))]
    fn from_scale(#[proxy] scale: bevy::math::DVec3) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates an affine transform from the given `rotation` quaternion.

    #[lua(kind = "Function", output(proxy))]
    fn from_quat(#[proxy] rotation: bevy::math::DQuat) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates an affine transform containing a 3D rotation around a normalized
/// rotation `axis` of `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_axis_angle(
        #[proxy]
        axis: bevy::math::DVec3,
        angle: f64,
    ) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates an affine transform containing a 3D rotation around the x axis of
/// `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_x(angle: f64) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates an affine transform containing a 3D rotation around the y axis of
/// `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_y(angle: f64) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates an affine transform containing a 3D rotation around the z axis of
/// `angle` (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_z(angle: f64) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates an affine transformation from the given 3D `translation`.

    #[lua(kind = "Function", output(proxy))]
    fn from_translation(#[proxy] translation: bevy::math::DVec3) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates an affine transform from a 3x3 matrix (expressing scale, shear and
/// rotation)

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3(#[proxy] mat3: bevy::math::DMat3) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
/// and a translation vector.
/// Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_mat3(mat3)`

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3_translation(
        #[proxy]
        mat3: bevy::math::DMat3,
        #[proxy]
        translation: bevy::math::DVec3,
    ) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates an affine transform from the given 3D `scale`, `rotation` and
/// `translation`.
/// Equivalent to `DAffine3::from_translation(translation) *
/// DAffine3::from_quat(rotation) * DAffine3::from_scale(scale)`

    #[lua(kind = "Function", output(proxy))]
    fn from_scale_rotation_translation(
        #[proxy]
        scale: bevy::math::DVec3,
        #[proxy]
        rotation: bevy::math::DQuat,
        #[proxy]
        translation: bevy::math::DVec3,
    ) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates an affine transform from the given 3D `rotation` and `translation`.
/// Equivalent to `DAffine3::from_translation(translation) * DAffine3::from_quat(rotation)`

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_translation(
        #[proxy]
        rotation: bevy::math::DQuat,
        #[proxy]
        translation: bevy::math::DVec3,
    ) -> bevy::math::DAffine3;

"#,
    r#"
/// The given `DMat4` must be an affine transform,
/// i.e. contain no perspective transform.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat4(#[proxy] m: bevy::math::DMat4) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates a left-handed view transform using a camera position, an up direction, and a facing
/// direction.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.

    #[lua(kind = "Function", output(proxy))]
    fn look_to_lh(
        #[proxy]
        eye: bevy::math::DVec3,
        #[proxy]
        dir: bevy::math::DVec3,
        #[proxy]
        up: bevy::math::DVec3,
    ) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates a right-handed view transform using a camera position, an up direction, and a facing
/// direction.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.

    #[lua(kind = "Function", output(proxy))]
    fn look_to_rh(
        #[proxy]
        eye: bevy::math::DVec3,
        #[proxy]
        dir: bevy::math::DVec3,
        #[proxy]
        up: bevy::math::DVec3,
    ) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates a left-handed view transform using a camera position, an up direction, and a focal
/// point.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
/// # Panics
/// Will panic if `up` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn look_at_lh(
        #[proxy]
        eye: bevy::math::DVec3,
        #[proxy]
        center: bevy::math::DVec3,
        #[proxy]
        up: bevy::math::DVec3,
    ) -> bevy::math::DAffine3;

"#,
    r#"
/// Creates a right-handed view transform using a camera position, an up direction, and a focal
/// point.
/// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
/// # Panics
/// Will panic if `up` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn look_at_rh(
        #[proxy]
        eye: bevy::math::DVec3,
        #[proxy]
        center: bevy::math::DVec3,
        #[proxy]
        up: bevy::math::DVec3,
    ) -> bevy::math::DAffine3;

"#,
    r#"
/// Transforms the given 3D points, applying shear, scale, rotation and translation.

    #[lua(kind = "Method", output(proxy))]
    fn transform_point3(&self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Transforms the given 3D vector, applying shear, scale and rotation (but NOT
/// translation).
/// To also apply translation, use [`Self::transform_point3()`] instead.

    #[lua(kind = "Method", output(proxy))]
    fn transform_vector3(&self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return
/// `false`.

    #[lua(kind = "Method")]
    fn is_finite(&self) -> bool;

"#,
    r#"
/// Returns `true` if any elements are `NaN`.

    #[lua(kind = "Method")]
    fn is_nan(&self) -> bool;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two 3x4 matrices contain similar elements. It works
/// best when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(&self, #[proxy] rhs: bevy::math::DAffine3, max_abs_diff: f64) -> bool;

"#,
    r#"
/// Return the inverse of this transform.
/// Note that if the transform is not invertible the result will be invalid.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(&self) -> bevy::math::DAffine3;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DAffine3) -> bevy::math::DAffine3;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::DAffine3) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::DAffine3;

"#]
)]
pub struct DAffine3 {
    #[lua(output(proxy))]
    matrix3: bevy::math::DMat3,
    #[lua(output(proxy))]
    translation: bevy::math::DVec3,
}
/// A quaternion representing an orientation.
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DQuat",
    functions[r#"
/// Creates a new rotation quaternion.
/// This should generally not be called manually unless you know what you are doing.
/// Use one of the other constructors instead such as `identity` or `from_axis_angle`.
/// `from_xyzw` is mostly used by unit tests and `serde` deserialization.
/// # Preconditions
/// This function does not check if the input is normalized, it is up to the user to
/// provide normalized input or to normalized the resulting quaternion.

    #[lua(kind = "Function", output(proxy))]
    fn from_xyzw(x: f64, y: f64, z: f64, w: f64) -> bevy::math::DQuat;

"#,
    r#"
/// Creates a rotation quaternion from an array.
/// # Preconditions
/// This function does not check if the input is normalized, it is up to the user to
/// provide normalized input or to normalized the resulting quaternion.

    #[lua(kind = "Function", output(proxy))]
    fn from_array(a: [f64; 4]) -> bevy::math::DQuat;

"#,
    r#"
/// Creates a new rotation quaternion from a 4D vector.
/// # Preconditions
/// This function does not check if the input is normalized, it is up to the user to
/// provide normalized input or to normalized the resulting quaternion.

    #[lua(kind = "Function", output(proxy))]
    fn from_vec4(#[proxy] v: bevy::math::DVec4) -> bevy::math::DQuat;

"#,
    r#"
/// Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
/// The axis must be a unit vector.
/// # Panics
/// Will panic if `axis` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_axis_angle(
        #[proxy]
        axis: bevy::math::DVec3,
        angle: f64,
    ) -> bevy::math::DQuat;

"#,
    r#"
/// Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
/// `from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.

    #[lua(kind = "Function", output(proxy))]
    fn from_scaled_axis(#[proxy] v: bevy::math::DVec3) -> bevy::math::DQuat;

"#,
    r#"
/// Creates a quaternion from the `angle` (in radians) around the x axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_x(angle: f64) -> bevy::math::DQuat;

"#,
    r#"
/// Creates a quaternion from the `angle` (in radians) around the y axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_y(angle: f64) -> bevy::math::DQuat;

"#,
    r#"
/// Creates a quaternion from the `angle` (in radians) around the z axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_z(angle: f64) -> bevy::math::DQuat;

"#,
    r#"
/// Creates a quaternion from the given Euler rotation sequence and the angles (in radians).

    #[lua(kind = "Function", output(proxy))]
    fn from_euler(
        #[proxy]
        euler: bevy::math::EulerRot,
        a: f64,
        b: f64,
        c: f64,
    ) -> bevy::math::DQuat;

"#,
    r#"
/// Creates a quaternion from a 3x3 rotation matrix.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat3(#[proxy] mat: &glam::DMat3) -> bevy::math::DQuat;

"#,
    r#"
/// Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.

    #[lua(kind = "Function", output(proxy))]
    fn from_mat4(#[proxy] mat: &glam::DMat4) -> bevy::math::DQuat;

"#,
    r#"
/// Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
/// plane spanned by the two vectors.  Will rotate at most 180 degrees.
/// The inputs must be unit vectors.
/// `from_rotation_arc(from, to) * from ≈ to`.
/// For near-singular cases (from≈to and from≈-to) the current implementation
/// is only accurate to about 0.001 (for `f32`).
/// # Panics
/// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_arc(
        #[proxy]
        from: bevy::math::DVec3,
        #[proxy]
        to: bevy::math::DVec3,
    ) -> bevy::math::DQuat;

"#,
    r#"
/// Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
/// that the resulting quaternion will rotate `from` so that it is colinear with `to`.
/// The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
/// degrees.
/// The inputs must be unit vectors.
/// `to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
/// # Panics
/// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_arc_colinear(
        #[proxy]
        from: bevy::math::DVec3,
        #[proxy]
        to: bevy::math::DVec3,
    ) -> bevy::math::DQuat;

"#,
    r#"
/// Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
/// around the z axis. Will rotate at most 180 degrees.
/// The inputs must be unit vectors.
/// `from_rotation_arc_2d(from, to) * from ≈ to`.
/// For near-singular cases (from≈to and from≈-to) the current implementation
/// is only accurate to about 0.001 (for `f32`).
/// # Panics
/// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Function", output(proxy))]
    fn from_rotation_arc_2d(
        #[proxy]
        from: bevy::math::DVec2,
        #[proxy]
        to: bevy::math::DVec2,
    ) -> bevy::math::DQuat;

"#,
    r#"
/// Returns the rotation axis scaled by the rotation in radians.

    #[lua(kind = "Method", output(proxy))]
    fn to_scaled_axis(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns the rotation angles for the given euler rotation sequence.

    #[lua(kind = "Method")]
    fn to_euler(self, #[proxy] euler: bevy::math::EulerRot) -> (f64, f64, f64);

"#,
    r#"
/// `[x, y, z, w]`

    #[lua(kind = "Method")]
    fn to_array(&self) -> [f64; 4];

"#,
    r#"
/// Returns the vector part of the quaternion.

    #[lua(kind = "Method", output(proxy))]
    fn xyz(self) -> bevy::math::DVec3;

"#,
    r#"
/// Returns the quaternion conjugate of `self`. For a unit quaternion the
/// conjugate is also the inverse.

    #[lua(kind = "Method", output(proxy))]
    fn conjugate(self) -> bevy::math::DQuat;

"#,
    r#"
/// Returns the inverse of a normalized quaternion.
/// Typically quaternion inverse returns the conjugate of a normalized quaternion.
/// Because `self` is assumed to already be unit length this method *does not* normalize
/// before returning the conjugate.
/// # Panics
/// Will panic if `self` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn inverse(self) -> bevy::math::DQuat;

"#,
    r#"
/// Computes the dot product of `self` and `rhs`. The dot product is
/// equal to the cosine of the angle between two quaternion rotations.

    #[lua(kind = "Method")]
    fn dot(self, #[proxy] rhs: bevy::math::DQuat) -> f64;

"#,
    r#"
/// Computes the length of `self`.

    #[lua(kind = "Method")]
    fn length(self) -> f64;

"#,
    r#"
/// Computes the squared length of `self`.
/// This is generally faster than `length()` as it avoids a square
/// root operation.

    #[lua(kind = "Method")]
    fn length_squared(self) -> f64;

"#,
    r#"
/// Computes `1.0 / length()`.
/// For valid results, `self` must _not_ be of length zero.

    #[lua(kind = "Method")]
    fn length_recip(self) -> f64;

"#,
    r#"
/// Returns `self` normalized to length 1.0.
/// For valid results, `self` must _not_ be of length zero.
/// Panics
/// Will panic if `self` is zero length when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn normalize(self) -> bevy::math::DQuat;

"#,
    r#"
/// Returns `true` if, and only if, all elements are finite.
/// If any element is either `NaN`, positive or negative infinity, this will return `false`.

    #[lua(kind = "Method")]
    fn is_finite(self) -> bool;

"#,
    r#"

    #[lua(kind = "Method")]
    fn is_nan(self) -> bool;

"#,
    r#"
/// Returns whether `self` of length `1.0` or not.
/// Uses a precision threshold of `1e-6`.

    #[lua(kind = "Method")]
    fn is_normalized(self) -> bool;

"#,
    r#"

    #[lua(kind = "Method")]
    fn is_near_identity(self) -> bool;

"#,
    r#"
/// Returns the angle (in radians) for the minimal rotation
/// for transforming this quaternion into another.
/// Both quaternions must be normalized.
/// # Panics
/// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method")]
    fn angle_between(self, #[proxy] rhs: bevy::math::DQuat) -> f64;

"#,
    r#"
/// Returns true if the absolute difference of all elements between `self` and `rhs`
/// is less than or equal to `max_abs_diff`.
/// This can be used to compare if two quaternions contain similar elements. It works
/// best when comparing with a known value. The `max_abs_diff` that should be used used
/// depends on the values being compared against.
/// For more see
/// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).

    #[lua(kind = "Method")]
    fn abs_diff_eq(self, #[proxy] rhs: bevy::math::DQuat, max_abs_diff: f64) -> bool;

"#,
    r#"
/// Performs a linear interpolation between `self` and `rhs` based on
/// the value `s`.
/// When `s` is `0.0`, the result will be equal to `self`.  When `s`
/// is `1.0`, the result will be equal to `rhs`.
/// # Panics
/// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn lerp(self, #[proxy] end: bevy::math::DQuat, s: f64) -> bevy::math::DQuat;

"#,
    r#"
/// Performs a spherical linear interpolation between `self` and `end`
/// based on the value `s`.
/// When `s` is `0.0`, the result will be equal to `self`.  When `s`
/// is `1.0`, the result will be equal to `end`.
/// # Panics
/// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn slerp(self, #[proxy] end: bevy::math::DQuat, s: f64) -> bevy::math::DQuat;

"#,
    r#"
/// Multiplies a quaternion and a 3D vector, returning the rotated vector.
/// # Panics
/// Will panic if `self` is not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn mul_vec3(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Multiplies two quaternions. If they each represent a rotation, the result will
/// represent the combined rotation.
/// Note that due to floating point rounding the result may not be perfectly normalized.
/// # Panics
/// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

    #[lua(kind = "Method", output(proxy))]
    fn mul_quat(self, #[proxy] rhs: bevy::math::DQuat) -> bevy::math::DQuat;

"#,
    r#"
/// Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.

    #[lua(kind = "Function", output(proxy))]
    fn from_affine3(#[proxy] a: &glam::DAffine3) -> bevy::math::DQuat;

"#,
    r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_quat(self) -> bevy::math::Quat;

"#,
    r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_f32(self) -> bevy::math::Quat;

"#,
    r#"
/// Divides a quaternion by a scalar value.
/// The quotient is not guaranteed to be normalized.

    #[lua(
        as_trait = "std::ops::Div",
        kind = "MetaFunction",
        output(proxy),
        composite = "div",
        metamethod = "Div",
    )]
    fn div(self, rhs: f64) -> bevy::math::DQuat;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::DQuat;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::DQuat) -> bool;

"#,
    r#"
/// Adds two quaternions.
/// The sum is not guaranteed to be normalized.
/// Note that addition is not the same as combining the rotations represented by the
/// two quaternions! That corresponds to multiplication.

    #[lua(
        as_trait = "std::ops::Add",
        kind = "MetaFunction",
        output(proxy),
        composite = "add",
        metamethod = "Add",
    )]
    fn add(self, #[proxy] rhs: bevy::math::DQuat) -> bevy::math::DQuat;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::DQuat;

"#,
    r#"
/// Multiplies a quaternion by a scalar value.
/// The product is not guaranteed to be normalized.

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f64) -> bevy::math::DQuat;

"#,
    r#"
/// Subtracts the `rhs` quaternion from `self`.
/// The difference is not guaranteed to be normalized.

    #[lua(
        as_trait = "std::ops::Sub",
        kind = "MetaFunction",
        output(proxy),
        composite = "sub",
        metamethod = "Sub",
    )]
    fn sub(self, #[proxy] rhs: bevy::math::DQuat) -> bevy::math::DQuat;

"#,
    r#"
/// Multiplies a quaternion and a 3D vector, returning the rotated vector.
/// # Panics
/// Will panic if `self` is not normalized when `glam_assert` is enabled.

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DVec3) -> bevy::math::DVec3;

"#,
    r#"
/// Multiplies two quaternions. If they each represent a rotation, the result will
/// represent the combined rotation.
/// Note that due to floating point rounding the result may not be perfectly
/// normalized.
/// # Panics
/// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, #[proxy] rhs: bevy::math::DQuat) -> bevy::math::DQuat;

"#]
)]
pub struct DQuat {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
/// Euler rotation sequences.
/// The angles are applied starting from the right.
/// E.g. XYZ will first apply the z-axis rotation.
/// YXZ can be used for yaw (y-axis), pitch (x-axis), roll (z-axis).
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::EulerRot",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::EulerRot;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &glam::EulerRot) -> bool;

"#]
)]
pub struct EulerRot {}
/// A 3-dimensional SIMD vector mask.
/// This type is 16 byte aligned.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::BVec3A",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::BVec3A) -> bool;

"#,
    r#"
/// Creates a new vector mask.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: bool, y: bool, z: bool) -> bevy::math::BVec3A;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: bool) -> bevy::math::BVec3A;

"#,
    r#"
/// Returns a bitmask with the lowest 3 bits set from the elements of `self`.
/// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn bitmask(self) -> u32;

"#,
    r#"
/// Returns true if any of the elements are true, false otherwise.

    #[lua(kind = "Method")]
    fn any(self) -> bool;

"#,
    r#"
/// Returns true if all the elements are true, false otherwise.

    #[lua(kind = "Method")]
    fn all(self) -> bool;

"#,
    r#"
/// Tests the value at `index`.
/// Panics if `index` is greater than 2.

    #[lua(kind = "Method")]
    fn test(&self, index: usize) -> bool;

"#,
    r#"
/// Sets the element at `index`.
/// Panics if `index` is greater than 2.

    #[lua(kind = "MutatingMethod")]
    fn set(&mut self, index: usize, value: bool) -> ();

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::BVec3A;

"#]
)]
pub struct BVec3A();
/// A 4-dimensional SIMD vector mask.
/// This type is 16 byte aligned.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::BVec4A",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::BVec4A;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] rhs: &glam::BVec4A) -> bool;

"#,
    r#"
/// Creates a new vector mask.

    #[lua(kind = "Function", output(proxy))]
    fn new(x: bool, y: bool, z: bool, w: bool) -> bevy::math::BVec4A;

"#,
    r#"
/// Creates a vector with all elements set to `v`.

    #[lua(kind = "Function", output(proxy))]
    fn splat(v: bool) -> bevy::math::BVec4A;

"#,
    r#"
/// Returns a bitmask with the lowest 4 bits set from the elements of `self`.
/// A true element results in a `1` bit and a false element in a `0` bit.  Element `x` goes
/// into the first lowest bit, element `y` into the second, etc.

    #[lua(kind = "Method")]
    fn bitmask(self) -> u32;

"#,
    r#"
/// Returns true if any of the elements are true, false otherwise.

    #[lua(kind = "Method")]
    fn any(self) -> bool;

"#,
    r#"
/// Returns true if all the elements are true, false otherwise.

    #[lua(kind = "Method")]
    fn all(self) -> bool;

"#,
    r#"
/// Tests the value at `index`.
/// Panics if `index` is greater than 3.

    #[lua(kind = "Method")]
    fn test(&self, index: usize) -> bool;

"#,
    r#"
/// Sets the element at `index`.
/// Panics if `index` is greater than 3.

    #[lua(kind = "MutatingMethod")]
    fn set(&mut self, index: usize, value: bool) -> ();

"#]
)]
pub struct BVec4A();
/// A normalized vector pointing in a direction in 2D space
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Direction2d",
    functions[r#"
/// Create a [`Direction2d`] from a [`Vec2`] that is already normalized.
/// # Warning
/// `value` must be normalized, i.e it's length must be `1.0`.

    #[lua(kind = "Function", output(proxy))]
    fn new_unchecked(
        #[proxy]
        value: bevy::math::Vec2,
    ) -> bevy::math::primitives::Direction2d;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Direction2d;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::primitives::Direction2d;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Direction2d) -> bool;

"#]
)]
pub struct Direction2d();
/// A circle primitive
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Circle",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Circle;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Circle) -> bool;

"#,
    r#"
/// Create a new [`Circle`] from a `radius`

    #[lua(kind = "Function", output(proxy))]
    fn new(radius: f32) -> bevy::math::primitives::Circle;

"#,
    r#"
/// Get the diameter of the circle

    #[lua(kind = "Method")]
    fn diameter(&self) -> f32;

"#,
    r#"
/// Get the area of the circle

    #[lua(kind = "Method")]
    fn area(&self) -> f32;

"#,
    r#"
/// Get the perimeter or circumference of the circle

    #[lua(kind = "Method")]
    fn perimeter(&self) -> f32;

"#,
    r#"
/// Finds the point on the circle that is closest to the given `point`.
/// If the point is outside the circle, the returned point will be on the perimeter of the circle.
/// Otherwise, it will be inside the circle and returned as is.

    #[lua(kind = "Method", output(proxy))]
    fn closest_point(&self, #[proxy] point: bevy::math::Vec2) -> bevy::math::Vec2;

"#]
)]
pub struct Circle {
    radius: f32,
}
/// An ellipse primitive
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Ellipse",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Ellipse) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Ellipse;

"#,
    r#"
/// Create a new `Ellipse` from half of its width and height.
/// This corresponds to the two perpendicular radii defining the ellipse.

    #[lua(kind = "Function", output(proxy))]
    fn new(half_width: f32, half_height: f32) -> bevy::math::primitives::Ellipse;

"#,
    r#"
/// Create a new `Ellipse` from a given full size.
/// `size.x` is the diameter along the X axis, and `size.y` is the diameter along the Y axis.

    #[lua(kind = "Function", output(proxy))]
    fn from_size(#[proxy] size: bevy::math::Vec2) -> bevy::math::primitives::Ellipse;

"#,
    r#"
/// Returns the length of the semi-major axis. This corresponds to the longest radius of the ellipse.

    #[lua(kind = "Method")]
    fn semi_major(self) -> f32;

"#,
    r#"
/// Returns the length of the semi-minor axis. This corresponds to the shortest radius of the ellipse.

    #[lua(kind = "Method")]
    fn semi_minor(self) -> f32;

"#,
    r#"
/// Get the area of the ellipse

    #[lua(kind = "Method")]
    fn area(&self) -> f32;

"#]
)]
pub struct Ellipse {
    #[lua(output(proxy))]
    half_size: bevy::math::Vec2,
}
/// An unbounded plane in 2D space. It forms a separating surface through the origin,
/// stretching infinitely far
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Plane2d",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Plane2d;

"#,
    r#"
/// Create a new `Plane2d` from a normal
/// # Panics
/// Panics if the given `normal` is zero (or very close to zero), or non-finite.

    #[lua(kind = "Function", output(proxy))]
    fn new(#[proxy] normal: bevy::math::Vec2) -> bevy::math::primitives::Plane2d;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Plane2d) -> bool;

"#]
)]
pub struct Plane2d {
    #[lua(output(proxy))]
    normal: bevy::math::primitives::Direction2d,
}
/// An infinite line along a direction in 2D space.
/// For a finite line: [`Segment2d`]
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Line2d",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Line2d) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Line2d;

"#]
)]
pub struct Line2d {
    #[lua(output(proxy))]
    direction: bevy::math::primitives::Direction2d,
}
/// A segment of a line along a direction in 2D space.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Segment2d",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Segment2d) -> bool;

"#,
    r#"
/// Create a new `Segment2d` from a direction and full length of the segment

    #[lua(kind = "Function", output(proxy))]
    fn new(
        #[proxy]
        direction: bevy::math::primitives::Direction2d,
        length: f32,
    ) -> bevy::math::primitives::Segment2d;

"#,
    r#"
/// Get the position of the first point on the line segment

    #[lua(kind = "Method", output(proxy))]
    fn point1(&self) -> bevy::math::Vec2;

"#,
    r#"
/// Get the position of the second point on the line segment

    #[lua(kind = "Method", output(proxy))]
    fn point2(&self) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Segment2d;

"#]
)]
pub struct Segment2d {
    #[lua(output(proxy))]
    direction: bevy::math::primitives::Direction2d,
    half_length: f32,
}
/// A triangle in 2D space
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Triangle2d",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Triangle2d) -> bool;

"#,
    r#"
/// Create a new `Triangle2d` from points `a`, `b`, and `c`

    #[lua(kind = "Function", output(proxy))]
    fn new(
        #[proxy]
        a: bevy::math::Vec2,
        #[proxy]
        b: bevy::math::Vec2,
        #[proxy]
        c: bevy::math::Vec2,
    ) -> bevy::math::primitives::Triangle2d;

"#,
    r#"
/// Get the area of the triangle

    #[lua(kind = "Method")]
    fn area(&self) -> f32;

"#,
    r#"
/// Get the perimeter of the triangle

    #[lua(kind = "Method")]
    fn perimeter(&self) -> f32;

"#,
    r#"
/// Reverse the [`WindingOrder`] of the triangle
/// by swapping the second and third vertices

    #[lua(kind = "MutatingMethod")]
    fn reverse(&mut self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Triangle2d;

"#]
)]
pub struct Triangle2d {
    vertices: ReflectedValue,
}
/// A rectangle primitive
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Rectangle",
    functions[r#"
/// Create a new `Rectangle` from a full width and height

    #[lua(kind = "Function", output(proxy))]
    fn new(width: f32, height: f32) -> bevy::math::primitives::Rectangle;

"#,
    r#"
/// Create a new `Rectangle` from a given full size

    #[lua(kind = "Function", output(proxy))]
    fn from_size(#[proxy] size: bevy::math::Vec2) -> bevy::math::primitives::Rectangle;

"#,
    r#"
/// Create a new `Rectangle` from two corner points

    #[lua(kind = "Function", output(proxy))]
    fn from_corners(
        #[proxy]
        point1: bevy::math::Vec2,
        #[proxy]
        point2: bevy::math::Vec2,
    ) -> bevy::math::primitives::Rectangle;

"#,
    r#"
/// Get the size of the rectangle

    #[lua(kind = "Method", output(proxy))]
    fn size(&self) -> bevy::math::Vec2;

"#,
    r#"
/// Get the area of the rectangle

    #[lua(kind = "Method")]
    fn area(&self) -> f32;

"#,
    r#"
/// Get the perimeter of the rectangle

    #[lua(kind = "Method")]
    fn perimeter(&self) -> f32;

"#,
    r#"
/// Finds the point on the rectangle that is closest to the given `point`.
/// If the point is outside the rectangle, the returned point will be on the perimeter of the rectangle.
/// Otherwise, it will be inside the rectangle and returned as is.

    #[lua(kind = "Method", output(proxy))]
    fn closest_point(&self, #[proxy] point: bevy::math::Vec2) -> bevy::math::Vec2;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Rectangle;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Rectangle) -> bool;

"#]
)]
pub struct Rectangle {
    #[lua(output(proxy))]
    half_size: bevy::math::Vec2,
}
/// A polygon where all vertices lie on a circle, equally far apart.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::RegularPolygon",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::RegularPolygon;

"#,
    r#"
/// Create a new `RegularPolygon`
/// from the radius of the circumcircle and a number of sides
/// # Panics
/// Panics if `circumradius` is non-positive

    #[lua(kind = "Function", output(proxy))]
    fn new(circumradius: f32, sides: usize) -> bevy::math::primitives::RegularPolygon;

"#,
    r#"
/// Get the radius of the circumcircle on which all vertices
/// of the regular polygon lie

    #[lua(kind = "Method")]
    fn circumradius(&self) -> f32;

"#,
    r#"
/// Get the inradius or apothem of the regular polygon.
/// This is the radius of the largest circle that can
/// be drawn within the polygon

    #[lua(kind = "Method")]
    fn inradius(&self) -> f32;

"#,
    r#"
/// Get the length of one side of the regular polygon

    #[lua(kind = "Method")]
    fn side_length(&self) -> f32;

"#,
    r#"
/// Get the area of the regular polygon

    #[lua(kind = "Method")]
    fn area(&self) -> f32;

"#,
    r#"
/// Get the perimeter of the regular polygon.
/// This is the sum of its sides

    #[lua(kind = "Method")]
    fn perimeter(&self) -> f32;

"#,
    r#"
/// Get the internal angle of the regular polygon in degrees.
/// This is the angle formed by two adjacent sides with points
/// within the angle being in the interior of the polygon

    #[lua(kind = "Method")]
    fn internal_angle_degrees(&self) -> f32;

"#,
    r#"
/// Get the internal angle of the regular polygon in radians.
/// This is the angle formed by two adjacent sides with points
/// within the angle being in the interior of the polygon

    #[lua(kind = "Method")]
    fn internal_angle_radians(&self) -> f32;

"#,
    r#"
/// Get the external angle of the regular polygon in degrees.
/// This is the angle formed by two adjacent sides with points
/// within the angle being in the exterior of the polygon

    #[lua(kind = "Method")]
    fn external_angle_degrees(&self) -> f32;

"#,
    r#"
/// Get the external angle of the regular polygon in radians.
/// This is the angle formed by two adjacent sides with points
/// within the angle being in the exterior of the polygon

    #[lua(kind = "Method")]
    fn external_angle_radians(&self) -> f32;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::RegularPolygon) -> bool;

"#]
)]
pub struct RegularPolygon {
    #[lua(output(proxy))]
    circumcircle: bevy::math::primitives::Circle,
    sides: usize,
}
/// A 2D capsule primitive, also known as a stadium or pill shape.
/// A two-dimensional capsule is defined as a neighborhood of points at a distance (radius) from a line
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Capsule2d",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Capsule2d) -> bool;

"#,
    r#"
/// Create a new `Capsule2d` from a radius and length

    #[lua(kind = "Function", output(proxy))]
    fn new(radius: f32, length: f32) -> bevy::math::primitives::Capsule2d;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Capsule2d;

"#]
)]
pub struct Capsule2d {
    radius: f32,
    half_length: f32,
}
/// A normalized vector pointing in a direction in 3D space
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Direction3d",
    functions[r#"

    #[lua(
        as_trait = "std::ops::Mul",
        kind = "MetaFunction",
        output(proxy),
        composite = "mul",
        metamethod = "Mul",
    )]
    fn mul(self, rhs: f32) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Direction3d;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Direction3d) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> bevy::math::primitives::Direction3d;

"#,
    r#"
/// Create a [`Direction3d`] from a [`Vec3`] that is already normalized.
/// # Warning
/// `value` must be normalized, i.e it's length must be `1.0`.

    #[lua(kind = "Function", output(proxy))]
    fn new_unchecked(
        #[proxy]
        value: bevy::math::Vec3,
    ) -> bevy::math::primitives::Direction3d;

"#]
)]
pub struct Direction3d();
/// A sphere primitive
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Sphere",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Sphere;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Sphere) -> bool;

"#,
    r#"
/// Create a new [`Sphere`] from a `radius`

    #[lua(kind = "Function", output(proxy))]
    fn new(radius: f32) -> bevy::math::primitives::Sphere;

"#,
    r#"
/// Get the diameter of the sphere

    #[lua(kind = "Method")]
    fn diameter(&self) -> f32;

"#,
    r#"
/// Get the surface area of the sphere

    #[lua(kind = "Method")]
    fn area(&self) -> f32;

"#,
    r#"
/// Get the volume of the sphere

    #[lua(kind = "Method")]
    fn volume(&self) -> f32;

"#,
    r#"
/// Finds the point on the sphere that is closest to the given `point`.
/// If the point is outside the sphere, the returned point will be on the surface of the sphere.
/// Otherwise, it will be inside the sphere and returned as is.

    #[lua(kind = "Method", output(proxy))]
    fn closest_point(&self, #[proxy] point: bevy::math::Vec3) -> bevy::math::Vec3;

"#]
)]
pub struct Sphere {
    radius: f32,
}
/// An unbounded plane in 3D space. It forms a separating surface through the origin,
/// stretching infinitely far
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Plane3d",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Plane3d) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Plane3d;

"#,
    r#"
/// Create a new `Plane3d` from a normal
/// # Panics
/// Panics if the given `normal` is zero (or very close to zero), or non-finite.

    #[lua(kind = "Function", output(proxy))]
    fn new(#[proxy] normal: bevy::math::Vec3) -> bevy::math::primitives::Plane3d;

"#]
)]
pub struct Plane3d {
    #[lua(output(proxy))]
    normal: bevy::math::primitives::Direction3d,
}
/// An infinite line along a direction in 3D space.
/// For a finite line: [`Segment3d`]
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Line3d",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Line3d;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Line3d) -> bool;

"#]
)]
pub struct Line3d {
    #[lua(output(proxy))]
    direction: bevy::math::primitives::Direction3d,
}
/// A segment of a line along a direction in 3D space.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Segment3d",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Segment3d) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Segment3d;

"#,
    r#"
/// Create a new `Segment3d` from a direction and full length of the segment

    #[lua(kind = "Function", output(proxy))]
    fn new(
        #[proxy]
        direction: bevy::math::primitives::Direction3d,
        length: f32,
    ) -> bevy::math::primitives::Segment3d;

"#,
    r#"
/// Get the position of the first point on the line segment

    #[lua(kind = "Method", output(proxy))]
    fn point1(&self) -> bevy::math::Vec3;

"#,
    r#"
/// Get the position of the second point on the line segment

    #[lua(kind = "Method", output(proxy))]
    fn point2(&self) -> bevy::math::Vec3;

"#]
)]
pub struct Segment3d {
    #[lua(output(proxy))]
    direction: bevy::math::primitives::Direction3d,
    half_length: f32,
}
/// A cuboid primitive, more commonly known as a box.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Cuboid",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Cuboid;

"#,
    r#"
/// Create a new `Cuboid` from a full x, y, and z length

    #[lua(kind = "Function", output(proxy))]
    fn new(
        x_length: f32,
        y_length: f32,
        z_length: f32,
    ) -> bevy::math::primitives::Cuboid;

"#,
    r#"
/// Create a new `Cuboid` from a given full size

    #[lua(kind = "Function", output(proxy))]
    fn from_size(#[proxy] size: bevy::math::Vec3) -> bevy::math::primitives::Cuboid;

"#,
    r#"
/// Create a new `Cuboid` from two corner points

    #[lua(kind = "Function", output(proxy))]
    fn from_corners(
        #[proxy]
        point1: bevy::math::Vec3,
        #[proxy]
        point2: bevy::math::Vec3,
    ) -> bevy::math::primitives::Cuboid;

"#,
    r#"
/// Get the size of the cuboid

    #[lua(kind = "Method", output(proxy))]
    fn size(&self) -> bevy::math::Vec3;

"#,
    r#"
/// Get the surface area of the cuboid

    #[lua(kind = "Method")]
    fn area(&self) -> f32;

"#,
    r#"
/// Get the volume of the cuboid

    #[lua(kind = "Method")]
    fn volume(&self) -> f32;

"#,
    r#"
/// Finds the point on the cuboid that is closest to the given `point`.
/// If the point is outside the cuboid, the returned point will be on the surface of the cuboid.
/// Otherwise, it will be inside the cuboid and returned as is.

    #[lua(kind = "Method", output(proxy))]
    fn closest_point(&self, #[proxy] point: bevy::math::Vec3) -> bevy::math::Vec3;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Cuboid) -> bool;

"#]
)]
pub struct Cuboid {
    #[lua(output(proxy))]
    half_size: bevy::math::Vec3,
}
/// A cylinder primitive
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Cylinder",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Cylinder;

"#,
    r#"
/// Create a new `Cylinder` from a radius and full height

    #[lua(kind = "Function", output(proxy))]
    fn new(radius: f32, height: f32) -> bevy::math::primitives::Cylinder;

"#,
    r#"
/// Get the base of the cylinder as a [`Circle`]

    #[lua(kind = "Method", output(proxy))]
    fn base(&self) -> bevy::math::primitives::Circle;

"#,
    r#"
/// Get the surface area of the side of the cylinder,
/// also known as the lateral area

    #[lua(kind = "Method")]
    fn lateral_area(&self) -> f32;

"#,
    r#"
/// Get the surface area of one base of the cylinder

    #[lua(kind = "Method")]
    fn base_area(&self) -> f32;

"#,
    r#"
/// Get the total surface area of the cylinder

    #[lua(kind = "Method")]
    fn area(&self) -> f32;

"#,
    r#"
/// Get the volume of the cylinder

    #[lua(kind = "Method")]
    fn volume(&self) -> f32;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Cylinder) -> bool;

"#]
)]
pub struct Cylinder {
    radius: f32,
    half_height: f32,
}
/// A 3D capsule primitive.
/// A three-dimensional capsule is defined as a surface at a distance (radius) from a line
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Capsule3d",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Capsule3d;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Capsule3d) -> bool;

"#,
    r#"
/// Create a new `Capsule3d` from a radius and length

    #[lua(kind = "Function", output(proxy))]
    fn new(radius: f32, length: f32) -> bevy::math::primitives::Capsule3d;

"#,
    r#"
/// Get the part connecting the hemispherical ends
/// of the capsule as a [`Cylinder`]

    #[lua(kind = "Method", output(proxy))]
    fn to_cylinder(&self) -> bevy::math::primitives::Cylinder;

"#,
    r#"
/// Get the surface area of the capsule

    #[lua(kind = "Method")]
    fn area(&self) -> f32;

"#,
    r#"
/// Get the volume of the capsule

    #[lua(kind = "Method")]
    fn volume(&self) -> f32;

"#]
)]
pub struct Capsule3d {
    radius: f32,
    half_length: f32,
}
/// A cone primitive.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Cone",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Cone) -> bool;

"#,
    r#"
/// Get the base of the cone as a [`Circle`]

    #[lua(kind = "Method", output(proxy))]
    fn base(&self) -> bevy::math::primitives::Circle;

"#,
    r#"
/// Get the slant height of the cone, the length of the line segment
/// connecting a point on the base to the apex

    #[lua(kind = "Method")]
    fn slant_height(&self) -> f32;

"#,
    r#"
/// Get the surface area of the side of the cone,
/// also known as the lateral area

    #[lua(kind = "Method")]
    fn lateral_area(&self) -> f32;

"#,
    r#"
/// Get the surface area of the base of the cone

    #[lua(kind = "Method")]
    fn base_area(&self) -> f32;

"#,
    r#"
/// Get the total surface area of the cone

    #[lua(kind = "Method")]
    fn area(&self) -> f32;

"#,
    r#"
/// Get the volume of the cone

    #[lua(kind = "Method")]
    fn volume(&self) -> f32;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Cone;

"#]
)]
pub struct Cone {
    radius: f32,
    height: f32,
}
/// A conical frustum primitive.
/// A conical frustum can be created
/// by slicing off a section of a cone.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::ConicalFrustum",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::ConicalFrustum) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::ConicalFrustum;

"#]
)]
pub struct ConicalFrustum {
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
}
/// A torus primitive, often representing a ring or donut shape
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::primitives::Torus",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::primitives::Torus) -> bool;

"#,
    r#"
/// Create a new `Torus` from an inner and outer radius.
/// The inner radius is the radius of the hole, and the outer radius
/// is the radius of the entire object

    #[lua(kind = "Function", output(proxy))]
    fn new(inner_radius: f32, outer_radius: f32) -> bevy::math::primitives::Torus;

"#,
    r#"
/// Get the inner radius of the torus.
/// For a ring torus, this corresponds to the radius of the hole,
/// or `major_radius - minor_radius`

    #[lua(kind = "Method")]
    fn inner_radius(&self) -> f32;

"#,
    r#"
/// Get the outer radius of the torus.
/// This corresponds to the overall radius of the entire object,
/// or `major_radius + minor_radius`

    #[lua(kind = "Method")]
    fn outer_radius(&self) -> f32;

"#,
    r#"
/// Get the surface area of the torus. Note that this only produces
/// the expected result when the torus has a ring and isn't self-intersecting

    #[lua(kind = "Method")]
    fn area(&self) -> f32;

"#,
    r#"
/// Get the volume of the torus. Note that this only produces
/// the expected result when the torus has a ring and isn't self-intersecting

    #[lua(kind = "Method")]
    fn volume(&self) -> f32;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::primitives::Torus;

"#]
)]
pub struct Torus {
    minor_radius: f32,
    major_radius: f32,
}
/// A rectangle defined by two opposite corners.
/// The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
/// stored in `IRect::min` and `IRect::max`, respectively. The minimum/maximum invariant
/// must be upheld by the user when directly assigning the fields, otherwise some methods
/// produce invalid results. It is generally recommended to use one of the constructor
/// methods instead, which will ensure this invariant is met, unless you already have
/// the minimum and maximum corners.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::IRect",
    functions[r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::IRect;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::IRect) -> bool;

"#,
    r#"
/// Create a new rectangle from two corner points.
/// The two points do not need to be the minimum and/or maximum corners.
/// They only need to be two opposite corners.
/// # Examples
/// ```
/// # use bevy_math::IRect;
/// let r = IRect::new(0, 4, 10, 6); // w=10 h=2
/// let r = IRect::new(2, 3, 5, -1); // w=3 h=4
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> bevy::math::IRect;

"#,
    r#"
/// Create a new rectangle from two corner points.
/// The two points do not need to be the minimum and/or maximum corners.
/// They only need to be two opposite corners.
/// # Examples
/// ```
/// # use bevy_math::{IRect, IVec2};
/// // Unit rect from [0,0] to [1,1]
/// let r = IRect::from_corners(IVec2::ZERO, IVec2::ONE); // w=1 h=1
/// // Same; the points do not need to be ordered
/// let r = IRect::from_corners(IVec2::ONE, IVec2::ZERO); // w=1 h=1
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_corners(
        #[proxy]
        p0: bevy::math::IVec2,
        #[proxy]
        p1: bevy::math::IVec2,
    ) -> bevy::math::IRect;

"#,
    r#"
/// Create a new rectangle from its center and size.
/// # Rounding Behaviour
/// If the size contains odd numbers they will be rounded down to the nearest whole number.
/// # Panics
/// This method panics if any of the components of the size is negative.
/// # Examples
/// ```
/// # use bevy_math::{IRect, IVec2};
/// let r = IRect::from_center_size(IVec2::ZERO, IVec2::new(3, 2)); // w=2 h=2
/// assert_eq!(r.min, IVec2::splat(-1));
/// assert_eq!(r.max, IVec2::splat(1));
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_center_size(
        #[proxy]
        origin: bevy::math::IVec2,
        #[proxy]
        size: bevy::math::IVec2,
    ) -> bevy::math::IRect;

"#,
    r#"
/// Create a new rectangle from its center and half-size.
/// # Panics
/// This method panics if any of the components of the half-size is negative.
/// # Examples
/// ```
/// # use bevy_math::{IRect, IVec2};
/// let r = IRect::from_center_half_size(IVec2::ZERO, IVec2::ONE); // w=2 h=2
/// assert_eq!(r.min, IVec2::splat(-1));
/// assert_eq!(r.max, IVec2::splat(1));
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_center_half_size(
        #[proxy]
        origin: bevy::math::IVec2,
        #[proxy]
        half_size: bevy::math::IVec2,
    ) -> bevy::math::IRect;

"#,
    r#"
/// Check if the rectangle is empty.
/// # Examples
/// ```
/// # use bevy_math::{IRect, IVec2};
/// let r = IRect::from_corners(IVec2::ZERO, IVec2::new(0, 1)); // w=0 h=1
/// assert!(r.is_empty());
/// ```

    #[lua(kind = "Method")]
    fn is_empty(&self) -> bool;

"#,
    r#"
/// Rectangle width (max.x - min.x).
/// # Examples
/// ```
/// # use bevy_math::IRect;
/// let r = IRect::new(0, 0, 5, 1); // w=5 h=1
/// assert_eq!(r.width(), 5);
/// ```

    #[lua(kind = "Method")]
    fn width(&self) -> i32;

"#,
    r#"
/// Rectangle height (max.y - min.y).
/// # Examples
/// ```
/// # use bevy_math::IRect;
/// let r = IRect::new(0, 0, 5, 1); // w=5 h=1
/// assert_eq!(r.height(), 1);
/// ```

    #[lua(kind = "Method")]
    fn height(&self) -> i32;

"#,
    r#"
/// Rectangle size.
/// # Examples
/// ```
/// # use bevy_math::{IRect, IVec2};
/// let r = IRect::new(0, 0, 5, 1); // w=5 h=1
/// assert_eq!(r.size(), IVec2::new(5, 1));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn size(&self) -> bevy::math::IVec2;

"#,
    r#"
/// Rectangle half-size.
/// # Rounding Behaviour
/// If the full size contains odd numbers they will be rounded down to the nearest whole number when calculating the half size.
/// # Examples
/// ```
/// # use bevy_math::{IRect, IVec2};
/// let r = IRect::new(0, 0, 4, 3); // w=4 h=3
/// assert_eq!(r.half_size(), IVec2::new(2, 1));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn half_size(&self) -> bevy::math::IVec2;

"#,
    r#"
/// The center point of the rectangle.
/// # Rounding Behaviour
/// If the (min + max) contains odd numbers they will be rounded down to the nearest whole number when calculating the center.
/// # Examples
/// ```
/// # use bevy_math::{IRect, IVec2};
/// let r = IRect::new(0, 0, 5, 2); // w=5 h=2
/// assert_eq!(r.center(), IVec2::new(2, 1));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn center(&self) -> bevy::math::IVec2;

"#,
    r#"
/// Check if a point lies within this rectangle, inclusive of its edges.
/// # Examples
/// ```
/// # use bevy_math::IRect;
/// let r = IRect::new(0, 0, 5, 1); // w=5 h=1
/// assert!(r.contains(r.center()));
/// assert!(r.contains(r.min));
/// assert!(r.contains(r.max));
/// ```

    #[lua(kind = "Method")]
    fn contains(&self, #[proxy] point: bevy::math::IVec2) -> bool;

"#,
    r#"
/// Build a new rectangle formed of the union of this rectangle and another rectangle.
/// The union is the smallest rectangle enclosing both rectangles.
/// # Examples
/// ```
/// # use bevy_math::{IRect, IVec2};
/// let r1 = IRect::new(0, 0, 5, 1); // w=5 h=1
/// let r2 = IRect::new(1, -1, 3, 3); // w=2 h=4
/// let r = r1.union(r2);
/// assert_eq!(r.min, IVec2::new(0, -1));
/// assert_eq!(r.max, IVec2::new(5, 3));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn union(&self, #[proxy] other: bevy::math::IRect) -> bevy::math::IRect;

"#,
    r#"
/// Build a new rectangle formed of the union of this rectangle and a point.
/// The union is the smallest rectangle enclosing both the rectangle and the point. If the
/// point is already inside the rectangle, this method returns a copy of the rectangle.
/// # Examples
/// ```
/// # use bevy_math::{IRect, IVec2};
/// let r = IRect::new(0, 0, 5, 1); // w=5 h=1
/// let u = r.union_point(IVec2::new(3, 6));
/// assert_eq!(u.min, IVec2::ZERO);
/// assert_eq!(u.max, IVec2::new(5, 6));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn union_point(&self, #[proxy] other: bevy::math::IVec2) -> bevy::math::IRect;

"#,
    r#"
/// Build a new rectangle formed of the intersection of this rectangle and another rectangle.
/// The intersection is the largest rectangle enclosed in both rectangles. If the intersection
/// is empty, this method returns an empty rectangle ([`IRect::is_empty()`] returns `true`), but
/// the actual values of [`IRect::min`] and [`IRect::max`] are implementation-dependent.
/// # Examples
/// ```
/// # use bevy_math::{IRect, IVec2};
/// let r1 = IRect::new(0, 0, 5, 1); // w=5 h=1
/// let r2 = IRect::new(1, -1, 3, 3); // w=2 h=4
/// let r = r1.intersect(r2);
/// assert_eq!(r.min, IVec2::new(1, 0));
/// assert_eq!(r.max, IVec2::new(3, 1));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn intersect(&self, #[proxy] other: bevy::math::IRect) -> bevy::math::IRect;

"#,
    r#"
/// Create a new rectangle with a constant inset.
/// The inset is the extra border on all sides. A positive inset produces a larger rectangle,
/// while a negative inset is allowed and produces a smaller rectangle. If the inset is negative
/// and its absolute value is larger than the rectangle half-size, the created rectangle is empty.
/// # Examples
/// ```
/// # use bevy_math::{IRect, IVec2};
/// let r = IRect::new(0, 0, 5, 1); // w=5 h=1
/// let r2 = r.inset(3); // w=11 h=7
/// assert_eq!(r2.min, IVec2::splat(-3));
/// assert_eq!(r2.max, IVec2::new(8, 4));
/// let r = IRect::new(0, -1, 4, 3); // w=4 h=4
/// let r2 = r.inset(-1); // w=2 h=2
/// assert_eq!(r2.min, IVec2::new(1, 0));
/// assert_eq!(r2.max, IVec2::new(3, 2));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn inset(&self, inset: i32) -> bevy::math::IRect;

"#,
    r#"
/// Returns self as [`Rect`] (f32)

    #[lua(kind = "Method", output(proxy))]
    fn as_rect(&self) -> bevy::math::Rect;

"#,
    r#"
/// Returns self as [`URect`] (u32)

    #[lua(kind = "Method", output(proxy))]
    fn as_urect(&self) -> bevy::math::URect;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#]
)]
pub struct IRect {
    #[lua(output(proxy))]
    min: bevy::math::IVec2,
    #[lua(output(proxy))]
    max: bevy::math::IVec2,
}
/// A rectangle defined by two opposite corners.
/// The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
/// stored in `Rect::min` and `Rect::max`, respectively. The minimum/maximum invariant
/// must be upheld by the user when directly assigning the fields, otherwise some methods
/// produce invalid results. It is generally recommended to use one of the constructor
/// methods instead, which will ensure this invariant is met, unless you already have
/// the minimum and maximum corners.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Rect",
    functions[r#"
/// Create a new rectangle from two corner points.
/// The two points do not need to be the minimum and/or maximum corners.
/// They only need to be two opposite corners.
/// # Examples
/// ```
/// # use bevy_math::Rect;
/// let r = Rect::new(0., 4., 10., 6.); // w=10 h=2
/// let r = Rect::new(2., 3., 5., -1.); // w=3 h=4
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> bevy::math::Rect;

"#,
    r#"
/// Create a new rectangle from two corner points.
/// The two points do not need to be the minimum and/or maximum corners.
/// They only need to be two opposite corners.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// // Unit rect from [0,0] to [1,1]
/// let r = Rect::from_corners(Vec2::ZERO, Vec2::ONE); // w=1 h=1
/// // Same; the points do not need to be ordered
/// let r = Rect::from_corners(Vec2::ONE, Vec2::ZERO); // w=1 h=1
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_corners(
        #[proxy]
        p0: bevy::math::Vec2,
        #[proxy]
        p1: bevy::math::Vec2,
    ) -> bevy::math::Rect;

"#,
    r#"
/// Create a new rectangle from its center and size.
/// # Panics
/// This method panics if any of the components of the size is negative.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// let r = Rect::from_center_size(Vec2::ZERO, Vec2::ONE); // w=1 h=1
/// assert!(r.min.abs_diff_eq(Vec2::splat(-0.5), 1e-5));
/// assert!(r.max.abs_diff_eq(Vec2::splat(0.5), 1e-5));
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_center_size(
        #[proxy]
        origin: bevy::math::Vec2,
        #[proxy]
        size: bevy::math::Vec2,
    ) -> bevy::math::Rect;

"#,
    r#"
/// Create a new rectangle from its center and half-size.
/// # Panics
/// This method panics if any of the components of the half-size is negative.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// let r = Rect::from_center_half_size(Vec2::ZERO, Vec2::ONE); // w=2 h=2
/// assert!(r.min.abs_diff_eq(Vec2::splat(-1.), 1e-5));
/// assert!(r.max.abs_diff_eq(Vec2::splat(1.), 1e-5));
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_center_half_size(
        #[proxy]
        origin: bevy::math::Vec2,
        #[proxy]
        half_size: bevy::math::Vec2,
    ) -> bevy::math::Rect;

"#,
    r#"
/// Check if the rectangle is empty.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// let r = Rect::from_corners(Vec2::ZERO, Vec2::new(0., 1.)); // w=0 h=1
/// assert!(r.is_empty());
/// ```

    #[lua(kind = "Method")]
    fn is_empty(&self) -> bool;

"#,
    r#"
/// Rectangle width (max.x - min.x).
/// # Examples
/// ```
/// # use bevy_math::Rect;
/// let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
/// assert!((r.width() - 5.).abs() <= 1e-5);
/// ```

    #[lua(kind = "Method")]
    fn width(&self) -> f32;

"#,
    r#"
/// Rectangle height (max.y - min.y).
/// # Examples
/// ```
/// # use bevy_math::Rect;
/// let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
/// assert!((r.height() - 1.).abs() <= 1e-5);
/// ```

    #[lua(kind = "Method")]
    fn height(&self) -> f32;

"#,
    r#"
/// Rectangle size.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
/// assert!(r.size().abs_diff_eq(Vec2::new(5., 1.), 1e-5));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn size(&self) -> bevy::math::Vec2;

"#,
    r#"
/// Rectangle half-size.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
/// assert!(r.half_size().abs_diff_eq(Vec2::new(2.5, 0.5), 1e-5));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn half_size(&self) -> bevy::math::Vec2;

"#,
    r#"
/// The center point of the rectangle.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
/// assert!(r.center().abs_diff_eq(Vec2::new(2.5, 0.5), 1e-5));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn center(&self) -> bevy::math::Vec2;

"#,
    r#"
/// Check if a point lies within this rectangle, inclusive of its edges.
/// # Examples
/// ```
/// # use bevy_math::Rect;
/// let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
/// assert!(r.contains(r.center()));
/// assert!(r.contains(r.min));
/// assert!(r.contains(r.max));
/// ```

    #[lua(kind = "Method")]
    fn contains(&self, #[proxy] point: bevy::math::Vec2) -> bool;

"#,
    r#"
/// Build a new rectangle formed of the union of this rectangle and another rectangle.
/// The union is the smallest rectangle enclosing both rectangles.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// let r1 = Rect::new(0., 0., 5., 1.); // w=5 h=1
/// let r2 = Rect::new(1., -1., 3., 3.); // w=2 h=4
/// let r = r1.union(r2);
/// assert!(r.min.abs_diff_eq(Vec2::new(0., -1.), 1e-5));
/// assert!(r.max.abs_diff_eq(Vec2::new(5., 3.), 1e-5));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn union(&self, #[proxy] other: bevy::math::Rect) -> bevy::math::Rect;

"#,
    r#"
/// Build a new rectangle formed of the union of this rectangle and a point.
/// The union is the smallest rectangle enclosing both the rectangle and the point. If the
/// point is already inside the rectangle, this method returns a copy of the rectangle.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
/// let u = r.union_point(Vec2::new(3., 6.));
/// assert!(u.min.abs_diff_eq(Vec2::ZERO, 1e-5));
/// assert!(u.max.abs_diff_eq(Vec2::new(5., 6.), 1e-5));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn union_point(&self, #[proxy] other: bevy::math::Vec2) -> bevy::math::Rect;

"#,
    r#"
/// Build a new rectangle formed of the intersection of this rectangle and another rectangle.
/// The intersection is the largest rectangle enclosed in both rectangles. If the intersection
/// is empty, this method returns an empty rectangle ([`Rect::is_empty()`] returns `true`), but
/// the actual values of [`Rect::min`] and [`Rect::max`] are implementation-dependent.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// let r1 = Rect::new(0., 0., 5., 1.); // w=5 h=1
/// let r2 = Rect::new(1., -1., 3., 3.); // w=2 h=4
/// let r = r1.intersect(r2);
/// assert!(r.min.abs_diff_eq(Vec2::new(1., 0.), 1e-5));
/// assert!(r.max.abs_diff_eq(Vec2::new(3., 1.), 1e-5));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn intersect(&self, #[proxy] other: bevy::math::Rect) -> bevy::math::Rect;

"#,
    r#"
/// Create a new rectangle with a constant inset.
/// The inset is the extra border on all sides. A positive inset produces a larger rectangle,
/// while a negative inset is allowed and produces a smaller rectangle. If the inset is negative
/// and its absolute value is larger than the rectangle half-size, the created rectangle is empty.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// let r = Rect::new(0., 0., 5., 1.); // w=5 h=1
/// let r2 = r.inset(3.); // w=11 h=7
/// assert!(r2.min.abs_diff_eq(Vec2::splat(-3.), 1e-5));
/// assert!(r2.max.abs_diff_eq(Vec2::new(8., 4.), 1e-5));
/// let r = Rect::new(0., -1., 6., 7.); // w=6 h=8
/// let r2 = r.inset(-2.); // w=11 h=7
/// assert!(r2.min.abs_diff_eq(Vec2::new(2., 1.), 1e-5));
/// assert!(r2.max.abs_diff_eq(Vec2::new(4., 5.), 1e-5));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn inset(&self, inset: f32) -> bevy::math::Rect;

"#,
    r#"
/// Build a new rectangle from this one with its coordinates expressed
/// relative to `other` in a normalized ([0..1] x [0..1]) coordinate system.
/// # Examples
/// ```
/// # use bevy_math::{Rect, Vec2};
/// let r = Rect::new(2., 3., 4., 6.);
/// let s = Rect::new(0., 0., 10., 10.);
/// let n = r.normalize(s);
/// assert_eq!(n.min.x, 0.2);
/// assert_eq!(n.min.y, 0.3);
/// assert_eq!(n.max.x, 0.4);
/// assert_eq!(n.max.y, 0.6);
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn normalize(&self, #[proxy] other: bevy::math::Rect) -> bevy::math::Rect;

"#,
    r#"
/// Returns self as [`IRect`] (i32)

    #[lua(kind = "Method", output(proxy))]
    fn as_irect(&self) -> bevy::math::IRect;

"#,
    r#"
/// Returns self as [`URect`] (u32)

    #[lua(kind = "Method", output(proxy))]
    fn as_urect(&self) -> bevy::math::URect;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::Rect;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::Rect) -> bool;

"#]
)]
pub struct Rect {
    #[lua(output(proxy))]
    min: bevy::math::Vec2,
    #[lua(output(proxy))]
    max: bevy::math::Vec2,
}
/// A rectangle defined by two opposite corners.
/// The rectangle is axis aligned, and defined by its minimum and maximum coordinates,
/// stored in `URect::min` and `URect::max`, respectively. The minimum/maximum invariant
/// must be upheld by the user when directly assigning the fields, otherwise some methods
/// produce invalid results. It is generally recommended to use one of the constructor
/// methods instead, which will ensure this invariant is met, unless you already have
/// the minimum and maximum corners.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::URect",
    functions[r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_math::URect) -> bool;

"#,
    r#"
/// Create a new rectangle from two corner points.
/// The two points do not need to be the minimum and/or maximum corners.
/// They only need to be two opposite corners.
/// # Examples
/// ```
/// # use bevy_math::URect;
/// let r = URect::new(0, 4, 10, 6); // w=10 h=2
/// let r = URect::new(2, 4, 5, 0); // w=3 h=4
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn new(x0: u32, y0: u32, x1: u32, y1: u32) -> bevy::math::URect;

"#,
    r#"
/// Create a new rectangle from two corner points.
/// The two points do not need to be the minimum and/or maximum corners.
/// They only need to be two opposite corners.
/// # Examples
/// ```
/// # use bevy_math::{URect, UVec2};
/// // Unit rect from [0,0] to [1,1]
/// let r = URect::from_corners(UVec2::ZERO, UVec2::ONE); // w=1 h=1
/// // Same; the points do not need to be ordered
/// let r = URect::from_corners(UVec2::ONE, UVec2::ZERO); // w=1 h=1
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_corners(
        #[proxy]
        p0: bevy::math::UVec2,
        #[proxy]
        p1: bevy::math::UVec2,
    ) -> bevy::math::URect;

"#,
    r#"
/// Create a new rectangle from its center and size.
/// # Rounding Behaviour
/// If the size contains odd numbers they will be rounded down to the nearest whole number.
/// # Panics
/// This method panics if any of the components of the size is negative or if `origin - (size / 2)` results in any negatives.
/// # Examples
/// ```
/// # use bevy_math::{URect, UVec2};
/// let r = URect::from_center_size(UVec2::ONE, UVec2::splat(2)); // w=2 h=2
/// assert_eq!(r.min, UVec2::splat(0));
/// assert_eq!(r.max, UVec2::splat(2));
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_center_size(
        #[proxy]
        origin: bevy::math::UVec2,
        #[proxy]
        size: bevy::math::UVec2,
    ) -> bevy::math::URect;

"#,
    r#"
/// Create a new rectangle from its center and half-size.
/// # Panics
/// This method panics if any of the components of the half-size is negative or if `origin - half_size` results in any negatives.
/// # Examples
/// ```
/// # use bevy_math::{URect, UVec2};
/// let r = URect::from_center_half_size(UVec2::ONE, UVec2::ONE); // w=2 h=2
/// assert_eq!(r.min, UVec2::splat(0));
/// assert_eq!(r.max, UVec2::splat(2));
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_center_half_size(
        #[proxy]
        origin: bevy::math::UVec2,
        #[proxy]
        half_size: bevy::math::UVec2,
    ) -> bevy::math::URect;

"#,
    r#"
/// Check if the rectangle is empty.
/// # Examples
/// ```
/// # use bevy_math::{URect, UVec2};
/// let r = URect::from_corners(UVec2::ZERO, UVec2::new(0, 1)); // w=0 h=1
/// assert!(r.is_empty());
/// ```

    #[lua(kind = "Method")]
    fn is_empty(&self) -> bool;

"#,
    r#"
/// Rectangle width (max.x - min.x).
/// # Examples
/// ```
/// # use bevy_math::URect;
/// let r = URect::new(0, 0, 5, 1); // w=5 h=1
/// assert_eq!(r.width(), 5);
/// ```

    #[lua(kind = "Method")]
    fn width(&self) -> u32;

"#,
    r#"
/// Rectangle height (max.y - min.y).
/// # Examples
/// ```
/// # use bevy_math::URect;
/// let r = URect::new(0, 0, 5, 1); // w=5 h=1
/// assert_eq!(r.height(), 1);
/// ```

    #[lua(kind = "Method")]
    fn height(&self) -> u32;

"#,
    r#"
/// Rectangle size.
/// # Examples
/// ```
/// # use bevy_math::{URect, UVec2};
/// let r = URect::new(0, 0, 5, 1); // w=5 h=1
/// assert_eq!(r.size(), UVec2::new(5, 1));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn size(&self) -> bevy::math::UVec2;

"#,
    r#"
/// Rectangle half-size.
/// # Rounding Behaviour
/// If the full size contains odd numbers they will be rounded down to the nearest whole number when calculating the half size.
/// # Examples
/// ```
/// # use bevy_math::{URect, UVec2};
/// let r = URect::new(0, 0, 4, 2); // w=4 h=2
/// assert_eq!(r.half_size(), UVec2::new(2, 1));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn half_size(&self) -> bevy::math::UVec2;

"#,
    r#"
/// The center point of the rectangle.
/// # Rounding Behaviour
/// If the (min + max) contains odd numbers they will be rounded down to the nearest whole number when calculating the center.
/// # Examples
/// ```
/// # use bevy_math::{URect, UVec2};
/// let r = URect::new(0, 0, 4, 2); // w=4 h=2
/// assert_eq!(r.center(), UVec2::new(2, 1));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn center(&self) -> bevy::math::UVec2;

"#,
    r#"
/// Check if a point lies within this rectangle, inclusive of its edges.
/// # Examples
/// ```
/// # use bevy_math::URect;
/// let r = URect::new(0, 0, 5, 1); // w=5 h=1
/// assert!(r.contains(r.center()));
/// assert!(r.contains(r.min));
/// assert!(r.contains(r.max));
/// ```

    #[lua(kind = "Method")]
    fn contains(&self, #[proxy] point: bevy::math::UVec2) -> bool;

"#,
    r#"
/// Build a new rectangle formed of the union of this rectangle and another rectangle.
/// The union is the smallest rectangle enclosing both rectangles.
/// # Examples
/// ```
/// # use bevy_math::{URect, UVec2};
/// let r1 = URect::new(0, 0, 5, 1); // w=5 h=1
/// let r2 = URect::new(1, 0, 3, 8); // w=2 h=4
/// let r = r1.union(r2);
/// assert_eq!(r.min, UVec2::new(0, 0));
/// assert_eq!(r.max, UVec2::new(5, 8));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn union(&self, #[proxy] other: bevy::math::URect) -> bevy::math::URect;

"#,
    r#"
/// Build a new rectangle formed of the union of this rectangle and a point.
/// The union is the smallest rectangle enclosing both the rectangle and the point. If the
/// point is already inside the rectangle, this method returns a copy of the rectangle.
/// # Examples
/// ```
/// # use bevy_math::{URect, UVec2};
/// let r = URect::new(0, 0, 5, 1); // w=5 h=1
/// let u = r.union_point(UVec2::new(3, 6));
/// assert_eq!(u.min, UVec2::ZERO);
/// assert_eq!(u.max, UVec2::new(5, 6));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn union_point(&self, #[proxy] other: bevy::math::UVec2) -> bevy::math::URect;

"#,
    r#"
/// Build a new rectangle formed of the intersection of this rectangle and another rectangle.
/// The intersection is the largest rectangle enclosed in both rectangles. If the intersection
/// is empty, this method returns an empty rectangle ([`URect::is_empty()`] returns `true`), but
/// the actual values of [`URect::min`] and [`URect::max`] are implementation-dependent.
/// # Examples
/// ```
/// # use bevy_math::{URect, UVec2};
/// let r1 = URect::new(0, 0, 2, 2); // w=2 h=2
/// let r2 = URect::new(1, 1, 3, 3); // w=2 h=2
/// let r = r1.intersect(r2);
/// assert_eq!(r.min, UVec2::new(1, 1));
/// assert_eq!(r.max, UVec2::new(2, 2));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn intersect(&self, #[proxy] other: bevy::math::URect) -> bevy::math::URect;

"#,
    r#"
/// Create a new rectangle with a constant inset.
/// The inset is the extra border on all sides. A positive inset produces a larger rectangle,
/// while a negative inset is allowed and produces a smaller rectangle. If the inset is negative
/// and its absolute value is larger than the rectangle half-size, the created rectangle is empty.
/// # Examples
/// ```
/// # use bevy_math::{URect, UVec2};
/// let r = URect::new(4, 4, 6, 6); // w=2 h=2
/// let r2 = r.inset(1); // w=4 h=4
/// assert_eq!(r2.min, UVec2::splat(3));
/// assert_eq!(r2.max, UVec2::splat(7));
/// let r = URect::new(4, 4, 8, 8); // w=4 h=4
/// let r2 = r.inset(-1); // w=2 h=2
/// assert_eq!(r2.min, UVec2::splat(5));
/// assert_eq!(r2.max, UVec2::splat(7));
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn inset(&self, inset: i32) -> bevy::math::URect;

"#,
    r#"
/// Returns self as [`Rect`] (f32)

    #[lua(kind = "Method", output(proxy))]
    fn as_rect(&self) -> bevy::math::Rect;

"#,
    r#"
/// Returns self as [`IRect`] (i32)

    #[lua(kind = "Method", output(proxy))]
    fn as_irect(&self) -> bevy::math::IRect;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::math::URect;

"#]
)]
pub struct URect {
    #[lua(output(proxy))]
    min: bevy::math::UVec2,
    #[lua(output(proxy))]
    max: bevy::math::UVec2,
}
/// A `SmolStr` is a string type that has the following properties:
/// * `size_of::<SmolStr>() == 24` (therefor `== size_of::<String>()` on 64 bit platforms)
/// * `Clone` is `O(1)`
/// * Strings are stack-allocated if they are:
///     * Up to 23 bytes long
///     * Longer than 23 bytes, but substrings of `WS` (see below). Such strings consist
///     solely of consecutive newlines, followed by consecutive spaces
/// * If a string does not satisfy the aforementioned conditions, it is heap-allocated
/// * Additionally, a `SmolStr` can be explicitely created from a `&'static str` without allocation
/// Unlike `String`, however, `SmolStr` is immutable. The primary use case for
/// `SmolStr` is a good enough default storage for tokens of typical programming
/// languages. Strings consisting of a series of newlines, followed by a series of
/// whitespace are a typical pattern in computer programs because of indentation.
/// Note that a specialized interner might be a better solution for some use cases.
/// `WS`: A string of 32 newlines followed by 128 spaces.
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "smol_str::SmolStr",
    functions[r#"

    #[lua(kind = "Method")]
    fn to_string(&self) -> std::string::String;

"#,
    r#"

    #[lua(kind = "Method")]
    fn len(&self) -> usize;

"#,
    r#"

    #[lua(kind = "Method")]
    fn is_empty(&self) -> bool;

"#,
    r#"

    #[lua(kind = "Method")]
    fn is_heap_allocated(&self) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &smol_str::SmolStr) -> bool;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> smol_str::SmolStr;

"#]
)]
pub struct SmolStr();
/// An integer that is known not to equal zero.
/// This enables some memory layout optimization.
///For example, `Option<NonZeroIsize>` is the same size as `isize`:
/// ```rust
/// use std::mem::size_of;
///assert_eq!(size_of::<Option<core::num::NonZeroIsize>>(), size_of::<isize>());
/// ```
/// # Layout
///`NonZeroIsize` is guaranteed to have the same layout and bit validity as `isize`
/// with the exception that `0` is not a valid instance.
///`Option<NonZeroIsize>` is guaranteed to be compatible with `isize`,
/// including in FFI.
/// Thanks to the [null pointer optimization],
///`NonZeroIsize` and `Option<NonZeroIsize>`
/// are guaranteed to have the same size and alignment:
/// ```
/// # use std::mem::{size_of, align_of};
///use std::num::NonZeroIsize;
///assert_eq!(size_of::<NonZeroIsize>(), size_of::<Option<NonZeroIsize>>());
///assert_eq!(align_of::<NonZeroIsize>(), align_of::<Option<NonZeroIsize>>());
/// ```
/// [null pointer optimization]: crate::option#representation
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::num::NonZeroIsize",
    functions[r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &std::num::NonZeroIsize) -> bool;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"
/// Creates a non-zero without checking whether the value is non-zero.
/// This results in undefined behaviour if the value is zero.
/// # Safety
/// The value must not be zero.

    #[lua(kind = "Function", output(proxy))]
    unsafe fn new_unchecked(n: isize) -> std::num::NonZeroIsize;

"#,
    r#"
/// Returns the value as a primitive type.

    #[lua(kind = "Method")]
    fn get(self) -> isize;

"#,
    r#"
/// Returns the number of leading zeros in the binary representation of `self`.
/// On many architectures, this function can perform better than `leading_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroIsize::new(-1isize).unwrap();
/// assert_eq!(n.leading_zeros(), 0);
/// ```

    #[lua(kind = "Method")]
    fn leading_zeros(self) -> u32;

"#,
    r#"
/// Returns the number of trailing zeros in the binary representation
/// of `self`.
/// On many architectures, this function can perform better than `trailing_zeros()` on the underlying integer type, as special handling of zero can be avoided.
/// # Examples
/// Basic usage:
/// ```
///let n = std::num::NonZeroIsize::new(0b0101000).unwrap();
/// assert_eq!(n.trailing_zeros(), 3);
/// ```

    #[lua(kind = "Method")]
    fn trailing_zeros(self) -> u32;

"#,
    r#"
/// Computes the absolute value of self.
///See [`isize::abs`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroIsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroIsize::new(1)?;
///let neg = NonZeroIsize::new(-1)?;
/// assert_eq!(pos, pos.abs());
/// assert_eq!(pos, neg.abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn abs(self) -> std::num::NonZeroIsize;

"#,
    r#"
/// Saturating absolute value, see
///[`isize::saturating_abs`].
/// # Example
/// ```
///# use std::num::NonZeroIsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroIsize::new(1)?;
///let neg = NonZeroIsize::new(-1)?;
///let min = NonZeroIsize::new(isize::MIN)?;
///let min_plus = NonZeroIsize::new(isize::MIN + 1)?;
///let max = NonZeroIsize::new(isize::MAX)?;
/// assert_eq!(pos, pos.saturating_abs());
/// assert_eq!(pos, neg.saturating_abs());
/// assert_eq!(max, min.saturating_abs());
/// assert_eq!(max, min_plus.saturating_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_abs(self) -> std::num::NonZeroIsize;

"#,
    r#"
/// Wrapping absolute value, see
///[`isize::wrapping_abs`].
/// # Example
/// ```
///# use std::num::NonZeroIsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos = NonZeroIsize::new(1)?;
///let neg = NonZeroIsize::new(-1)?;
///let min = NonZeroIsize::new(isize::MIN)?;
///# let max = NonZeroIsize::new(isize::MAX)?;
/// assert_eq!(pos, pos.wrapping_abs());
/// assert_eq!(pos, neg.wrapping_abs());
/// assert_eq!(min, min.wrapping_abs());
/// assert_eq!(max, (-max).wrapping_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_abs(self) -> std::num::NonZeroIsize;

"#,
    r#"
/// Computes the absolute value of self
/// without any wrapping or panicking.
/// # Example
/// ```
///# use std::num::NonZeroIsize;
///# use std::num::NonZeroUsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let u_pos = NonZeroUsize::new(1)?;
///let i_pos = NonZeroIsize::new(1)?;
///let i_neg = NonZeroIsize::new(-1)?;
///let i_min = NonZeroIsize::new(isize::MIN)?;
///let u_max = NonZeroUsize::new(usize::MAX / 2 + 1)?;
/// assert_eq!(u_pos, i_pos.unsigned_abs());
/// assert_eq!(u_pos, i_neg.unsigned_abs());
/// assert_eq!(u_max, i_min.unsigned_abs());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn unsigned_abs(self) -> std::num::NonZeroUsize;

"#,
    r#"
/// Returns `true` if `self` is positive and `false` if the
/// number is negative.
/// # Example
/// ```
///# use std::num::NonZeroIsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroIsize::new(5)?;
///let neg_five = NonZeroIsize::new(-5)?;
/// assert!(pos_five.is_positive());
/// assert!(!neg_five.is_positive());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_positive(self) -> bool;

"#,
    r#"
/// Returns `true` if `self` is negative and `false` if the
/// number is positive.
/// # Example
/// ```
///# use std::num::NonZeroIsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroIsize::new(5)?;
///let neg_five = NonZeroIsize::new(-5)?;
/// assert!(neg_five.is_negative());
/// assert!(!pos_five.is_negative());
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn is_negative(self) -> bool;

"#,
    r#"
/// Saturating negation. Computes `-self`,
///returning [`NonZeroIsize::MAX`]
///if `self == NonZeroIsize::MIN`
/// instead of overflowing.
/// # Example
/// ```
///# use std::num::NonZeroIsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroIsize::new(5)?;
///let neg_five = NonZeroIsize::new(-5)?;
///let min = NonZeroIsize::new(isize::MIN)?;
///let min_plus_one = NonZeroIsize::new(isize::MIN + 1)?;
///let max = NonZeroIsize::new(isize::MAX)?;
/// assert_eq!(pos_five.saturating_neg(), neg_five);
/// assert_eq!(min.saturating_neg(), max);
/// assert_eq!(max.saturating_neg(), min_plus_one);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_neg(self) -> std::num::NonZeroIsize;

"#,
    r#"
/// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary
/// of the type.
///See [`isize::wrapping_neg`]
/// for documentation on overflow behaviour.
/// # Example
/// ```
///# use std::num::NonZeroIsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let pos_five = NonZeroIsize::new(5)?;
///let neg_five = NonZeroIsize::new(-5)?;
///let min = NonZeroIsize::new(isize::MIN)?;
/// assert_eq!(pos_five.wrapping_neg(), neg_five);
/// assert_eq!(min.wrapping_neg(), min);
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn wrapping_neg(self) -> std::num::NonZeroIsize;

"#,
    r#"
/// Multiplies two non-zero integers together.
///Return [`NonZeroIsize::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroIsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let two = NonZeroIsize::new(2)?;
///let four = NonZeroIsize::new(4)?;
///let max = NonZeroIsize::new(isize::MAX)?;
/// assert_eq!(four, two.saturating_mul(two));
/// assert_eq!(max, four.saturating_mul(max));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_mul(
        self,
        #[proxy]
        other: std::num::NonZeroIsize,
    ) -> std::num::NonZeroIsize;

"#,
    r#"
/// Raise non-zero value to an integer power.
///Return [`NonZeroIsize::MIN`] or [`NonZeroIsize::MAX`] on overflow.
/// # Examples
/// ```
///# use std::num::NonZeroIsize;
/// # fn main() { test().unwrap(); }
/// # fn test() -> Option<()> {
///let three = NonZeroIsize::new(3)?;
///let twenty_seven = NonZeroIsize::new(27)?;
///let max = NonZeroIsize::new(isize::MAX)?;
/// assert_eq!(twenty_seven, three.saturating_pow(3));
/// assert_eq!(max, max.saturating_pow(3));
/// # Some(())
/// # }
/// ```

    #[lua(kind = "Method", output(proxy))]
    fn saturating_pow(self, other: u32) -> std::num::NonZeroIsize;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> std::num::NonZeroIsize;

"#,
    r#"

    #[lua(
        as_trait = "std::ops::Neg",
        kind = "MetaFunction",
        output(proxy),
        composite = "neg",
        metamethod = "Unm",
    )]
    fn neg(self) -> std::num::NonZeroIsize;

"#]
)]
pub struct NonZeroIsize();
/// A Universally Unique Identifier (UUID).
/// # Examples
/// Parse a UUID given in the simple format and print it as a urn:
/// ```
/// # use uuid::Uuid;
/// # fn main() -> Result<(), uuid::Error> {
/// let my_uuid = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")?;
/// println!("{}", my_uuid.urn());
/// # Ok(())
/// # }
/// ```
/// Create a new random (V4) UUID and print it out in hexadecimal form:
/// ```
/// // Note that this requires the `v4` feature enabled in the uuid crate.
/// # use uuid::Uuid;
/// # fn main() {
/// # #[cfg(feature = "v4")] {
/// let my_uuid = Uuid::new_v4();
/// println!("{}", my_uuid);
/// # }
/// # }
/// ```
/// # Formatting
/// A UUID can be formatted in one of a few ways:
/// * [`simple`](#method.simple): `a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8`.
/// * [`hyphenated`](#method.hyphenated):
///   `a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8`.
/// * [`urn`](#method.urn): `urn:uuid:A1A2A3A4-B1B2-C1C2-D1D2-D3D4D5D6D7D8`.
/// * [`braced`](#method.braced): `{a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8}`.
/// The default representation when formatting a UUID with `Display` is
/// hyphenated:
/// ```
/// # use uuid::Uuid;
/// # fn main() -> Result<(), uuid::Error> {
/// let my_uuid = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")?;
/// assert_eq!(
///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
///     my_uuid.to_string(),
/// );
/// # Ok(())
/// # }
/// ```
/// Other formats can be specified using adapter methods on the UUID:
/// ```
/// # use uuid::Uuid;
/// # fn main() -> Result<(), uuid::Error> {
/// let my_uuid = Uuid::parse_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8")?;
/// assert_eq!(
///     "urn:uuid:a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
///     my_uuid.urn().to_string(),
/// );
/// # Ok(())
/// # }
/// ```
/// # Endianness
/// The specification for UUIDs encodes the integer fields that make up the
/// value in big-endian order. This crate assumes integer inputs are already in
/// the correct order by default, regardless of the endianness of the
/// environment. Most methods that accept integers have a `_le` variant (such as
/// `from_fields_le`) that assumes any integer values will need to have their
/// bytes flipped, regardless of the endianness of the environment.
/// Most users won't need to worry about endianness unless they need to operate
/// on individual fields (such as when converting between Microsoft GUIDs). The
/// important things to remember are:
/// - The endianness is in terms of the fields of the UUID, not the environment.
/// - The endianness is assumed to be big-endian when there's no `_le` suffix
///   somewhere.
/// - Byte-flipping in `_le` methods applies to each integer.
/// - Endianness roundtrips, so if you create a UUID with `from_fields_le`
///   you'll get the same values back out with `to_fields_le`.
/// # ABI
/// The `Uuid` type is always guaranteed to be have the same ABI as [`Bytes`].
#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::utils::Uuid",
    functions[r#"
/// Returns the version number of the UUID.
/// This represents the algorithm used to generate the value.
/// This method is the future-proof alternative to [`Uuid::get_version`].
/// # Examples
/// Basic usage:
/// ```
/// # use uuid::Uuid;
/// # fn main() -> Result<(), uuid::Error> {
/// let my_uuid = Uuid::parse_str("02f09a3f-1624-3b1d-8409-44eff7708208")?;
/// assert_eq!(3, my_uuid.get_version_num());
/// # Ok(())
/// # }
/// ```
/// # References
/// * [Version in RFC4122](https://datatracker.ietf.org/doc/html/rfc4122#section-4.1.3)

    #[lua(kind = "Method")]
    fn get_version_num(&self) -> usize;

"#,
    r#"
/// Returns a 128bit value containing the value.
/// The bytes in the UUID will be packed directly into a `u128`.
/// # Examples
/// ```
/// # use uuid::Uuid;
/// # fn main() -> Result<(), uuid::Error> {
/// let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
/// assert_eq!(
///     uuid.as_u128(),
///     0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8,
/// );
/// # Ok(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn as_u128(&self) -> u128;

"#,
    r#"
/// Returns a 128bit little-endian value containing the value.
/// The bytes in the `u128` will be flipped to convert into big-endian
/// order. This is based on the endianness of the UUID, rather than the
/// target environment so bytes will be flipped on both big and little
/// endian machines.
/// Note that this will produce a different result than
/// [`Uuid::to_fields_le`], because the entire UUID is reversed, rather
/// than reversing the individual fields in-place.
/// # Examples
/// ```
/// # use uuid::Uuid;
/// # fn main() -> Result<(), uuid::Error> {
/// let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
/// assert_eq!(
///     uuid.to_u128_le(),
///     0xd8d7d6d5d4d3d2d1c2c1b2b1a4a3a2a1,
/// );
/// # Ok(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn to_u128_le(&self) -> u128;

"#,
    r#"
/// Returns two 64bit values containing the value.
/// The bytes in the UUID will be split into two `u64`.
/// The first u64 represents the 64 most significant bits,
/// the second one represents the 64 least significant.
/// # Examples
/// ```
/// # use uuid::Uuid;
/// # fn main() -> Result<(), uuid::Error> {
/// let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
/// assert_eq!(
///     uuid.as_u64_pair(),
///     (0xa1a2a3a4b1b2c1c2, 0xd1d2d3d4d5d6d7d8),
/// );
/// # Ok(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn as_u64_pair(&self) -> (u64, u64);

"#,
    r#"
/// Consumes self and returns the underlying byte value of the UUID.
/// # Examples
/// ```
/// # use uuid::Uuid;
/// let bytes = [
///     0xa1, 0xa2, 0xa3, 0xa4,
///     0xb1, 0xb2,
///     0xc1, 0xc2,
///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
/// ];
/// let uuid = Uuid::from_bytes(bytes);
/// assert_eq!(bytes, uuid.into_bytes());
/// ```

    #[lua(kind = "Method")]
    fn into_bytes(self) -> [u8; 16];

"#,
    r#"
/// Returns the bytes of the UUID in little-endian order.
/// The bytes will be flipped to convert into little-endian order. This is
/// based on the endianness of the UUID, rather than the target environment
/// so bytes will be flipped on both big and little endian machines.
/// # Examples
/// ```
/// use uuid::Uuid;
/// # fn main() -> Result<(), uuid::Error> {
/// let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
/// assert_eq!(
///     uuid.to_bytes_le(),
///     ([
///         0xa4, 0xa3, 0xa2, 0xa1, 0xb2, 0xb1, 0xc2, 0xc1, 0xd1, 0xd2,
///         0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8
///     ])
/// );
/// # Ok(())
/// # }
/// ```

    #[lua(kind = "Method")]
    fn to_bytes_le(&self) -> [u8; 16];

"#,
    r#"
/// Tests if the UUID is nil (all zeros).

    #[lua(kind = "Method")]
    fn is_nil(&self) -> bool;

"#,
    r#"
/// Tests if the UUID is max (all ones).

    #[lua(kind = "Method")]
    fn is_max(&self) -> bool;

"#,
    r#"
/// A buffer that can be used for `encode_...` calls, that is
/// guaranteed to be long enough for any of the format adapters.
/// # Examples
/// ```
/// # use uuid::Uuid;
/// let uuid = Uuid::nil();
/// assert_eq!(
///     uuid.simple().encode_lower(&mut Uuid::encode_buffer()),
///     "00000000000000000000000000000000"
/// );
/// assert_eq!(
///     uuid.hyphenated()
///         .encode_lower(&mut Uuid::encode_buffer()),
///     "00000000-0000-0000-0000-000000000000"
/// );
/// assert_eq!(
///     uuid.urn().encode_lower(&mut Uuid::encode_buffer()),
///     "urn:uuid:00000000-0000-0000-0000-000000000000"
/// );
/// ```

    #[lua(kind = "Function")]
    fn encode_buffer() -> [u8; 45];

"#,
    r#"
/// The 'nil UUID' (all zeros).
/// The nil UUID is a special form of UUID that is specified to have all
/// 128 bits set to zero.
/// # References
/// * [Nil UUID in RFC4122](https://tools.ietf.org/html/rfc4122.html#section-4.1.7)
/// # Examples
/// Basic usage:
/// ```
/// # use uuid::Uuid;
/// let uuid = Uuid::nil();
/// assert_eq!(
///     "00000000-0000-0000-0000-000000000000",
///     uuid.hyphenated().to_string(),
/// );
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn nil() -> bevy::utils::Uuid;

"#,
    r#"
/// The 'max UUID' (all ones).
/// The max UUID is a special form of UUID that is specified to have all
/// 128 bits set to one.
/// # References
/// * [Max UUID in Draft RFC: New UUID Formats, Version 4](https://datatracker.ietf.org/doc/html/draft-peabody-dispatch-new-uuid-format-04#section-5.4)
/// # Examples
/// Basic usage:
/// ```
/// # use uuid::Uuid;
/// let uuid = Uuid::max();
/// assert_eq!(
///     "ffffffff-ffff-ffff-ffff-ffffffffffff",
///     uuid.hyphenated().to_string(),
/// );
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn max() -> bevy::utils::Uuid;

"#,
    r#"
/// Creates a UUID from a 128bit value.
/// # Examples
/// Basic usage:
/// ```
/// # use uuid::Uuid;
/// let v = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128;
/// let uuid = Uuid::from_u128(v);
/// assert_eq!(
///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
///     uuid.hyphenated().to_string(),
/// );
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_u128(v: u128) -> bevy::utils::Uuid;

"#,
    r#"
/// Creates a UUID from a 128bit value in little-endian order.
/// The entire value will be flipped to convert into big-endian order.
/// This is based on the endianness of the UUID, rather than the target
/// environment so bytes will be flipped on both big and little endian
/// machines.
/// # Examples
/// Basic usage:
/// ```
/// # use uuid::Uuid;
/// let v = 0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128;
/// let uuid = Uuid::from_u128_le(v);
/// assert_eq!(
///     "d8d7d6d5-d4d3-d2d1-c2c1-b2b1a4a3a2a1",
///     uuid.hyphenated().to_string(),
/// );
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_u128_le(v: u128) -> bevy::utils::Uuid;

"#,
    r#"
/// Creates a UUID from two 64bit values.
/// # Examples
/// Basic usage:
/// ```
/// # use uuid::Uuid;
/// let hi = 0xa1a2a3a4b1b2c1c2u64;
/// let lo = 0xd1d2d3d4d5d6d7d8u64;
/// let uuid = Uuid::from_u64_pair(hi, lo);
/// assert_eq!(
///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8",
///     uuid.hyphenated().to_string(),
/// );
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_u64_pair(high_bits: u64, low_bits: u64) -> bevy::utils::Uuid;

"#,
    r#"
/// Creates a UUID using the supplied bytes.
/// # Examples
/// Basic usage:
/// ```
/// # fn main() -> Result<(), uuid::Error> {
/// # use uuid::Uuid;
/// let bytes = [
///     0xa1, 0xa2, 0xa3, 0xa4,
///     0xb1, 0xb2,
///     0xc1, 0xc2,
///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
/// ];
/// let uuid = Uuid::from_bytes(bytes);
/// assert_eq!(
///     uuid.hyphenated().to_string(),
///     "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8"
/// );
/// # Ok(())
/// # }
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_bytes(bytes: [u8; 16]) -> bevy::utils::Uuid;

"#,
    r#"
/// Creates a UUID using the supplied bytes in little endian order.
/// The individual fields encoded in the buffer will be flipped.
/// # Examples
/// Basic usage:
/// ```
/// # fn main() -> Result<(), uuid::Error> {
/// # use uuid::Uuid;
/// let bytes = [
///     0xa1, 0xa2, 0xa3, 0xa4,
///     0xb1, 0xb2,
///     0xc1, 0xc2,
///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
/// ];
/// let uuid = Uuid::from_bytes_le(bytes);
/// assert_eq!(
///     "a4a3a2a1-b2b1-c2c1-d1d2-d3d4d5d6d7d8",
///     uuid.hyphenated().to_string(),
/// );
/// # Ok(())
/// # }
/// ```

    #[lua(kind = "Function", output(proxy))]
    fn from_bytes_le(b: [u8; 16]) -> bevy::utils::Uuid;

"#,
    r#"

    #[lua(
        as_trait = "bevy::reflect::erased_serde::__private::serde::__private::Clone",
        kind = "Method",
        output(proxy),
    )]
    fn clone(&self) -> bevy::utils::Uuid;

"#,
    r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
    r#"
/// Creates a random UUID.
/// This uses the [`getrandom`] crate to utilise the operating system's RNG
/// as the source of random numbers. If you'd like to use a custom
/// generator, don't use this method: generate random bytes using your
/// custom generator and pass them to the
/// [`uuid::Builder::from_random_bytes`][from_random_bytes] function
/// instead.
/// Note that usage of this method requires the `v4` feature of this crate
/// to be enabled.
/// # Examples
/// Basic usage:
/// ```
/// # use uuid::{Uuid, Version};
/// let uuid = Uuid::new_v4();
/// assert_eq!(Some(Version::Random), uuid.get_version());
/// ```
/// # References
/// * [Version 4 UUIDs in RFC4122](https://www.rfc-editor.org/rfc/rfc4122#section-4.4)
/// [`getrandom`]: https://crates.io/crates/getrandom
/// [from_random_bytes]: struct.Builder.html#method.from_random_bytes

    #[lua(kind = "Function", output(proxy))]
    fn new_v4() -> bevy::utils::Uuid;

"#,
    r#"

    #[lua(
        as_trait = "std::cmp::PartialEq",
        kind = "MetaFunction",
        composite = "eq",
        metamethod = "Eq",
    )]
    fn eq(&self, #[proxy] other: &bevy_utils::Uuid) -> bool;

"#]
)]
pub struct Uuid();
#[derive(Default)]
pub(crate) struct Globals;
impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for Globals {
    fn add_instances<
        'lua,
        T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>,
    >(self, instances: &mut T) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()> {
        instances
            .add_instance(
                "Duration",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDuration>::new,
            )?;
        instances
            .add_instance(
                "Instant",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaInstant>::new,
            )?;
        instances
            .add_instance(
                "NonZeroI128",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroI128>::new,
            )?;
        instances
            .add_instance(
                "NonZeroI16",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroI16>::new,
            )?;
        instances
            .add_instance(
                "NonZeroI32",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroI32>::new,
            )?;
        instances
            .add_instance(
                "NonZeroI64",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroI64>::new,
            )?;
        instances
            .add_instance(
                "NonZeroI8",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroI8>::new,
            )?;
        instances
            .add_instance(
                "NonZeroU128",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroU128>::new,
            )?;
        instances
            .add_instance(
                "NonZeroU16",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroU16>::new,
            )?;
        instances
            .add_instance(
                "NonZeroU32",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroU32>::new,
            )?;
        instances
            .add_instance(
                "NonZeroU64",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroU64>::new,
            )?;
        instances
            .add_instance(
                "NonZeroU8",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroU8>::new,
            )?;
        instances
            .add_instance(
                "NonZeroUsize",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroUsize>::new,
            )?;
        instances
            .add_instance(
                "PathBuf",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaPathBuf>::new,
            )?;
        instances
            .add_instance(
                "Quat",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaQuat>::new,
            )?;
        instances
            .add_instance(
                "Vec3",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaVec3>::new,
            )?;
        instances
            .add_instance(
                "IVec2",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaIVec2>::new,
            )?;
        instances
            .add_instance(
                "IVec3",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaIVec3>::new,
            )?;
        instances
            .add_instance(
                "IVec4",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaIVec4>::new,
            )?;
        instances
            .add_instance(
                "I64Vec2",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaI64Vec2>::new,
            )?;
        instances
            .add_instance(
                "I64Vec3",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaI64Vec3>::new,
            )?;
        instances
            .add_instance(
                "I64Vec4",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaI64Vec4>::new,
            )?;
        instances
            .add_instance(
                "UVec2",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaUVec2>::new,
            )?;
        instances
            .add_instance(
                "UVec3",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaUVec3>::new,
            )?;
        instances
            .add_instance(
                "UVec4",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaUVec4>::new,
            )?;
        instances
            .add_instance(
                "U64Vec2",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaU64Vec2>::new,
            )?;
        instances
            .add_instance(
                "U64Vec3",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaU64Vec3>::new,
            )?;
        instances
            .add_instance(
                "U64Vec4",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaU64Vec4>::new,
            )?;
        instances
            .add_instance(
                "Vec2",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaVec2>::new,
            )?;
        instances
            .add_instance(
                "Vec3A",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaVec3A>::new,
            )?;
        instances
            .add_instance(
                "Vec4",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaVec4>::new,
            )?;
        instances
            .add_instance(
                "BVec2",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBVec2>::new,
            )?;
        instances
            .add_instance(
                "BVec3",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBVec3>::new,
            )?;
        instances
            .add_instance(
                "BVec4",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBVec4>::new,
            )?;
        instances
            .add_instance(
                "DVec2",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDVec2>::new,
            )?;
        instances
            .add_instance(
                "DVec3",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDVec3>::new,
            )?;
        instances
            .add_instance(
                "DVec4",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDVec4>::new,
            )?;
        instances
            .add_instance(
                "Mat2",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaMat2>::new,
            )?;
        instances
            .add_instance(
                "Mat3",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaMat3>::new,
            )?;
        instances
            .add_instance(
                "Mat3A",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaMat3A>::new,
            )?;
        instances
            .add_instance(
                "Mat4",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaMat4>::new,
            )?;
        instances
            .add_instance(
                "DMat2",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDMat2>::new,
            )?;
        instances
            .add_instance(
                "DMat3",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDMat3>::new,
            )?;
        instances
            .add_instance(
                "DMat4",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDMat4>::new,
            )?;
        instances
            .add_instance(
                "Affine2",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaAffine2>::new,
            )?;
        instances
            .add_instance(
                "Affine3A",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaAffine3A>::new,
            )?;
        instances
            .add_instance(
                "DAffine2",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDAffine2>::new,
            )?;
        instances
            .add_instance(
                "DAffine3",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDAffine3>::new,
            )?;
        instances
            .add_instance(
                "DQuat",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDQuat>::new,
            )?;
        instances
            .add_instance(
                "BVec3A",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBVec3A>::new,
            )?;
        instances
            .add_instance(
                "BVec4A",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBVec4A>::new,
            )?;
        instances
            .add_instance(
                "Direction2d",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDirection2d>::new,
            )?;
        instances
            .add_instance(
                "Circle",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaCircle>::new,
            )?;
        instances
            .add_instance(
                "Ellipse",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaEllipse>::new,
            )?;
        instances
            .add_instance(
                "Plane2d",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaPlane2d>::new,
            )?;
        instances
            .add_instance(
                "Segment2d",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaSegment2d>::new,
            )?;
        instances
            .add_instance(
                "Triangle2d",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaTriangle2d>::new,
            )?;
        instances
            .add_instance(
                "Rectangle",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaRectangle>::new,
            )?;
        instances
            .add_instance(
                "RegularPolygon",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<
                    LuaRegularPolygon,
                >::new,
            )?;
        instances
            .add_instance(
                "Capsule2d",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaCapsule2d>::new,
            )?;
        instances
            .add_instance(
                "Direction3d",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDirection3d>::new,
            )?;
        instances
            .add_instance(
                "Sphere",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaSphere>::new,
            )?;
        instances
            .add_instance(
                "Plane3d",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaPlane3d>::new,
            )?;
        instances
            .add_instance(
                "Segment3d",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaSegment3d>::new,
            )?;
        instances
            .add_instance(
                "Cuboid",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaCuboid>::new,
            )?;
        instances
            .add_instance(
                "Cylinder",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaCylinder>::new,
            )?;
        instances
            .add_instance(
                "Capsule3d",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaCapsule3d>::new,
            )?;
        instances
            .add_instance(
                "Torus",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaTorus>::new,
            )?;
        instances
            .add_instance(
                "IRect",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaIRect>::new,
            )?;
        instances
            .add_instance(
                "Rect",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaRect>::new,
            )?;
        instances
            .add_instance(
                "URect",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaURect>::new,
            )?;
        instances
            .add_instance(
                "NonZeroIsize",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNonZeroIsize>::new,
            )?;
        instances
            .add_instance(
                "Uuid",
                bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaUuid>::new,
            )?;
        Ok(())
    }
}
pub struct BevyReflectAPIProvider;
impl bevy_mod_scripting_core::hosts::APIProvider for BevyReflectAPIProvider {
    type APITarget = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type ScriptContext = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type DocTarget = bevy_mod_scripting_lua::docs::LuaDocFragment;
    fn attach_api(
        &mut self,
        ctx: &mut Self::APITarget,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        let ctx = ctx.get_mut().expect("Unable to acquire lock on Lua context");
        bevy_mod_scripting_lua::tealr::mlu::set_global_env(Globals, ctx)
            .map_err(|e| bevy_mod_scripting_core::error::ScriptError::Other(
                e.to_string(),
            ))
    }
    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(
            bevy_mod_scripting_lua::docs::LuaDocFragment::new(
                "BevyReflectAPI",
                |tw| {
                    tw.document_global_instance::<Globals>()
                        .expect("Something went wrong documenting globals")
                        .process_type::<LuaDuration>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaDuration,
                            >,
                        >()
                        .process_type::<LuaInstant>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaInstant>,
                        >()
                        .process_type::<LuaNonZeroI128>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroI128,
                            >,
                        >()
                        .process_type::<LuaNonZeroI16>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroI16,
                            >,
                        >()
                        .process_type::<LuaNonZeroI32>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroI32,
                            >,
                        >()
                        .process_type::<LuaNonZeroI64>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroI64,
                            >,
                        >()
                        .process_type::<LuaNonZeroI8>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroI8,
                            >,
                        >()
                        .process_type::<LuaNonZeroU128>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroU128,
                            >,
                        >()
                        .process_type::<LuaNonZeroU16>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroU16,
                            >,
                        >()
                        .process_type::<LuaNonZeroU32>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroU32,
                            >,
                        >()
                        .process_type::<LuaNonZeroU64>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroU64,
                            >,
                        >()
                        .process_type::<LuaNonZeroU8>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroU8,
                            >,
                        >()
                        .process_type::<LuaNonZeroUsize>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroUsize,
                            >,
                        >()
                        .process_type::<LuaPathBuf>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaPathBuf>,
                        >()
                        .process_type::<LuaRangeFull>()
                        .process_type::<LuaQuat>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaQuat>,
                        >()
                        .process_type::<LuaVec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaVec3>,
                        >()
                        .process_type::<LuaIVec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaIVec2>,
                        >()
                        .process_type::<LuaIVec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaIVec3>,
                        >()
                        .process_type::<LuaIVec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaIVec4>,
                        >()
                        .process_type::<LuaI64Vec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaI64Vec2>,
                        >()
                        .process_type::<LuaI64Vec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaI64Vec3>,
                        >()
                        .process_type::<LuaI64Vec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaI64Vec4>,
                        >()
                        .process_type::<LuaUVec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaUVec2>,
                        >()
                        .process_type::<LuaUVec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaUVec3>,
                        >()
                        .process_type::<LuaUVec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaUVec4>,
                        >()
                        .process_type::<LuaU64Vec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaU64Vec2>,
                        >()
                        .process_type::<LuaU64Vec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaU64Vec3>,
                        >()
                        .process_type::<LuaU64Vec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaU64Vec4>,
                        >()
                        .process_type::<LuaVec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaVec2>,
                        >()
                        .process_type::<LuaVec3A>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaVec3A>,
                        >()
                        .process_type::<LuaVec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaVec4>,
                        >()
                        .process_type::<LuaBVec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaBVec2>,
                        >()
                        .process_type::<LuaBVec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaBVec3>,
                        >()
                        .process_type::<LuaBVec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaBVec4>,
                        >()
                        .process_type::<LuaDVec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDVec2>,
                        >()
                        .process_type::<LuaDVec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDVec3>,
                        >()
                        .process_type::<LuaDVec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDVec4>,
                        >()
                        .process_type::<LuaMat2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaMat2>,
                        >()
                        .process_type::<LuaMat3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaMat3>,
                        >()
                        .process_type::<LuaMat3A>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaMat3A>,
                        >()
                        .process_type::<LuaMat4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaMat4>,
                        >()
                        .process_type::<LuaDMat2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDMat2>,
                        >()
                        .process_type::<LuaDMat3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDMat3>,
                        >()
                        .process_type::<LuaDMat4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDMat4>,
                        >()
                        .process_type::<LuaAffine2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaAffine2>,
                        >()
                        .process_type::<LuaAffine3A>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaAffine3A,
                            >,
                        >()
                        .process_type::<LuaDAffine2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaDAffine2,
                            >,
                        >()
                        .process_type::<LuaDAffine3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaDAffine3,
                            >,
                        >()
                        .process_type::<LuaDQuat>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDQuat>,
                        >()
                        .process_type::<LuaEulerRot>()
                        .process_type::<LuaBVec3A>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaBVec3A>,
                        >()
                        .process_type::<LuaBVec4A>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaBVec4A>,
                        >()
                        .process_type::<LuaDirection2d>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaDirection2d,
                            >,
                        >()
                        .process_type::<LuaCircle>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaCircle>,
                        >()
                        .process_type::<LuaEllipse>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaEllipse>,
                        >()
                        .process_type::<LuaPlane2d>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaPlane2d>,
                        >()
                        .process_type::<LuaLine2d>()
                        .process_type::<LuaSegment2d>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaSegment2d,
                            >,
                        >()
                        .process_type::<LuaTriangle2d>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaTriangle2d,
                            >,
                        >()
                        .process_type::<LuaRectangle>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaRectangle,
                            >,
                        >()
                        .process_type::<LuaRegularPolygon>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaRegularPolygon,
                            >,
                        >()
                        .process_type::<LuaCapsule2d>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaCapsule2d,
                            >,
                        >()
                        .process_type::<LuaDirection3d>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaDirection3d,
                            >,
                        >()
                        .process_type::<LuaSphere>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaSphere>,
                        >()
                        .process_type::<LuaPlane3d>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaPlane3d>,
                        >()
                        .process_type::<LuaLine3d>()
                        .process_type::<LuaSegment3d>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaSegment3d,
                            >,
                        >()
                        .process_type::<LuaCuboid>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaCuboid>,
                        >()
                        .process_type::<LuaCylinder>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaCylinder,
                            >,
                        >()
                        .process_type::<LuaCapsule3d>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaCapsule3d,
                            >,
                        >()
                        .process_type::<LuaCone>()
                        .process_type::<LuaConicalFrustum>()
                        .process_type::<LuaTorus>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaTorus>,
                        >()
                        .process_type::<LuaIRect>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaIRect>,
                        >()
                        .process_type::<LuaRect>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaRect>,
                        >()
                        .process_type::<LuaURect>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaURect>,
                        >()
                        .process_type::<LuaSmolStr>()
                        .process_type::<LuaNonZeroIsize>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaNonZeroIsize,
                            >,
                        >()
                        .process_type::<LuaUuid>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaUuid>,
                        >()
                },
            ),
        )
    }
    fn setup_script(
        &mut self,
        script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }
    fn setup_script_runtime(
        &mut self,
        world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        _script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }
    fn register_with_app(&self, app: &mut bevy::app::App) {
        app.register_foreign_lua_type::<bevy::utils::Duration>();
        app.register_foreign_lua_type::<bevy::utils::Instant>();
        app.register_foreign_lua_type::<std::num::NonZeroI128>();
        app.register_foreign_lua_type::<std::num::NonZeroI16>();
        app.register_foreign_lua_type::<std::num::NonZeroI32>();
        app.register_foreign_lua_type::<std::num::NonZeroI64>();
        app.register_foreign_lua_type::<std::num::NonZeroI8>();
        app.register_foreign_lua_type::<std::num::NonZeroU128>();
        app.register_foreign_lua_type::<std::num::NonZeroU16>();
        app.register_foreign_lua_type::<std::num::NonZeroU32>();
        app.register_foreign_lua_type::<std::num::NonZeroU64>();
        app.register_foreign_lua_type::<std::num::NonZeroU8>();
        app.register_foreign_lua_type::<std::num::NonZeroUsize>();
        app.register_foreign_lua_type::<std::path::PathBuf>();
        app.register_foreign_lua_type::<std::ops::RangeFull>();
        app.register_foreign_lua_type::<bevy::math::Quat>();
        app.register_foreign_lua_type::<bevy::math::Vec3>();
        app.register_foreign_lua_type::<bevy::math::IVec2>();
        app.register_foreign_lua_type::<bevy::math::IVec3>();
        app.register_foreign_lua_type::<bevy::math::IVec4>();
        app.register_foreign_lua_type::<bevy::math::I64Vec2>();
        app.register_foreign_lua_type::<bevy::math::I64Vec3>();
        app.register_foreign_lua_type::<bevy::math::I64Vec4>();
        app.register_foreign_lua_type::<bevy::math::UVec2>();
        app.register_foreign_lua_type::<bevy::math::UVec3>();
        app.register_foreign_lua_type::<bevy::math::UVec4>();
        app.register_foreign_lua_type::<bevy::math::U64Vec2>();
        app.register_foreign_lua_type::<bevy::math::U64Vec3>();
        app.register_foreign_lua_type::<bevy::math::U64Vec4>();
        app.register_foreign_lua_type::<bevy::math::Vec2>();
        app.register_foreign_lua_type::<bevy::math::Vec3A>();
        app.register_foreign_lua_type::<bevy::math::Vec4>();
        app.register_foreign_lua_type::<bevy::math::BVec2>();
        app.register_foreign_lua_type::<bevy::math::BVec3>();
        app.register_foreign_lua_type::<bevy::math::BVec4>();
        app.register_foreign_lua_type::<bevy::math::DVec2>();
        app.register_foreign_lua_type::<bevy::math::DVec3>();
        app.register_foreign_lua_type::<bevy::math::DVec4>();
        app.register_foreign_lua_type::<bevy::math::Mat2>();
        app.register_foreign_lua_type::<bevy::math::Mat3>();
        app.register_foreign_lua_type::<bevy::math::Mat3A>();
        app.register_foreign_lua_type::<bevy::math::Mat4>();
        app.register_foreign_lua_type::<bevy::math::DMat2>();
        app.register_foreign_lua_type::<bevy::math::DMat3>();
        app.register_foreign_lua_type::<bevy::math::DMat4>();
        app.register_foreign_lua_type::<bevy::math::Affine2>();
        app.register_foreign_lua_type::<bevy::math::Affine3A>();
        app.register_foreign_lua_type::<bevy::math::DAffine2>();
        app.register_foreign_lua_type::<bevy::math::DAffine3>();
        app.register_foreign_lua_type::<bevy::math::DQuat>();
        app.register_foreign_lua_type::<bevy::math::EulerRot>();
        app.register_foreign_lua_type::<bevy::math::BVec3A>();
        app.register_foreign_lua_type::<bevy::math::BVec4A>();
        app.register_foreign_lua_type::<bevy::math::primitives::Direction2d>();
        app.register_foreign_lua_type::<bevy::math::primitives::Circle>();
        app.register_foreign_lua_type::<bevy::math::primitives::Ellipse>();
        app.register_foreign_lua_type::<bevy::math::primitives::Plane2d>();
        app.register_foreign_lua_type::<bevy::math::primitives::Line2d>();
        app.register_foreign_lua_type::<bevy::math::primitives::Segment2d>();
        app.register_foreign_lua_type::<bevy::math::primitives::Triangle2d>();
        app.register_foreign_lua_type::<bevy::math::primitives::Rectangle>();
        app.register_foreign_lua_type::<bevy::math::primitives::RegularPolygon>();
        app.register_foreign_lua_type::<bevy::math::primitives::Capsule2d>();
        app.register_foreign_lua_type::<bevy::math::primitives::Direction3d>();
        app.register_foreign_lua_type::<bevy::math::primitives::Sphere>();
        app.register_foreign_lua_type::<bevy::math::primitives::Plane3d>();
        app.register_foreign_lua_type::<bevy::math::primitives::Line3d>();
        app.register_foreign_lua_type::<bevy::math::primitives::Segment3d>();
        app.register_foreign_lua_type::<bevy::math::primitives::Cuboid>();
        app.register_foreign_lua_type::<bevy::math::primitives::Cylinder>();
        app.register_foreign_lua_type::<bevy::math::primitives::Capsule3d>();
        app.register_foreign_lua_type::<bevy::math::primitives::Cone>();
        app.register_foreign_lua_type::<bevy::math::primitives::ConicalFrustum>();
        app.register_foreign_lua_type::<bevy::math::primitives::Torus>();
        app.register_foreign_lua_type::<bevy::math::IRect>();
        app.register_foreign_lua_type::<bevy::math::Rect>();
        app.register_foreign_lua_type::<bevy::math::URect>();
        app.register_foreign_lua_type::<smol_str::SmolStr>();
        app.register_foreign_lua_type::<std::num::NonZeroIsize>();
        app.register_foreign_lua_type::<bevy::utils::Uuid>();
    }
}