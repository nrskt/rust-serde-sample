use std::marker::PhantomData;

use core::SampleValue;
use serde::{de, Deserialize, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct CsvValue<T, Target>(T, PhantomData<Target>);

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

#[derive(Clone, Debug, PartialEq, Eq)]
enum DefaultTarget {}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    let converted: Option<SampleValue> = maybe;
    let converted: CsvValue<Option<String>, DefaultTarget> = converted.to_csv();

    println!("{}", serde_json::to_string_pretty(&converted).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
}
