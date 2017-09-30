use Decimal;
use num::FromPrimitive;
use serde;
use std::fmt;
use std::str::FromStr;
use serde::de::Unexpected;
use num::Zero;

impl<'de> serde::Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: serde::de::Deserializer<'de>, {
        deserializer.deserialize_str(DecimalVisitor)
    }
}

struct DecimalVisitor;

impl<'de> serde::de::Visitor<'de> for DecimalVisitor {
    type Value = Decimal;

    fn visit_f64<E>(self, value: f64) -> Result<Decimal, E>
        where E: serde::de::Error
    {
        Decimal::from_str(&value.to_string()).map_err(|_| E::invalid_value(Unexpected::Float(value), &self))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Decimal, E>
        where E: serde::de::Error
    {
        match Decimal::from_i64(value) {
            Some(s) => Ok(s),
            None => Err(E::invalid_value(Unexpected::Signed(value), &self)),
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Decimal, E>
        where E: serde::de::Error
    {
        match Decimal::from_u64(value) {
            Some(s) => Ok(s),
            None => Err(E::invalid_value(Unexpected::Unsigned(value), &self)),
        }
    }

    fn visit_str<E>(self, value: &str) -> Result<Decimal, E>
        where E: serde::de::Error
    {
        Decimal::from_str(value).map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
    }

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a Decimal type representing a fixed-point number"
        )
    }
}

impl serde::Serialize for Decimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer, {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod test {
    extern crate serde_json;
    extern crate bincode;

    use super::*;

    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct Record {
        amount: Decimal
    }

    #[test]
    fn deserialize_valid_decimal() {
        let data = [
            ("{\"amount\":\"1.234\"}", "1.234"),
            ("{\"amount\":1234}", "1234"),
        ];
        for &(serialized,value) in data.iter() {
            let record : Record = serde_json::from_str(serialized).unwrap();
            assert_eq!(value, record.amount.to_string());
        }
    }

    #[test]
    #[should_panic]
    fn deserialize_invalid_decimal() {
        let serialized = "{\"amount\":\"foo\"}";
        let _ : Record = serde_json::from_str(serialized).unwrap();
    }

    #[test]
    fn serialize_decimal() {
        let record = Record { amount: Decimal::new(1234, 3) };
        let serialized = serde_json::to_string(&record).unwrap();
        assert_eq!("{\"amount\":\"1.234\"}", serialized);
    }
    #[test]
    fn bincode() {
        let record = Record { amount: Decimal::new(1234, 3) };
        let encoded: Vec<u8> = bincode::serialize(&record, bincode::Infinite).unwrap();
        let decoded: Record = bincode::deserialize(&encoded[..]).unwrap();

        assert_eq!(record, decoded);
    }
    #[test]
    fn bincode_2() {
        let decimal = Decimal::new(1234, 3);
        let encoded: Vec<u8> = bincode::serialize(&decimal, bincode::Infinite).unwrap();
        let decoded: Decimal = bincode::deserialize(&encoded[..]).unwrap();

        assert_eq!(decimal, decoded);
    }
}
