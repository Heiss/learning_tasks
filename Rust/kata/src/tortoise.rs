struct Hour(i32);
struct Minute(i32);
struct Second(i32);

struct FeetPerHour(i32);
struct Feet(i32);

pub fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    let catch = Catch::create(FeetPerHour(v1), FeetPerHour(v2), Feet(g)).ok()?;

    Some(vec![
        catch.hours().0,
        catch.minutes_under_hour().0,
        catch.seconds_under_minute().0,
    ])
}

struct Catch {
    val: Second,
}

enum CatchError {
    InvalidVelocity,
}

impl Catch {
    fn create(v1: FeetPerHour, v2: FeetPerHour, g: Feet) -> Result<Catch, CatchError> {
        if v1.0 >= v2.0 {
            return Err(CatchError::InvalidVelocity);
        }

        // x = g / (v_2 - v_1)
        let val = Second(((g.0 as f32 * 3600.0) / (v2.0 as f32 - v1.0 as f32)) as i32);
        Ok(Catch { val })
    }

    fn seconds_under_minute(&self) -> Second {
        Second(self.val.0 % 60)
    }
    fn minutes_under_hour(&self) -> Minute {
        Minute(self.val.0 / 60 % 60)
    }
    fn hours(&self) -> Hour {
        Hour(self.val.0 / 3600 % 60)
    }
}

#[test]
fn basic_tests() {
    assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
    assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
    assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
    assert_eq!(race(820, 81, 550), None);
}
