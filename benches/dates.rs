use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use unic_datetime::DateTime;
use unic_datetime::DateTimeFormat;
use unic_datetime::{DateStyle, TimeStyle};

fn date_time(c: &mut Criterion) {
    let dates = &[
        DateTime::new(2001, 9, 8, 18, 46, 40),
        DateTime::new(2017, 7, 13, 19, 40, 0),
        DateTime::new(2020, 9, 13, 5, 26, 40),
        DateTime::new(2021, 1, 6, 22, 13, 20),
        DateTime::new(2021, 5, 2, 17, 0, 0),
        DateTime::new(2021, 8, 26, 10, 46, 40),
        DateTime::new(2021, 12, 20, 3, 33, 20),
        DateTime::new(2022, 4, 14, 22, 20, 0),
        DateTime::new(2022, 8, 8, 16, 6, 40),
        DateTime::new(2033, 5, 17, 20, 33, 20),
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
                let dtf = DateTimeFormat::new(value.0, value.1, value.2, None);
                for date in dates {
                    let _ = dtf.format(date);
                }
            }
        })
    });

    #[cfg(feature = "serde")]
    c.bench_function("date_time_dynamic", |b| {
        let data = unic_datetime::data::load::get_calendar_data();
        b.iter(|| {
            for value in values {
                let dtf = DateTimeFormat::new(value.0, value.1, value.2, Some(std::borrow::Cow::Owned(data.clone())));
                for date in dates {
                    let _ = dtf.format(date);
                }
            }
        })
    });
}

criterion_group!(benches, date_time);
criterion_main!(benches);
