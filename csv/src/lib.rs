use core::SampleValue;
use serde::{de, Deserialize, Serialize, Serializer};

trait ToCsv: Clone {
    type Target;

    fn to_csv(&self) -> Self::Target;
}

fn to_csv<T>(value: T) -> T::Target
where
    T: ToCsv,
{
    value.to_csv()
}

trait FromCsv: Sized {
    type Target;

    fn from_csv(val: Self::Target) -> Result<Self, String>;
}

impl ToCsv for SampleValue {
    type Target = u64;

    fn to_csv(&self) -> Self::Target {
        match self {
            SampleValue::SampleA => 0,
            SampleValue::SampleB => 1,
            SampleValue::Other(_) => 999,
        }
    }
}

impl FromCsv for SampleValue {
    type Target = u64;

    fn from_csv(val: Self::Target) -> Result<Self, String> {
        match val {
            0 => Ok(SampleValue::SampleA),
            1 => Ok(SampleValue::SampleB),
            999 => Ok(SampleValue::Other("".to_string())),
            _ => Err("Fail".to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct CsvValue<T>(T)
where
    T: Eq + ToCsv + FromCsv;

impl<T> CsvValue<T>
where
    T: Sized + Eq + ToCsv + FromCsv,
{
    fn new(val: T) -> Self {
        Self(val)
    }

    fn value(&self) -> &T {
        &self.0
    }
}

impl<T: Eq + ToCsv<Target = u64> + FromCsv<Target = u64>> Serialize for CsvValue<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let val: Self = self.clone();
        serializer.serialize_u64(val.value().to_csv())
    }
}

impl<'de, T: Eq + ToCsv<Target = u64> + FromCsv<Target = u64>> Deserialize<'de> for CsvValue<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        let csv_value = T::from_csv(value).unwrap();
        Ok(Self::new(csv_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_csv_value() {
        let v = SampleValue::SampleA;
        assert_eq!(0, to_csv(v));
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct SampleCsvRecord {
        value_a: CsvValue<SampleValue>,
        value_b: CsvValue<SampleValue>,
    }

    #[test]
    fn parse_csv() {
        let data = "value_a,value_b\n1,0\n";
        let mut rdr = csv::Reader::from_reader(data.as_bytes());
        for result in rdr.deserialize() {
            let record: SampleCsvRecord = result.unwrap();
            assert_eq!(
                SampleCsvRecord {
                    value_a: CsvValue::new(SampleValue::SampleB),
                    value_b: CsvValue::new(SampleValue::SampleA)
                },
                record
            )
        }
    }
}
