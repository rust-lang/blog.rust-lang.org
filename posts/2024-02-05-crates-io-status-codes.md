---
layout: post
title: "crates.io: API status code changes"
author: Tobias Bieniek
team: the crates.io team <https://www.rust-lang.org/governance/teams/crates-io>
---

When cargo and crates.io were first implemented, [HTTP response status codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status) were ignored. cargo assumed that crates.io would always respond with a "[200 OK](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/200)" and any error messages are detected from the response JSON body. When crates.io responded with a different status code then cargo was showing the raw JSON body instead of a regular error message:

```
error: failed to get a 200 OK response, got 400
headers:
    HTTP/1.1 400 Bad Request
    Content-Type: application/json; charset=utf-8
    Content-Length: 171

body:
{"errors":[{"detail":"missing or empty metadata fields: description, license. Please see https://doc.rust-lang.org/cargo/reference/manifest.html for how to upload metadata"}]}
```

This was improved in [cargo#6771](https://github.com/rust-lang/cargo/pull/6771), which was released in cargo 1.34 (mid-2019). Since then, cargo has supported [4xx](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#client_error_responses) and [5xx](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) status codes too and extracts the error message from the JSON response, if available.

The crates.io team has decided to phase out these "200 OK" response status codes for error responses to make our API a little more intuitive. Since this can be seen as a breaking change to our API we decided to announce this change publicly, including a cut-off date. On **2024-03-04** we will switch the API from the previous "200 OK" behavior to the new 4xx/5xx behavior, for the endpoints that are used by `cargo`:

- `GET /api/v1/crates`
- `PUT /api/v1/crates/new`
- `PUT /api/v1/crates/:crate/:version/yank`
- `DELETE /api/v1/crates/:crate/:version/unyank`
- `GET /api/v1/crates/:crate/owners`
- `PUT /api/v1/crates/:crate/owners`
- `DELETE /api/v1/crates/:crate/owners`

All other endpoints have already been using regular HTTP status codes for some time.

**cargo 1.33 and below will keep working after this change**, but will show the raw JSON response body instead of a regular error message. However, we recommend upgrading to a newer version of cargo to get the improved error messages and all the other nice things that the cargo team has built since then.