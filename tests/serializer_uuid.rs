use async_graphql::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
struct AssetId(pub uuid::Uuid);

async_graphql::scalar!(AssetId);

#[tokio::test]
/// Test case for
/// https://github.com/async-graphql/async-graphql/issues/603
pub async fn test_serialize_uuid() {
    let generated_uuid = uuid::Uuid::new_v4();
    let generated_non_nil_uuid = uuid::NonNilUuid::new(generated_uuid).unwrap();

    struct Query {
        data: AssetId,
        non_nil: uuid::NonNilUuid
    }

    #[Object]
    impl Query {
        async fn data(&self) -> &AssetId {
            &self.data
        }
        async fn non_nil(&self) -> &uuid::NonNilUuid {
            &self.non_nil
        }
    }

    let schema = Schema::new(
        Query {
            data: AssetId(generated_uuid),
            non_nil: generated_non_nil_uuid,
        },
        EmptyMutation,
        EmptySubscription,
    );
    let query = r#"{ data, non_nil }"#;
    assert_eq!(
        schema.execute(query).await.data,
        value!({
            "data": generated_uuid.to_string(),
            "non_nil": generated_non_nil_uuid.to_string(),
        })
    );
}
