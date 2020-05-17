use std::marker::PhantomData;

use core::SampleValue;
use serde::{de, Deserialize, Serialize, Serializer};

/// CsvValue
///
/// Represents csv value
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
struct CsvValue<After, Before, Target>(After, PhantomData<Before>, PhantomData<Target>);

impl<After, Before, Target> Serialize for CsvValue<After, Before, Target>
where
    After: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<After, Before, Target> CsvValue<After, Before, Target> {
    pub fn new(value: After) -> Self {
        Self(value, PhantomData, PhantomData)
    }
    pub fn value(&self) -> &After {
        &self.0
    }
}

trait ToCsv<After, Before, Target>: Clone {
    fn to_csv(&self) -> CsvValue<After, Before, Target>;
}

trait FromCsv<T>: Sized {
    fn from_csv(val: Self) -> Result<T, String>;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum DefaultTarget {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum Japanese {}

impl<After, Before, Target> ToCsv<Option<After>, Before, Target> for Option<Before>
where
    Before: ToCsv<After, Before, Target>,
    After: Clone,
{
    fn to_csv(&self) -> CsvValue<Option<After>, Before, Target> {
        match self {
            Some(v) => {
                let v: CsvValue<After, Before, Target> = v.to_csv();
                let v = v.value();
                CsvValue(Some(v.clone()), PhantomData, PhantomData)
            }
            None => CsvValue(None, PhantomData, PhantomData),
        }
    }
}

impl ToCsv<String, Self, DefaultTarget> for SampleValue {
    fn to_csv(&self) -> CsvValue<String, Self, DefaultTarget> {
        match self {
            SampleValue::SampleA => CsvValue::new("SampleA".to_string()),
            SampleValue::SampleB => CsvValue::new("SampleB".to_string()),
            SampleValue::Other(s) => CsvValue::new(s.to_string()),
        }
    }
}

impl ToCsv<String, Self, Japanese> for SampleValue {
    fn to_csv(&self) -> CsvValue<String, Self, Japanese> {
        match self {
            SampleValue::SampleA => CsvValue::new("サンプルA".to_string()),
            SampleValue::SampleB => CsvValue::new("サンプルB".to_string()),
            SampleValue::Other(s) => CsvValue::new(format!("その他: {}", s)),
        }
    }
}

impl ToCsv<i64, Self, DefaultTarget> for SampleValue {
    fn to_csv(&self) -> CsvValue<i64, Self, DefaultTarget> {
        match self {
            SampleValue::SampleA => CsvValue::new(0),
            SampleValue::SampleB => CsvValue::new(1),
            SampleValue::Other(_) => CsvValue::new(999),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_csv_value() {
        let v = SampleValue::SampleA;
        let expected: CsvValue<String, _, DefaultTarget> = CsvValue::new("SampleA".to_string());
        let converted: CsvValue<String, _, DefaultTarget> = v.to_csv();
        assert_eq!(expected, converted);

        let expected: CsvValue<i64, _, DefaultTarget> = CsvValue::new(0);
        let converted: CsvValue<i64, _, DefaultTarget> = v.to_csv();
        assert_eq!(expected, converted);

        let maybe = Some(SampleValue::SampleA);
        let expected: CsvValue<Option<i64>, _, DefaultTarget> = CsvValue::new(Some(0));
        let converted: CsvValue<Option<i64>, _, DefaultTarget> = maybe.to_csv();

        assert_eq!(expected, converted);
        println!(
            "Debug: {}",
            serde_json::to_string_pretty(&converted).unwrap()
        )
    }

    #[derive(Serialize, Deserialize)]
    struct TestRecord<Target> {
        column_a: CsvValue<String, SampleValue, Target>,
        column_b: CsvValue<i64, SampleValue, Target>,
        column_c: CsvValue<Option<String>, SampleValue, Target>,
        column_d: CsvValue<Option<String>, SampleValue, Target>,
    }

    #[test]
    fn test_csv_record() {
        let record = TestRecord::<DefaultTarget> {
            column_a: CsvValue::new("SampleValueA".to_string()),
            column_b: CsvValue::new(10),
            column_c: CsvValue::new(None),
            column_d: CsvValue::new(Some("SampleValueA".to_string())),
        };

        let mut wrt = csv::Writer::from_writer(std::io::stdout());
        wrt.serialize(record).unwrap();
    }
}
