use std::marker::PhantomData;

use core::SampleValue;
use serde::{de, Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
struct CsvValue<T, Target>(T, PhantomData<Target>);

impl<T, Target> Serialize for CsvValue<T, Target>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<T, Target> CsvValue<T, Target> {
    pub fn value(&self) -> &T {
        &self.0
    }
}

impl<T, Target> From<T> for CsvValue<T, Target> {
    fn from(val: T) -> Self {
        CsvValue(val, PhantomData)
    }
}

trait ToCsv<T, Target>: Clone {
    fn to_csv(&self) -> CsvValue<T, Target>;
}

trait FromCsv<T>: Sized {
    fn from_csv(val: Self) -> Result<T, String>;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum DefaultTarget {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum Japanese {}

impl<T, U, Target> ToCsv<Option<U>, Target> for Option<T>
where
    T: ToCsv<U, Target>,
    U: Clone,
{
    fn to_csv(&self) -> CsvValue<Option<U>, Target> {
        match self {
            Some(v) => {
                let v: CsvValue<U, Target> = v.to_csv();
                let v = v.value();
                CsvValue(Some(v.clone()), PhantomData)
            }
            None => CsvValue(None, PhantomData),
        }
    }
}

impl ToCsv<String, DefaultTarget> for SampleValue {
    fn to_csv(&self) -> CsvValue<String, DefaultTarget> {
        match self {
            SampleValue::SampleA => "SampleA".to_string().into(),
            SampleValue::SampleB => "SampleB".to_string().into(),
            SampleValue::Other(s) => s.to_string().into(),
        }
    }
}

impl ToCsv<String, Japanese> for SampleValue {
    fn to_csv(&self) -> CsvValue<String, Japanese> {
        match self {
            SampleValue::SampleA => "サンプルA".to_string().into(),
            SampleValue::SampleB => "サンプルB".to_string().into(),
            SampleValue::Other(s) => format!("その他: {}", s).into(),
        }
    }
}

impl ToCsv<i64, DefaultTarget> for SampleValue {
    fn to_csv(&self) -> CsvValue<i64, DefaultTarget> {
        match self {
            SampleValue::SampleA => 0.into(),
            SampleValue::SampleB => 1.into(),
            SampleValue::Other(_) => 999.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_csv() {
        let v = SampleValue::SampleA;
        let expected: CsvValue<String, DefaultTarget> = "SampleA".to_string().into();
        let converted: CsvValue<String, DefaultTarget> = v.to_csv();
        assert_eq!(expected, converted);

        let expected: CsvValue<i64, DefaultTarget> = 0.into();
        let converted: CsvValue<i64, DefaultTarget> = v.to_csv();
        assert_eq!(expected, converted);

        let maybe = Some(SampleValue::SampleA);
        let expected: CsvValue<Option<i64>, DefaultTarget> = Some(0).into();
        let converted: CsvValue<Option<i64>, DefaultTarget> = maybe.to_csv();

        assert_eq!(expected, converted);
        println!(
            "Debug: {}",
            serde_json::to_string_pretty(&converted).unwrap()
        )
    }

    #[derive(Serialize, Deserialize)]
    struct TestRecord<Target> {
        column_a: CsvValue<String, Target>,
        column_b: CsvValue<i64, Target>,
        column_c: CsvValue<Option<String>, Target>,
        column_d: CsvValue<Option<String>, Target>,
    }

    #[test]
    fn test_csv_record() {
        let record = TestRecord::<DefaultTarget> {
            column_a: "SampleValueA".to_string().into(),
            column_b: 10.into(),
            column_c: None.into(),
            column_d: Some("SampleValueA".to_string()).into(),
        };

        let mut wrt = csv::Writer::from_writer(std::io::stdout());
        wrt.serialize(record).unwrap();
    }
}
