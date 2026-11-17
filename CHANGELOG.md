# Changelog

Version = tag; all crates of the workspace carry the same version (ADR 0064, 0.0.N). No tag is set by the work branch.

## 0.0.1

### English

- First version as a library of its own: the ten counting methods (plurality, majority, supermajority, approval, score, Schulze, instant runoff, Borda, consent, systemic consensing), the four ballot forms, `evaluate`, `evaluate_with_held_back`, `method_for`, the trait `DecisionMethod`.
- Moved unchanged in behaviour from the crate `fs-forms-methods` of fs-forms (own code; the tests moved with it). Renamed: crate `fs-tally`, library `fs_tally`.
- New tests: the Schulze example with 45 voters from the literature, an independent brute-force reference for Schulze against random ballot sets, the Condorcet property.
- Not here on purpose: storage, anonymity, the smallest group, who may vote (those are `fs-ballot` and the program).

### Deutsch

- Erste Version als eigene Bibliothek: die zehn Zählverfahren (Plurality, Majority, Supermajority, Approval, Score, Schulze, Instant Runoff, Borda, Consent, Systemisches Konsensieren), die vier Stimmzettel-Formen, `evaluate`, `evaluate_with_held_back`, `method_for`, der Trait `DecisionMethod`.
- Im Verhalten unverändert aus dem Crate `fs-forms-methods` von fs-forms umgezogen (eigener Code; die Tests sind mitgezogen). Umbenannt: Crate `fs-tally`, Bibliothek `fs_tally`.
- Neue Tests: das Schulze-Beispiel mit 45 Wählenden aus der Literatur, eine unabhängige Referenz-Umsetzung von Schulze gegen Zufalls-Stimmzettel, die Condorcet-Eigenschaft.
- Absichtlich nicht hier: Speicher, Anonymität, Mindestgruppe, wer abstimmen darf (das sind `fs-ballot` und das Programm).
