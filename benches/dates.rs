use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use unic_datetime::DateTime;
use unic_datetime::DateTimeFormat;
use unic_datetime::{DateStyle, TimeStyle};

fn date_time(c: &mut Criterion) {
    let dates = &[
        DateTime::new(2019, 10, 29, 10, 23, 5),
        DateTime::new(2019, 1, 1, 10, 7, 35),
        DateTime::new(2018, 11, 1, 1, 59, 25),
        DateTime::new(2011, 7, 7, 2, 2, 59),
        DateTime::new(2019, 10, 29, 10, 23, 5),
        DateTime::new(2019, 10, 29, 10, 23, 5),
        DateTime::new(2019, 10, 29, 10, 23, 5),
        DateTime::new(2019, 10, 29, 10, 23, 5),
        DateTime::new(2019, 10, 29, 10, 23, 5),
        DateTime::new(2019, 10, 29, 10, 23, 5),
    ];
    let values = &[
        ("pl", Some(DateStyle::FULL), None),
        ("pl", Some(DateStyle::LONG), None),
        ("pl", Some(DateStyle::MEDIUM), None),
        ("pl", Some(DateStyle::SHORT), None),
        ("pl", None, Some(TimeStyle::FULL)),
        ("pl", None, Some(TimeStyle::LONG)),
        ("pl", None, Some(TimeStyle::MEDIUM)),
        ("pl", None, Some(TimeStyle::SHORT)),
        ("pl", Some(DateStyle::FULL), Some(TimeStyle::FULL)),
        ("pl", Some(DateStyle::LONG), Some(TimeStyle::LONG)),
        ("pl", Some(DateStyle::MEDIUM), Some(TimeStyle::MEDIUM)),
        ("pl", Some(DateStyle::SHORT), Some(TimeStyle::SHORT)),
    ];
    c.bench_function("date_time", |b| {
        b.iter(|| {
            for value in values {
                let dtf = DateTimeFormat::new(value.0, value.1, value.2);
                for date in dates {
                    let _ = dtf.format(date);
                }
            }
        })
    });
}

criterion_group!(benches, date_time);
criterion_main!(benches);
