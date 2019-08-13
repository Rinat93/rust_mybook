pub mod Enums {
    enum TimeUnit {
        Seconds, 
        Minutes, 
        Hours,
        Days,
        Months, 
        Years
    }

    impl TimeUnit {
        fn plural(self) -> &'static str {
            match self {
                TimeUnit::Seconds => "seconds",
                TimeUnit::Minutes => "Minutes",
                TimeUnit::Hours => "Hours",
                TimeUnit::Days => "Days",
                TimeUnit::Months => "Months",
                TimeUnit::Years => "Years",
            }
        }
    }

    enum Variant {
        TimesIn (TimeUnit, u32),
        JustNow,
        InTheFuture(TimeUnit, u32)
    }

    fn tests_enum(vr: Variant) -> String {
        match vr {
            Variant::TimesIn(units, count) => 
                format!("{} {} ago", units.plural(), count),
            Variant::JustNow => 
                format!("just now"),
            Variant::InTheFuture(units, count) => 
                format!("{} {} ago", units.plural(),count)
        }
    }

    pub fn server() {
        // Проверка как работает enum
        println!("{}",tests_enum(Variant::InTheFuture(TimeUnit::Days,1)));
    }
}