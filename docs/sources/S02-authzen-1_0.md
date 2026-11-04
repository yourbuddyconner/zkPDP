# S02: OpenID AuthZEN Authorization API 1.0

Gazitt, O., Brossard, D., Tulshibagwale, A., editors. OpenID Foundation.
Final. Published 11 January 2026.
URL: https://openid.net/specs/authorization-api-1_0.html
Accessed 2026-11-04. Trust: primary. Sections 6, 7, 9 and 11.6 read through a
fetch tool. Last 4 percent of the page not read.

## What it says

- Access Evaluation API, `POST /access/v1/evaluation`. Request: `subject` (type, id, properties), `action` (name, properties), `resource` (type, id, properties), optional `context` object.
- Response: `decision` boolean, optional `context` object whose semantics are "an implementation concern".
- Receivers MUST ignore unknown fields in request and response bodies.
- Access Evaluations API, `POST /access/v1/evaluations`, batch with defaults and an `evaluations_semantic` option.
- Search APIs for subject, resource, action, with pagination.
- Discovery at `/.well-known/authzen-configuration` with required `policy_decision_point` and `access_evaluation_endpoint`, optional endpoints, `capabilities` as an array of registered URNs, and `signed_metadata`.
- Section 11.6 allows a PDP to sign its response without specifying the mechanism.

## What we take

- The envelope in `docs/protocol.md` is built on the response `context` and the ignore-unknown-fields rule. Decision 0002.
- Batch shape for the N-decision journal.
- Search APIs are out of scope.

## Open items

- A secondary source gives the approval date as 2026-01-12. The spec page says 11 January 2026. We use the spec page.
- Check whether any capability URN registry exists yet.
