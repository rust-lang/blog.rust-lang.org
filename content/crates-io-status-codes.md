+++
path = "2024/02/06/crates-io-status-codes"
title = "crates.io: API status code changes"
authors = ["Tobias Bieniek"]
aliases = ["2024/02/06/crates-io-status-codes.html"]

[extra]
team = "the crates.io team"
team_url = "https://www.rust-lang.org/governance/teams/crates-io"
+++

Cargo and crates.io were developed in the rush leading up to the Rust 1.0 release to fill the needs for a tool to manage dependencies and a registry that people could use to share code. This rapid work resulted in these tools being connected with an API that initially didn't return the correct [HTTP response status codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status). After the Rust 1.0 release, Rust's stability guarantees around backward compatibility made this non-trivial to fix, as we wanted older versions of Cargo to continue working with the current crates.io API.

When an old version of Cargo receives a non-"[200 OK](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/200)" response, it displays the raw JSON body like this:

```
error: failed to get a 200 OK response, got 400
headers:
    HTTP/1.1 400 Bad Request
    Content-Type: application/json; charset=utf-8
    Content-Length: 171

body:
{"errors":[{"detail":"missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata"}]}
```

This was improved in pull request [#6771](https://github.com/rust-lang/cargo/pull/6771), which was released in Cargo 1.34 (mid-2019). Since then, Cargo has supported receiving [4xx](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#client_error_responses) and [5xx](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) status codes too and extracts the error message from the JSON response, if available.

On **2024-03-04** we will switch the API from returning "200 OK" status codes for errors to the new 4xx/5xx behavior. **Cargo 1.33 and below will keep working after this change**, but will show the raw JSON body instead of a nicely formatted error message. We feel confident that this degraded error message display will not affect very many users. According to the [crates.io request logs](https://p.datadoghq.com/sb/3a172e20-e9e1-11ed-80e3-da7ad0900002-973f4c1011257befa8598303217bfe3a) only very few requests are made by Cargo 1.33 and older versions.

This is the list of API endpoints that will be affected by this change:

- `GET /api/v1/crates`
- `PUT /api/v1/crates/new`
- `PUT /api/v1/crates/:crate/:version/yank`
- `DELETE /api/v1/crates/:crate/:version/unyank`
- `GET /api/v1/crates/:crate/owners`
- `PUT /api/v1/crates/:crate/owners`
- `DELETE /api/v1/crates/:crate/owners`

All other endpoints have already been using regular HTTP status codes for some time.

If you are still using Cargo 1.33 or older, we recommend upgrading to a newer version to get the improved error messages and all the other nice things that the Cargo team has built since then.
