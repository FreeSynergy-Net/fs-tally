# Changelog

Version = tag; all crates of the workspace carry the same version (ADR 0064, 0.0.N). No tag is set by the work branch.

## Unreleased

### English

- **Date poll counting rule sorted in (workflow `geruest-heben`).** The counting rule of the date poll moved here from the frontend of `fs-decisions` (`fs-decisions-simple/src/grid.rs` at 5cd8806; owner decision 2026-11-17: Yes, If needed, No stay apart, no weighting; the suggested window has the most commitments, then fewer "If needed", then the earliest): `src/availability.rs` with `Answer`, `AnswerCount`, `count_answers`, `best_window`, tests in `tests/availability.rs`. Only renamed (`Tally`, `tally`, `best` would clash with the result type of this crate); logic unchanged. It is built and tested: the gates pass in both environments (normal and CI) and in a shallow clone without tags, and the tests turn red under seven mutations of the rule (including one that only a window with "If needed" answers catches). It is not yet a `Method` of the registry and has no ballot form (open talk: make it a `DecisionMethod`).

### Deutsch

- **Zählregel der Terminumfrage einsortiert (Workflow `geruest-heben`).** Die Zählregel der Terminumfrage ist aus dem Frontend von `fs-decisions` (`fs-decisions-simple/src/grid.rs` bei 5cd8806; Entscheidung des Inhabers 2026-11-17: Ja, Wenn nötig, Nein bleiben getrennt, keine Gewichtung; vorgeschlagen wird das Fenster mit den meisten Zusagen, dann mit weniger „Wenn nötig“, dann das früheste) hierher gezogen: `src/availability.rs` mit `Answer`, `AnswerCount`, `count_answers`, `best_window`, Tests in `tests/availability.rs`. Nur umbenannt (`Tally`, `tally`, `best` stießen mit dem Ergebnistyp dieses Crates zusammen); Logik unverändert. Gebaut und getestet: die Gates laufen in beiden Umgebungen (normal und CI) und in einem flachen Klon ohne Tags grün, und die Tests werden unter sieben Mutationen der Regel rot (darunter eine, die nur ein Fenster mit „Wenn nötig“-Antworten findet). Noch keine `Method` der Registry und keine Stimmzettel-Form (offenes Gespräch: zu einem `DecisionMethod` machen).

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
