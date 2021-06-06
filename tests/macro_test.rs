use futures_util::{stream, StreamExt};

use async_for::async_for;

#[futures_test::test]
async fn test_normal() {
    let st = stream::iter(vec![1, 2, 3]);

    let mut result = vec![];

    async_for! {
        async for n in st {
            result.push(n);
        }
    }

    assert_eq!(result, vec![1, 2, 3]);
}

#[futures_test::test]
async fn test_directly() {
    let mut result = vec![];

    async_for! {
        async for n in stream::iter(vec![1, 2, 3]) {
            result.push(n);
        }
    }

    assert_eq!(result, vec![1, 2, 3]);
}

#[futures_test::test]
async fn test_empty() {
    async_for! {
        async for _ in stream::iter(vec![1, 2, 3]) {}
    }
}
