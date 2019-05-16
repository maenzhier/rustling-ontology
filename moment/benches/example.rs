#[macro_use]
extern crate bencher;
extern crate rustling_ontology_moment;
extern crate chrono;

use rustling_ontology_moment::*;
use bencher::Bencher;
use chrono::TimeZone;
use chrono::offset::local::Local;

fn build_context(moment: Moment<Local>) -> Context<Local> {
    let now = Interval::starting_at(moment, Grain::Second);
    Context::for_reference(now)
}

fn bench_hour_minute_with_intersection(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = Hour::clock_24(10).intersect(&Minute::new(5));
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_hour_minute(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = HourMinute::clock_24(10, 5);
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_month_day_with_intersection(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = Month::new(10).intersect(&DayOfMonth::new(5));
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_month_day(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = MonthDay::new(10, 5);
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_year_month_day_with_intersection(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = Year::new(2017).intersect(&Month::new(10)).intersect(&DayOfMonth::new(5));
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_day_month_year_with_intersection(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = DayOfMonth::new(5).intersect(&Month::new(10)).intersect(&Year::new(2017));
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_weekday_month_day_with_intersection(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = DayOfWeek::new(Weekday::Mon).intersect(&DayOfMonth::new(5)).intersect(&Month::new(10));
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_weeday_day_to_weekday_day(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let start = DayOfWeek::new(Weekday::Mon).intersect(&DayOfMonth::new(5));
    let end = DayOfWeek::new(Weekday::Sun).intersect(&DayOfMonth::new(12));
    let constraint = start.span_to(&end);
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

fn bench_year_month_day(bench: &mut Bencher) {
    let context = build_context(Moment(Local.ymd(2017, 04, 25).and_hms(9, 10, 11)));
    let constraint = YearMonthDay::new(2017, 10, 5);
    let walker = constraint.to_walker(&context.reference, &context);
    bench.iter(|| walker.forward.clone().into_iter().take(5).collect::<Vec<_>>());
}

benchmark_group!(benches,
                 bench_hour_minute,
                 bench_hour_minute_with_intersection,
                 bench_month_day_with_intersection,
                 bench_year_month_day_with_intersection,
                 bench_year_month_day,
                 bench_month_day,
                 bench_day_month_year_with_intersection,
                 bench_weekday_month_day_with_intersection,
                 bench_weeday_day_to_weekday_day
                 );
benchmark_main!(benches);