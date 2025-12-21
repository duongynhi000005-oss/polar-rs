# polar-rs

`polar-rs` is a Rust SDK for interacting with the [Polar.sh](https://polar.sh) payment platform.  
It provides an ergonomic and type-safe interface for working with customers, subscriptions, products, orders, checkouts, benefits, and webhooks.

---

## Features

- Create, update, retrieve, and delete customers.
- Manage subscriptions and products.
- Create and list checkout sessions.
- Handle license keys and customer portal sessions.
- Manage benefits and benefit grants.
- Verify webhooks and parse webhook events.
- Paginated API responses with `ListParams`.

---

## Installation

Add `polar-rs` to your `Cargo.toml`:

```toml
[dependencies]
polar-rs = "0.1.0"
