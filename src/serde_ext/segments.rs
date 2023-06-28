use super::milliseconds_bound_tuples;
use serde::{de, ser, Deserialize, Serialize};
use sqlx::postgres::types::PgRange;
use std::ops::Bound;

pub type BoundedOffsetTuples = Vec<(Bound<i64>, Bound<i64>)>;

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type, Eq, PartialEq)]
#[sqlx(transparent)]
#[serde(from = "BoundedOffsetTuples")]
#[serde(into = "BoundedOffsetTuples")]
pub struct Segments(Vec<PgRange<i64>>);

impl From<BoundedOffsetTuples> for Segments {
    fn from(segments: BoundedOffsetTuples) -> Self {
        Self(segments.into_iter().map(PgRange::from).collect())
    }
}

impl From<Segments> for BoundedOffsetTuples {
    fn from(val: Segments) -> Self {
        val.0.into_iter().map(|s| (s.start, s.end)).collect()
    }
}

impl From<Segments> for Vec<PgRange<i64>> {
    fn from(val: Segments) -> Self {
        val.0
    }
}

pub fn serialize<S>(value: &Segments, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    let bounded_offset_tuples: BoundedOffsetTuples = value.to_owned().into();
    milliseconds_bound_tuples::serialize(&bounded_offset_tuples, serializer)
}

pub fn deserialize<'de, D>(d: D) -> Result<Segments, D::Error>
where
    D: de::Deserializer<'de>,
{
    milliseconds_bound_tuples::deserialize(d).map(Segments::from)
}
