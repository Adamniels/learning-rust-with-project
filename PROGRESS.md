# Progress

Detta är projektets operativa återupptagningspunkt. Filen ska vara kort och uppdateras när arbetets faktiska tillstånd förändras.

## Nuvarande position

- **Fas:** Fas 1, Rust som språk
- **Konceptenhet:** Enhet 6, `Result`, felmodellering och felpropagering
- **Steg i inlärningsloopen:** Mental modell genomgången, förutsägelsefrågor är nästa steg
- **Status:** Enhet 5 är avslutad och verifierad
- **Repetition:** 6 öppna objekt, inget ska repeteras omedelbart

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
- Enhet 5:s mentalmodell har introducerat hur `Vec<T>`, `VecDeque<T>` och `HashMap<K, V>` äger sina element, hur deras åtkomstmönster skiljer sig och hur `iter`, `iter_mut` och `into_iter` påverkar lån och ownership.
- Enhet 5:s första fyra förutsägelsefrågor gav två klara svar, ett delvis korrekt svar och en missuppfattning. Delad iteration och konsumerande iteration förklarades korrekt; den kvarvarande luckan gäller att `iter_mut` lånar collectionen exklusivt och förhindrar samtidig strukturell mutation.
- Det separata collection-labbet hoppades över efter svårighetskalibrering. Borrowingregeln prövas i stället genom projektinkrementets lån av jobb ur registret.
- Första versionen av enhet 5:s projektinkrement har ett `HashMap`-register, en separat `VecDeque` i FIFO-ordning, delad iteration över ID:n och ett `next_queued` som returnerar ett mutabelt lån till ett fortsatt registerägt jobb.
- `cargo fmt --check`, `cargo check`, sex tester och `cargo run` passerar; körningen ger det förväntade terminala Email-resultatet. `cargo check` och `cargo run` varnar för att `get` inte används i binary-flödet.
- Granskningen fann att `submit` ännu inte returnerar det tilldelade ID:t, att testerna därför inte verifierar returkontraktet, samt att `next_queued` använder `?` före enhet 6 där operatorn introduceras.
- Den uppdaterade implementationen returnerar och verifierar ettbaserade ID:n. Codex ersatte därefter `?` med explicit `match` och korrigerade FIFO-testets missvisande namn på Adams uttryckliga begäran.
- Enhet 5:s slutversion passerar `cargo fmt --check`, `cargo check`, sex tester och `cargo run`. Den enda varningen är att `get` ännu inte används i binary-flödet.
- Enhet 5 uppfyller sitt avslutskriterium: registret och kön har separata ansvar, delad iteration konsumerar inte kön, och jobb muteras genom lån utan kloner eller förlorat registerägarskap.
- Enhet 6:s mentalmodell har introducerat `Result<T, E>`, skillnaden mellan frånvaro och fel, `?` som tidig retur samt gränsen mellan domänfel och brutna interna invarianter.

## Nästa konkreta handling

Besvara enhet 6:s fyra första förutsägelsefrågor om `Option` kontra `Result`, ägarskap i `Ok` och `Err`, kontrollflödet för `?` samt klassificering av domänfel och brutna invarianter utan att köra koden.

## Aktuell lärdom

`Result<T, E>` gör både framgångsvärdet och felinformationen explicita och ägda av varsin enumvariant. `?` packar upp `Ok`, men returnerar tidigt vid `Err`; `Option` ska användas för informationslös frånvaro, inte när anroparen behöver veta varför operationen misslyckades.

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
