# FHELA cryptographic library

This library implements our vision of TFHE schema.

## Testing
We use Proptest as well as regular unit tests.
To run tests:
```
> cargo test
```
Since this library is in very begining of it's development some test cases can fail.

## Examples
Currently we have one example which implements string comparison.
To run example:
```
> cargo run --release --example strings check-equility STRING_TO_BE_ENCRYPTED STRING_TO_COMPARED
```

## Perfomance
Since this library is in very beginning of its development performance optimisations are supposed to be done in future.