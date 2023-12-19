#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    let mut total = 0;
    for n in 1..num {
        if num % n == 0 {
            total += n
        }
    }
    if total == num {
        Some(Classification::Perfect)
    } else if total > num {
        Some(Classification::Abundant)
    } else if total < num {
        Some(Classification::Deficient)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! tests {
    ($property_test_func:ident {
        $( $(#[$attr:meta])* $test_name:ident( $( $param:expr ),* ); )+
    }) => {
        $(
            $(#[$attr])*
            #[test]
            fn $test_name() {
                $property_test_func($( $param ),* )
            }
        )+
    }
}

    fn test_classification(num: u64, result: Classification) {
        assert_eq!(classify(num), Some(result));
    }

    #[test]
    fn basic() {
        assert_eq!(classify(0), None);
    }

    tests! {
        test_classification {
            test_1(1, Classification::Deficient);
            test_2(2, Classification::Deficient);
            test_4(4, Classification::Deficient);
            test_6(6, Classification::Perfect);
            test_12(12, Classification::Abundant);
            test_28(28, Classification::Perfect);
            test_30(30, Classification::Abundant);
            test_32(32, Classification::Deficient);
            test_33550335(33_550_335, Classification::Abundant);
            test_33550336(33_550_336, Classification::Perfect);
            test_33550337(33_550_337, Classification::Deficient);
        }
    }
}
