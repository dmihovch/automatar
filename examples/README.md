# Examples

Each example pairs a `.fsa` machine definition with a `.is` input string of the same name. Run commands from the repository root with the machine file first:

```sh
cargo run -- examples/valid/bin_end_1.fsa examples/valid/bin_end_1.is
cargo run -- examples/invalid/unknown_state.fsa examples/invalid/unknown_state.is
```

`valid/` contains machine definitions and inputs that are accepted. `invalid/` contains rejected inputs and malformed machine definitions. Input strings are trimmed before evaluation, so trailing newlines are ignored.

## Accepted inputs

| Pair | Language | Input |
| --- | --- | --- |
| `bin_end_1` | Binary strings ending in `1`. | `0110101010110001` |
| `substr_ab` | Strings over `a`, `b`, and `c` containing `ab`. | `ccaaabcc` |
| `symbols` | The existing four-state machine using letters, digits, and punctuation. | `a}1\,b01` |
| `even_zeros` | Binary strings containing an even number of `0` characters. | `001100` |
| `alternating_ab` | One or more repetitions of `ab`. | `ababab` |
| `zero_or_more_a` | Zero or more `a` characters. | Empty file, testing acceptance of the empty string. |

## Rejected inputs and definitions

| Pair | Expected result |
| --- | --- |
| `bin_end_1_rejected` | Input `011010` ends in `0`, leaving the machine in a non-accepting state. |
| `input_outside_alphabet` | Input `0121` contains `2` at index 2, outside the binary alphabet. |
| `missing_initial` | Definition is missing the `initial:` field. |
| `malformed_transition` | Transition `q0 1` is missing its result state. |
| `unknown_state` | Transition `q0 1 qMissing` targets a state absent from `states:`. |
| `unknown_symbol` | Transition `q0 x q1` uses a symbol absent from `alphabet:`. |
| `unclosed_transitions` | The transitions section is missing its closing brace. |

The CLI currently exits with status 0 for both accepted and rejected inputs. Rejected inputs print `The string is not in the language` or `The string contains all valid characters, but is not in the language`. Malformed definitions print an error and exit with status 1.
