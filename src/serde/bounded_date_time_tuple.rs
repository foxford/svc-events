use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{de, ser};
use std::{fmt, ops::Bound};

pub type BoundedDateTimeTuple = (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>);

pub(crate) fn serialize<S>(
    value: &(Bound<DateTime<Utc>>, Bound<DateTime<Utc>>),
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    use ser::SerializeTuple;

    let (lt, rt) = value;
    let mut tup = serializer.serialize_tuple(2)?;

    match lt {
        Bound::Included(lt) => {
            let val = lt.timestamp();
            tup.serialize_element(&val)?;
        }
        Bound::Excluded(lt) => {
            // Adjusting the range to '[lt, rt)'
            let val = lt.timestamp() + 1;
            tup.serialize_element(&val)?;
        }
        Bound::Unbounded => {
            let val: Option<i64> = None;
            tup.serialize_element(&val)?;
        }
    }

    match rt {
        Bound::Included(rt) => {
            // Adjusting the range to '[lt, rt)'
            let val = rt.timestamp() - 1;
            tup.serialize_element(&val)?;
        }
        Bound::Excluded(rt) => {
            let val = rt.timestamp();
            tup.serialize_element(&val)?;
        }
        Bound::Unbounded => {
            let val: Option<i64> = None;
            tup.serialize_element(&val)?;
        }
    }

    tup.end()
}

pub fn deserialize<'de, D>(d: D) -> Result<BoundedDateTimeTuple, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_tuple(2, TupleSecondsTimestampVisitor)
}

struct TupleSecondsTimestampVisitor;

impl<'de> de::Visitor<'de> for TupleSecondsTimestampVisitor {
    type Value = BoundedDateTimeTuple;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a [lt, rt) range of unix time (seconds) or null (unbounded)")
    }

    /// Deserialize a tuple of two Bounded DateTime<Utc>
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let lt = match seq.next_element()? {
            Some(Some(val)) => {
                let ndt = NaiveDateTime::from_timestamp_opt(val, 0).ok_or(de::Error::custom(
                    format!("cannot convert {} secs to NaiveDateTime", val),
                ))?;
                let dt = DateTime::<Utc>::from_utc(ndt, Utc);
                Bound::Included(dt)
            }
            Some(None) => Bound::Unbounded,
            None => return Err(de::Error::invalid_length(1, &self)),
        };

        let rt = match seq.next_element()? {
            Some(Some(val)) => {
                let ndt = NaiveDateTime::from_timestamp_opt(val, 0).ok_or(de::Error::custom(
                    format!("cannot convert {} secs to NaiveDateTime", val),
                ))?;
                let dt = DateTime::<Utc>::from_utc(ndt, Utc);
                Bound::Excluded(dt)
            }
            Some(None) => Bound::Unbounded,
            None => return Err(de::Error::invalid_length(2, &self)),
        };

        Ok((lt, rt))
    }
}
