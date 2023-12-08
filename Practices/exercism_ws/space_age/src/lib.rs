const EARTH_YEARS_IN_SECONDS: f64 = 365.25 * 24. * 60. * 60.;
const EARTH_ORBITAL_PERIOD: f64 = 1.;
const MERCURY_ORBITAL_PERIOD: f64 = 0.2408467;
const VENUS_ORBITAL_PERIOD: f64 = 0.61519726;
const MARS_ORBITAL_PERIOD: f64 = 1.8808158;
const JUPITER_ORBITAL_PERIOD: f64 = 11.862615;
const SATURN_ORBITAL_PERIOD: f64 = 29.447498;
const URANUS_ORBITAL_PERIOD: f64 = 84.016846;
const NEPTUNE_ORBITAL_PERIOD: f64 = 164.79132;
#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (EARTH_YEARS_IN_SECONDS * EARTH_ORBITAL_PERIOD)
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (EARTH_YEARS_IN_SECONDS * MERCURY_ORBITAL_PERIOD)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (EARTH_YEARS_IN_SECONDS * VENUS_ORBITAL_PERIOD)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (EARTH_YEARS_IN_SECONDS * EARTH_ORBITAL_PERIOD)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (EARTH_YEARS_IN_SECONDS * MARS_ORBITAL_PERIOD)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (EARTH_YEARS_IN_SECONDS * JUPITER_ORBITAL_PERIOD)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (EARTH_YEARS_IN_SECONDS * SATURN_ORBITAL_PERIOD)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (EARTH_YEARS_IN_SECONDS * URANUS_ORBITAL_PERIOD)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.seconds / (EARTH_YEARS_IN_SECONDS * NEPTUNE_ORBITAL_PERIOD)
    }
}

fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
    }
}

#[test]

fn age_on_earth() {
    let seconds = 1000000000;
    let duration = Duration::from(seconds);
    let output = Earth::years_during(&duration);
    let expected = 31.69;
    assert_in_delta(expected, output);
}

#[test]
fn age_on_mercury() {
    let seconds = 2134835688;
    let duration = Duration::from(seconds);
    let output = Mercury::years_during(&duration);
    let expected = 280.88;
    assert_in_delta(expected, output);
}

#[test]
fn age_on_venus() {
    let seconds = 189839836;
    let duration = Duration::from(seconds);
    let output = Venus::years_during(&duration);
    let expected = 9.78;
    assert_in_delta(expected, output);
}

#[test]
fn age_on_mars() {
    let seconds = 2129871239;
    let duration = Duration::from(seconds);
    let output = Mars::years_during(&duration);
    let expected = 35.88;
    assert_in_delta(expected, output);
}

#[test]
fn age_on_jupiter() {
    let seconds = 901876382;
    let duration = Duration::from(seconds);
    let output = Jupiter::years_during(&duration);
    let expected = 2.41;
    assert_in_delta(expected, output);
}

#[test]
fn age_on_saturn() {
    let seconds = 2000000000;
    let duration = Duration::from(seconds);
    let output = Saturn::years_during(&duration);
    let expected = 2.15;
    assert_in_delta(expected, output);
}

#[test]
fn age_on_uranus() {
    let seconds = 1210123456;
    let duration = Duration::from(seconds);
    let output = Uranus::years_during(&duration);
    let expected = 0.46;
    assert_in_delta(expected, output);
}

#[test]
fn age_on_neptune() {
    let seconds = 1821023456;
    let duration = Duration::from(seconds);
    let output = Neptune::years_during(&duration);
    let expected = 0.35;
    assert_in_delta(expected, output);
}
