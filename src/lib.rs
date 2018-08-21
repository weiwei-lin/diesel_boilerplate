#[macro_export]
macro_rules! impl_from_sql_for_json {
    ($target:ty) => {
        impl ::diesel::deserialize::FromSql<::diesel::sql_types::Jsonb, ::diesel::pg::Pg> for $target {
            fn from_sql(bytes: Option<&[u8]>) -> ::diesel::deserialize::Result<Self> {
                use serde_json;
                use diesel::{sql_types, pg, deserialize};

                let value = <serde_json::Value as deserialize::FromSql<sql_types::Jsonb, pg::Pg>>::from_sql(bytes)?;
                let status = serde_json::from_value(value)?;
                Ok(status)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_to_sql_for_json {
    ($target:ty) => {
        impl ::diesel::serialize::ToSql<::diesel::sql_types::Jsonb, ::diesel::pg::Pg> for $target {
            fn to_sql<W: ::std::io::Write>(&self, out: &mut ::diesel::serialize::Output<W, ::diesel::pg::Pg>) -> ::diesel::serialize::Result {
                use serde_json;
                use diesel::{sql_types, pg, serialize};

                let value = serde_json::to_value(&self).unwrap();
                <serde_json::Value as serialize::ToSql<sql_types::Jsonb, pg::Pg>>::to_sql(&value, out)?;
                Ok(serialize::IsNull::No)
            }
        }
    };
}
