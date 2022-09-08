use chrono::prelude::*;

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn check_date_time() {

        let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
        let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

        assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
        assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));
    }
}