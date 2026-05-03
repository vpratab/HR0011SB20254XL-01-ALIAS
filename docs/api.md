# API

## C ABI

Declared in `bindings/include/rt_vlas.h`.

- `init_monitor()`: create a monitor handle
- `update_monitor(handle, snapshot)`: ingest a state snapshot and return an integer verdict
- `get_trust_score(handle)`: fetch the current trust score
- `shutdown_monitor(handle)`: release resources

Verdict encoding:

- `0`: Accept
- `1`: Flag
- `2`: Reject

## Snapshot Contract

The runtime monitor consumes an autonomy state snapshot containing:

- position, velocity, heading
- topic-specific trust inputs
- timing and command context

See `core/src/model.rs` for the exact schema.
