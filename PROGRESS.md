# Progress

Detta är projektets operativa återupptagningspunkt. Filen ska vara kort och uppdateras när arbetets faktiska tillstånd förändras.

## Nuvarande position

- **Fas:** Fas 1, Rust som språk
- **Konceptenhet:** Enhet 5, collections och iteration
- **Steg i inlärningsloopen:** Mental modell är nästa steg
- **Status:** Enhet 4 är avslutad och verifierad
- **Repetition:** 5 öppna objekt, inget ska repeteras omedelbart

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
- Enhet 3:s första förutsägelser gav tre klara svar av fyra. Delade lån, flera läsare och lånets slut vid sista användning förklarades korrekt.
- Ett separat mikrolabb hoppas över; skillnaden mellan ett lånat `&str` och owned `String` prövas genom metoder direkt i projektinkrementet.
- `Job` äger nu sin attempt-räknare och exponerar observerande metoder med `&self`, en muterande metod med `&mut self` och payloaden som lånad `&str`.
- `simulate_job` lånar ett jobb exklusivt under körningen och behöver inte längre konsumera eller returnera det. Anroparen använder jobbet igen efter att lånet avslutats.
- Unit 3-implementationen passerar `cargo fmt --check`, `cargo check`, två tester och `cargo run`; körningen ger `send-email, 3, false, 6`.
- Enhet 3 uppfyller sitt avslutskriterium och är markerad som klar i roadmapen.
- Enhet 4:s första förutsägelser gav tre klara svar av fyra. Associated data, exhaustiveness och ownership vid matching by value respektive reference förklarades korrekt.
- Tillståndsfrågan besvarades med en giltig ogiltig boolkombination men bara ett av två efterfrågade exempel; inget separat reviewobjekt eller mikrolabb behövs.
- `JobKind` och `JobState` representerar kind samt `Queued`, `Running`, `Succeeded` och `Failed` med state-specifik data.
- Simulatorn använder `Option<u32>` för planerad success, explicita transition methods och en lånad `&mut Job` utan boolskt domäntillstånd eller sentinelvärdet `0`.
- Tre tester täcker `Queued → Running → Queued`, success på attempt 2 och terminal failure. `cargo fmt --check`, `cargo check`, `cargo test` och `cargo run` passerar; den enda varningen är att `Cleanup` ännu inte konstrueras.
- Enhet 4 uppfyller sitt avslutskriterium och är markerad som klar i roadmapen.

## Nästa konkreta handling

Starta enhet 5 med mentalmodellen för collections och iteration: ägarskap av element i `Vec`, `VecDeque` och `HashMap`, åtkomstmönster, samt skillnaden mellan `iter`, `iter_mut` och konsumerande `into_iter`.

## Aktuell lärdom

En enum gör terminala och icke-terminala jobbtillstånd ömsesidigt uteslutande. Associated output och error ägs endast av de variants där datan är giltig, och `Option<u32>` uttrycker frånvarande successplan utan ett magiskt tal.

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
