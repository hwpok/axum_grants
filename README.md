## Axum Grants
Axum Grants provides two Rust attribute macros based on permission-protected resources. Resource permissions are passed as parameters to the attribute macros, and the current user's permissions are contained within an Extension. Access to a resource can be determined by comparing whether the current user's permissions include the resource permissions.The basic principle of Axum Grants is to insert logic code into the processing functions using macros, thereby protecting resources through this logical code.  
For specific usage, please refer to the axum_example crate.


## Preparatory work
You need to define a structure,It must have perms field; the type can be Vec or HashSet.  
When there are many permission items, please use HashSet because its contains function has a time complexity of O(1).  
It like this:
```rust
#[derive(Debug, Clone, Default)]
pub struct Claims {
    pub user_id: u64, 
    pub user_name: String,
    ...
    pub perms: HashSet<String>, // must have this field
}
```

In your token middleware, insert Claims into the Axum Extension; the claims here can come from a database, Redis, or JWT:
```rust
async fn auth_middle_war(mut req: Request<Body>, next: Next) -> Response<Body> {
    // parse the claims from the token string
    let claims = verify_token(&APP_CFG.app.jwt_secret, token.as_str());
    
    // insert claims into extensions
    req.extensions_mut().insert(claims);
    next.run(req).await
}
```

## Sage instructions for the protect attribute macro:
When using the protect attribute macro on your Axum Handler  
if the protected resource only requires a single permission to access, you can set the permissions like this:
```rust
#[protect("opt_qry")]
```

When the protected resource requires any one of multiple permissions to access, you can set the permissions like this:
```rust
#[protect(any("opt_crt", "opt_upt", opt_del"))]
```

When the protected resource requires all of multiple permissions to access, you can set the permissions like this:
```rust
#[protect(all("opt_upt", "opt_del"))]
```

Here is a general example:
```rust
#[protect("opt_crt")]
async fn crt_handler(Extension(claims): Extension<Claims>) -> impl IntoResponse {
    Json(json!(
        {
            "cd": "0",
            "msg": "success",
            "data": "your business data"
        }
    )).into_response()
}
```

In the example above, the crt_handler handler function will have the following code inserted:
```rust
if !claims.perms.contains("opt_crt".to_string()) {
    return axum::http::Response::builder()
        .status(axum::http::StatusCode::FORBIDDEN)
        .body(Body::from("Insufficient permissions, need the permission: opt_crt "))
        .unwrap()
}
```

## Sage instructions for the protect_diy attribute macro:
The protect_diy macro’s method of setting permissions is consistent with the usage of the protect macro, but the protect_diy macro allows you to define your own Axum IntoResponse.  
You need to provide a structure with a specific name AxumGrantsResponse that has no fields. This structure must have a function with the signature fn get_into_response(msg: &str) -> impl IntoResponse.  
Here is a general example:
```rust
pub struct AxumGrantsResponse;
impl AxumGrantsResponse {
    pub fn get_into_response(msg: &str) -> impl IntoResponse {
        // axum::http::Response::builder()
        //     .status(axum::http::StatusCode::FORBIDDEN)
        //     .body(Body::from(msg.to_string())).unwrap()
        axum::Json(json!(
        {
            "cd": "403",
            "msg": msg,
        }))
    }
}
```
And then you can use it like this:
```rust
#[protect_diy("opt_crt")]
async fn crt_handler(Extension(claims): Extension<Claims>) -> impl IntoResponse {
  // your business code
  ...
}
```

In the example above, the crt_handler handler function will have the following code inserted:
```rust
if !claims.perms.contains("opt_crt".to_string()) {
    return return AxumGrantsResponse::get_into_response("Insufficient permissions, need the permission: opt_crt ").into_response();
}
```


