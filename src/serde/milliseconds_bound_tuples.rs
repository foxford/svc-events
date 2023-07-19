use serde::{de, ser};
use std::{fmt, ops::Bound};

pub fn serialize<S>(value: &[(Bound<i64>, Bound<i64>)], serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    use ser::SerializeSeq;

    let mut seq = serializer.serialize_seq(Some(value.len()))?;

    for (lt, rt) in value {
        let lt = match lt {
            Bound::Included(lt) | Bound::Excluded(lt) => Some(lt),
            Bound::Unbounded => None,
        };

        let rt = match rt {
            Bound::Included(rt) | Bound::Excluded(rt) => Some(rt),
            Bound::Unbounded => None,
        };

        seq.serialize_element(&(lt, rt))?;
    }

    seq.end()
}

type BoundedI64Tuple = (Bound<i64>, Bound<i64>);

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<BoundedI64Tuple>, D::Error>
where
    D: de::Deserializer<'de>,
{
    pub struct MillisecondsBoundTupleVisitor;

    impl<'de> de::Visitor<'de> for MillisecondsBoundTupleVisitor {
        type Value = Vec<(Bound<i64>, Bound<i64>)>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a list of [lt, rt) range of integer milliseconds")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut elements: Self::Value = vec![];

            while let Some((Some(lt), Some(rt))) = seq.next_element()? {
                if lt <= rt {
                    elements.push((Bound::Included(lt), Bound::Excluded(rt)))
                } else {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Str(&format!("[{lt}, {rt}]")),
                        &"lt <= rt",
                    ));
                }
            }

            Ok(elements)
        }
    }

    deserializer.deserialize_seq(MillisecondsBoundTupleVisitor)
}
