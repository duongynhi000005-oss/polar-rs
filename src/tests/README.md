# Running Tests

## Unit Tests
Run all unit tests with mocked HTTP calls:
```bash
cargo test
```

## Specific test modules
```bash
cargo test --test client_tests
cargo test --test webhook_tests
cargo test --test models_tests
```

## Integration Tests
Integration tests run against the real Polar sandbox API.
Set your sandbox token first:

```bash
export POLAR_SANDBOX_TOKEN="polar_oat_sandbox_..."
cargo test --features integration-tests
```

## Notes
- Unit tests use `wiremock` to mock HTTP responses
- Webhook tests verify HMAC signature validation
- Model tests ensure proper serialization/deserialization
- Integration tests require a valid sandbox API token
*/