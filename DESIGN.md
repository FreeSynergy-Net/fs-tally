# fs-tally: Design

`fs-tally` is the counting library of FreeSynergy: pure strategies over ballots. It is used by `fs-forms` (surveys, polls) and `fs-decisions` (decisions), and can be used by anyone: it depends only on `thiserror`.

## Place in the family (whose data?)

A library has no data of its own. `fs-tally` never sees a person: a ballot is the voice of "one", and who that was is not its business. The urn that collects ballots without a link to a person is `fs-ballot`; the program that owns the decision (`fs-decisions`, `fs-forms`) owns the question, the electorate, the smallest group and the deadline. The creator fixes the method BEFORE the vote (decision b7, 17.11.).

## Ballots and methods

| Ballot form | Used by |
|---|---|
| `Single(choice)` | plurality, majority, supermajority |
| `Multiple(choices)` | approval |
| `Ranking(choices best to worst)` | Schulze, instant runoff, Borda |
| `Scores(choice, i32)` | score, consent (1 = objection, 0 = none), systemic consensing (resistance 0 to 10) |

Patterns: Strategy (`DecisionMethod`, one struct per method), Registry + Factory (`method_for`), Value objects (`Tally`, `OptionResult`), Tagged union (`Ballot`, `Method`).

Rules that do not bend:

- A ballot without content is dropped before counting; a form the method does not accept is an error.
- No chance in ties. Equal results share a rank; several winners are listed together.
- Held-back answers (`evaluate_with_held_back`): if the answers that are not shown could change a decided result, nobody is named.
- Schulze uses winning votes as the strength of a pairwise defeat; unranked choices are below every ranked one and equal to each other.
- Systemic consensing is provisional: a choice missing from a ballot counts as resistance 10.

## Tests

Oracles: the Schulze example of 45 voters (E wins; order E, A, C, B, D) from Schulze's text, an independent brute-force reference (every simple path, max of the weakest edge) against the library on pseudo-random partial rankings, the Condorcet property (a choice that beats all others pairwise is the only winner). Mutants of the core rules are run by hand before a tag (see CHANGELOG).

## Not here

Storage, anonymity, tokens, minimum group (`fs-ballot`); weights of voters; multi-winner and proportional methods; any clock.
