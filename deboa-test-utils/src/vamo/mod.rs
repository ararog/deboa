use crate::common::data::{JSON_PATCH, JSON_POST};
use caramelo::{expect, matchers::eq};
use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    serde::RequestBody,
    url::IntoUrl as _,
    Client, InnerClient, TestResult,
};
use deboa_extras::serde::json::JsonBody;
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use http::StatusCode;
use serde::Serialize;
use vamo::{
    resource::{Resource, ResourceMethod},
    Vamo,
};

#[derive(Serialize)]
struct Post {
    id: u64,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<u64>,
}

impl Resource for Post {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn name(&self) -> &str {
        "posts"
    }

    fn body_type(&self) -> impl RequestBody {
        JsonBody
    }
}

pub async fn test_get<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"pong"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let mut vamo = Vamo::from_client(client)?
        .base_url(
            server
                .base_url()
                .into_url()?,
        )
        .version(protocol_version);
    let response = vamo
        .get("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .text()
            .await?,
        "pong"
    );

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_put<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PUT").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"pong"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let mut vamo = Vamo::from_client(client)?
        .base_url(
            server
                .base_url()
                .into_url()?,
        )
        .version(protocol_version);
    let response = vamo
        .put("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    expect(
        response
            .text()
            .await?,
    )
    .to_be(eq("pong"));

    Ok(())
}

pub async fn test_post<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("POST").and(path("/api/posts"))).will_return(
            StatusCode::CREATED
                .respond()
                .with_body(
                    b"{\"id\":1,\"title\":\"Some title\",\"body\":\"Some body\",\"user_id\":1}",
                ),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let base_url = server
        .base_url()
        .into_url()?
        .join("/api")?;

    let vamo = Vamo::from_client(client)?
        .base_url(base_url)
        .version(protocol_version);
    let response = vamo
        .body_as(JsonBody, post)?
        .post("/posts")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);
    expect(
        response
            .bytes()
            .await?,
    )
    .to_be(eq(
        b"{\"id\":1,\"title\":\"Some title\",\"body\":\"Some body\",\"user_id\":1}".to_vec()
    ));

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_patch<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PATCH").and(path("/api/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"pong"),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let base_url = server
        .base_url()
        .into_url()?
        .join("/api")?;

    let mut vamo = Vamo::from_client(client)?
        .base_url(base_url)
        .version(protocol_version);
    let response = vamo
        .patch("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    expect(
        response
            .text()
            .await?,
    )
    .to_be(eq("pong"));

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_delete<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("DELETE").and(path("/api/posts/1"))).will_return(
            StatusCode::NO_CONTENT
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let base_url = server
        .base_url()
        .into_url()?
        .join("/api")?;

    let mut vamo = Vamo::from_client(client)?
        .base_url(base_url)
        .version(protocol_version);
    let response = vamo
        .delete("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_post_resource<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("POST").and(path("/api/posts"))).will_return(
            StatusCode::CREATED
                .respond()
                .with_body(JSON_POST),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let mut post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let base_url = server
        .base_url()
        .into_url()?
        .join("/api")?;

    let mut vamo = Vamo::from_client(client)?
        .base_url(base_url)
        .version(protocol_version);
    let response = vamo
        .create(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);
    expect(
        response
            .bytes()
            .await?,
    )
    .to_be(eq(JSON_POST.to_vec()));

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_put_resource<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PUT").and(path("/api/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(JSON_PATCH),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let mut post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let base_url = server
        .base_url()
        .into_url()?
        .join("/api")?;

    let mut vamo = Vamo::from_client(client)?
        .base_url(base_url)
        .version(protocol_version);
    let response = vamo
        .update(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    expect(
        response
            .bytes()
            .await?,
    )
    .to_be(eq(JSON_PATCH.to_vec()));

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_patch_resource<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("PATCH").and(path("/api/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(JSON_PATCH),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let mut post = Post { id: 1, title: "Some other title".to_string(), body: None, user_id: None };
    let base_url = server
        .base_url()
        .into_url()?
        .join("/api")?;
    let mut vamo = Vamo::from_client(client)?
        .base_url(base_url)
        .version(protocol_version);
    let response = vamo
        .edit(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    expect(
        response
            .bytes()
            .await?,
    )
    .to_be(eq(JSON_PATCH.to_vec()));

    server
        .stop()
        .await?;

    Ok(())
}

pub async fn test_remove_resource<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("DELETE").and(path("/api/posts/1"))).will_return(
            StatusCode::NO_CONTENT
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let mut post = Post { id: 1, title: "Some other title".to_string(), body: None, user_id: None };
    let base_url = server
        .base_url()
        .into_url()?
        .join("/api")?;
    let mut vamo = Vamo::from_client(client)?
        .base_url(base_url)
        .version(protocol_version);
    let response = vamo
        .remove(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    server
        .stop()
        .await?;

    Ok(())
}
