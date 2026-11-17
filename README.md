# FreeSynergy Tally: count votes by well-known methods

**A building block. Ballots go in, a result comes out, and the code never keeps anything.**

`fs-tally` is a small Rust library with ten counting methods for votes and decisions. You give it the choices and the ballots; it gives back who won, the number for every choice, the rank, and a plain-English note when nothing is decided or a tie needs reading. It has no storage, no clock and no network, so it can sit under a poll, a club ballot, a consent round or a date finder.

## What it can do today

Latest tag: none yet. The version in this tree is `0.0.1`.

Four ballot forms: one choice, several approved choices, a ranking, a score per choice. Ten methods:

| Method | Ballot form | What wins | Source |
|---|---|---|---|
| Plurality | one choice | most votes | standard (first past the post) |
| Majority | one choice | more than half of the ballots | standard |
| Supermajority | one choice | at least a set share (51 to 100 percent) | standard |
| Approval | several choices | most approvals | Brams and Fishburn, *Approval Voting* (1983) |
| Score | scores | highest sum | standard (range voting) |
| Schulze | ranking | undefeated by the strongest paths (beatpath, winning votes) | Markus Schulze, "A new monotonic, clone-independent, reversal symmetric, and Condorcet-consistent single-winner election method", *Social Choice and Welfare* 36 (2011) |
| Instant runoff | ranking | last place drops out, its ballots move on, until one has a majority | standard (alternative vote, single-winner STV) |
| Borda | ranking | most points by position | Jean-Charles de Borda (1770) |
| Consent | scores 1 (objection) or 0 | passes when nobody has a serious objection | sociocracy (Sociocracy 3.0 consent decision-making, CC BY-SA, described only) |
| Systemic consensing | resistance 0 to 10 | least total resistance | Paulus and Schrotta, systemic consensing (SK-Prinzip), described only |

Things to know:

- A ballot with no content is not counted, whatever the method. A ballot form that does not fit the method is an error, never a guess.
- Ties are never broken by chance: all tied winners are listed, or nothing is decided and the note says why.
- `evaluate_with_held_back` takes the number of answers that exist but are not shown yet. If they could change a decided result, it says "not decided yet" instead of naming a winner.
- Date rule (`availability`): for a date poll with the answers Yes, If needed, No. `count_answers` counts the three apart, `best_window` suggests the window with the most commitments (Yes plus If needed, no weighting), then fewer "If needed", then the earliest; it suggests nothing when nobody committed. A person decides; this only suggests. It is not yet a `Method` of the registry.
- Tests: the Schulze example of the 45 voters from the literature, an independent brute-force reference implementation of Schulze checked against the library on many pseudo-random ballot sets, the Condorcet property, and regression tests from review.

## What it does not do

- It does not store anything, and it does not know who voted. Anonymity and one-vote-per-person are the job of the urn library `fs-ballot`.
- Who may vote, the smallest group size and the deadline are not here. The program that uses this library decides that before the vote.
- No rules for seats (multi-winner methods), no proportional methods.

## Part of FreeSynergy

FreeSynergy is a family of programs that are free for everyone, forever, and that keep your data yours. The documentation, the manifesto and the overall plan live in [fs-documentation](https://github.com/FreeSynergy/fs-documentation); the reading version is at <https://freesynergy.github.io/fs-documentation/>.

## For developers

- Workspace crate: `fs-tally`. Entry points: `evaluate(&Method, &[String], &[Ballot]) -> Result<Tally>` and `evaluate_with_held_back`. Each method is one strategy behind the trait `DecisionMethod`; `method_for` maps a `Method` to its strategy (Strategy, Registry, Factory).
- Gates: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings -W clippy::pedantic`, `cargo test --workspace --all-features --locked`. Minimum Rust 1.89.
- Licence: MIT OR Apache-2.0. Claude (Anthropic) helped write this code; the work is checked by tests and by an independent review.
- Origin: moved from the crate `fs-forms-methods` of [fs-forms](https://github.com/FreeSynergy/fs-forms) (own code, moved with its tests).

---

# FreeSynergy Tally: Stimmen nach bekannten Verfahren zählen

**Ein Baustein. Stimmzettel gehen hinein, ein Ergebnis kommt heraus, und der Code speichert nichts.**

`fs-tally` ist eine kleine Rust-Bibliothek mit zehn Zählverfahren für Abstimmungen und Beschlüsse. Man gibt die Auswahl und die Stimmzettel hinein und bekommt zurück, wer gewonnen hat, die Zahl je Auswahl, den Rang und einen Satz in einfachem Englisch (Hinweis), wenn nichts entschieden ist oder ein Gleichstand gelesen werden muss. Es gibt keinen Speicher, keine Uhr und kein Netz; darum passt es unter eine Umfrage, einen Vereinsbeschluss, eine Konsent-Runde oder eine Terminfindung.

## Was heute geht

Letzter Tag: noch keiner. Die Version in diesem Baum ist `0.0.1`.

Vier Stimmzettel-Formen: eine Wahl, mehrere gebilligte Wahlen, eine Rangfolge, eine Punktzahl je Auswahl. Zehn Verfahren: Plurality (meiste Stimmen), Majority (mehr als die Hälfte), Supermajority (feste Mindest-Quote), Approval (meiste Zustimmungen), Score (höchste Punktsumme), Schulze (Beatpath), Instant Runoff (Letzter scheidet aus), Borda (Punkte nach Rang), Consent (kein schwerwiegender Einwand; Soziokratie) und Systemisches Konsensieren (geringster Widerstand, 0 bis 10). Quellen stehen in der englischen Tabelle oben.

- Ein leerer Stimmzettel zählt nicht, bei keinem Verfahren. Eine Form, die nicht zum Verfahren passt, ist ein Fehler, nie eine Vermutung.
- Gleichstände werden nie durch Zufall entschieden: alle gleichen Sieger stehen in der Liste, oder es ist nichts entschieden und der Hinweis sagt warum.
- `evaluate_with_held_back` bekommt die Zahl der Antworten, die es gibt, aber noch nicht gezeigt werden. Könnten sie ein entschiedenes Ergebnis ändern, sagt es „noch nicht entschieden“ statt einen Sieger zu nennen.
- Terminregel (`availability`): für eine Terminumfrage mit den Antworten Ja, Wenn nötig, Nein. `count_answers` zählt die drei getrennt, `best_window` schlägt das Fenster mit den meisten Zusagen vor (Ja plus Wenn nötig, ohne Gewichtung), dann mit weniger „Wenn nötig“, dann das früheste; hat niemand zugesagt, schlägt es nichts vor. Ein Mensch entscheidet; es schlägt nur vor. Noch keine `Method` der Registry.
- Tests: das Schulze-Beispiel mit 45 Wählenden aus der Literatur, eine unabhängige Referenz-Umsetzung von Schulze (Pfade ausprobiert) gegen die Bibliothek auf vielen Zufalls-Stimmzettel-Mengen, die Condorcet-Eigenschaft und Fehler-Tests aus der Prüfung.

## Was es nicht tut

- Es speichert nichts und weiß nicht, wer abgestimmt hat. Anonymität und „eine Stimme je Person“ sind Aufgabe der Urnen-Bibliothek `fs-ballot`.
- Wer abstimmen darf, die Mindestgruppe und die Frist stehen nicht hier. Das Programm, das die Bibliothek nutzt, legt sie vor der Abstimmung fest.
- Keine Sitzverteilung (Mehrfach-Sieger-Verfahren), keine Verhältniswahl.

## Teil von FreeSynergy

FreeSynergy ist eine Familie von Programmen, die für alle frei und für immer frei sind und deine Daten bei dir lassen. Dokumentation, Manifest und Gesamtplan liegen in [fs-documentation](https://github.com/FreeSynergy/fs-documentation); die Leseversion ist <https://freesynergy.github.io/fs-documentation/>.

## Für Entwickelnde

- Workspace-Crate: `fs-tally`. Einstiege: `evaluate` und `evaluate_with_held_back`; jedes Verfahren ist eine Strategie hinter dem Trait `DecisionMethod`.
- Lizenz: MIT OR Apache-2.0. Claude (Anthropic) hat beim Schreiben geholfen; die Arbeit wird durch Tests und eine unabhängige Prüfung abgesichert.
- Herkunft: aus dem Crate `fs-forms-methods` von [fs-forms](https://github.com/FreeSynergy/fs-forms) umgezogen (eigener Code, mit den Tests).
