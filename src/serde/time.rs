use super::bounded_date_time_tuple::{self, BoundedDateTimeTuple};
use chrono::{DateTime, Utc};
use serde::{de, ser, Deserialize, Serialize};
use sqlx::postgres::types::PgRange;
use std::ops::Bound;

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type, Eq, PartialEq)]
#[sqlx(transparent)]
#[serde(from = "RoomTime")]
#[serde(into = "BoundedDateTimeTuple")]
pub struct Time(PgRange<DateTime<Utc>>);

impl From<RoomTime> for Time {
    fn from(time: RoomTime) -> Self {
        let time: BoundedDateTimeTuple = time.into();
        Self(PgRange::from(time))
    }
}

impl TryFrom<Time> for RoomTime {
    type Error = String;

    fn try_from(t: Time) -> Result<Self, Self::Error> {
        match RoomTime::new((t.0.start, t.0.end)) {
            Some(rt) => Ok(rt),
            None => Err(format!(
                "Invalid room time: ({:?}, {:?})",
                t.0.start, t.0.end
            )),
        }
    }
}

impl From<Time> for PgRange<DateTime<Utc>> {
    fn from(val: Time) -> Self {
        val.0
    }
}

impl From<Time> for BoundedDateTimeTuple {
    fn from(time: Time) -> BoundedDateTimeTuple {
        (time.0.start, time.0.end)
    }
}

impl From<BoundedDateTimeTuple> for Time {
    fn from(tuple: BoundedDateTimeTuple) -> Time {
        Self(PgRange::from(tuple))
    }
}

pub fn serialize<S>(value: &Time, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    bounded_date_time_tuple::serialize(&value.to_owned().into(), serializer)
}

pub fn deserialize<'de, D>(d: D) -> Result<Time, D::Error>
where
    D: de::Deserializer<'de>,
{
    let time = bounded_date_time_tuple::deserialize(d)?;
    Ok(Time::from(time))
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(try_from = "BoundedDateTimeTuple")]
#[serde(into = "BoundedDateTimeTuple")]
pub struct RoomTime {
    start: DateTime<Utc>,
    end: RoomTimeBound,
}

impl RoomTime {
    pub fn new(tuple: impl Into<BoundedDateTimeTuple>) -> Option<Self> {
        match tuple.into() {
            (Bound::Included(start), Bound::Excluded(end)) if start < end => Some(Self {
                start,
                end: RoomTimeBound::Excluded(end),
            }),
            (Bound::Included(start), Bound::Unbounded) => Some(Self {
                start,
                end: RoomTimeBound::Unbounded,
            }),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum RoomTimeBound {
    Excluded(DateTime<Utc>),
    Unbounded,
}

impl From<RoomTime> for BoundedDateTimeTuple {
    fn from(time: RoomTime) -> BoundedDateTimeTuple {
        match time.end {
            RoomTimeBound::Unbounded => (Bound::Included(time.start), Bound::Unbounded),
            RoomTimeBound::Excluded(end) => (Bound::Included(time.start), Bound::Excluded(end)),
        }
    }
}

impl TryFrom<BoundedDateTimeTuple> for RoomTime {
    type Error = String;

    fn try_from(t: BoundedDateTimeTuple) -> Result<Self, Self::Error> {
        match RoomTime::new((t.0, t.1)) {
            Some(rt) => Ok(rt),
            None => Err(format!("Invalid room time: ({:?}, {:?})", t.0, t.1)),
        }
    }
}
