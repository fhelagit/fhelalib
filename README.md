# FHELA cryptographic library

This library implements our vision of TFHE schema.

## Testing
We use Proptest as well as regular unit test.
To run tests:
```
> cargo test
```
Since this library is in very begining of it's development some test casess can fail.

## Examples
Currently we have one example which implements string comparison.
to run example:
```
> cargo run --release --example strings
```

## Perfomance
Since this library is in very beginning of its development performance optimisations are supposed to be done in future.