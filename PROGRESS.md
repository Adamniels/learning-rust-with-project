# Progress

Detta är projektets operativa återupptagningspunkt. Filen ska vara kort och uppdateras när arbetets faktiska tillstånd förändras.

## Nuvarande position

- **Fas:** Fas 1, Rust som språk
- **Konceptenhet:** Enhet 3, borrowing, slices och metoder
- **Steg i inlärningsloopen:** Mental modell är nästa steg
- **Status:** Enhet 2 är avslutad och verifierad
- **Repetition:** 4 öppna objekt, inget ska repeteras omedelbart

## Senast slutfört

- Enhet 0 etablerade och verifierade Rusts toolchain, Cargo-modell och `job-server`-package.
- Enhet 1:s teori och förutsägelser är genomförda. Mikrolabbet hoppades över efter svårighetskalibrering.
- Job servern simulerar ett synkront jobb med högst ett konfigurerat antal försök och retry delay före varje retry.
- Simulatorns tabelltest täcker noll försök, framgång på försök 1–3 och failure när maxgränsen nås.
- `cargo fmt --check`, `cargo check`, `cargo test` och `cargo run` passerar; körningen ger `(3, false, 6)` för `simulate_job(3, 4)`.
- Enhet 1 uppfyller sitt avslutskriterium och är markerad som klar i roadmapen.
- Enhet 2:s första ownershipförutsägelser gav fem klara svar av sex. Moves, `Copy`, `Clone`, scopes, funktionsargument och partial moves förklarades korrekt.
- Ett separat mikrolabb hoppas över eftersom svaren redan visar tillräcklig förståelse; ownership prövas direkt i projektinkrementet.
- `Job` äger sin `String`-payload och konsumeras av `simulate_job`, som flyttar jobbet vidare till `SimulationResult` utan `clone()`.
- Ett reducerat tabelltest täcker fyra skilda kontrollflöden och ett separat test verifierar att jobbdata bevaras i resultatet.
- `cargo fmt --check`, `cargo check`, `cargo test` och `cargo run` passerar. Körningen ger `send-email, 3, false, 6`.
- Enhet 2 uppfyller sitt avslutskriterium: ownership, moves, `Copy`, `Clone`, scopes och partial moves har förklarats och använts i projektkod. Ett separat compiler-error-experiment hoppades över eftersom use-after-move redan behärskas.

## Nästa konkreta handling

Starta enhet 3 med mentalmodellen för borrowing: varför lån behövs, `&T` och `&mut T`, aliasingreglerna, reborrowing på grundnivå, slices samt skillnaden mellan `self`, `&self` och `&mut self`.

## Aktuell lärdom

Ett funktionsargument by value flyttar `Job` till parametern. Funktionen kan därefter flytta samma jobb vidare till `SimulationResult`, så anroparen återfår åtkomst genom `result.job`, inte genom den tidigare och nu invaliderade bindningen `job`.

## Öppna frågor eller blockerare

Inga.

## Beslut som ska bestå

- Job servern börjar som en synkron domänkärna utan nätverk.
- Persistence, HTTP och async introduceras först när deras språkliga förkunskaper finns.
- Huvudprojektet bär merparten av lärandet. Isolerade labbar används när det ger bättre förståelse.
- Adam skriver huvuddelen av övnings- och projektkoden. Codex ska inte automatiskt implementera lärmoment åt honom.
- Kodmappar och arkitektur skapas när de behövs, inte som tom spekulation i förväg.

## Innan ett arbetstillfälle avslutas

Uppdatera vid behov:

1. nuvarande position,
2. senast slutfört,
3. nästa konkreta handling,
4. aktuell lärdom,
5. öppna frågor eller blockerare.
