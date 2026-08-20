# Progress

Detta är projektets operativa återupptagningspunkt. Filen ska vara kort och uppdateras när arbetets faktiska tillstånd förändras.

## Nuvarande position

- **Fas:** Fas 1 avslutad; Fas 2 är nästa men inte påbörjad
- **Konceptenhet:** Enhet 7, konsolidering av den synkrona kärnan, avslutad
- **Steg i inlärningsloopen:** Fasens kvalitetsgrind verifierad
- **Status:** Enhet 7 och Fas 1 är avslutade
- **Repetition:** 3 öppna och 4 förstärkta objekt, inget ska repeteras omedelbart

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
- Enhet 6:s fyra första förutsägelsefrågor besvarades klart. `Option` valdes utifrån ett explorativt kontrakt, moves ur `Result` följdes korrekt, `?` förklarades som tidig retur och ett saknat registerjobb bakom ett privat kö-ID klassificerades korrekt som en bruten invariant.
- Ett separat mikrolabb hoppas över eftersom svaren visar tillräcklig förståelse; `Result`, `?` och felklassificeringen prövas direkt i projektinkrementet.
- Enhet 6-implementationen har infört `JobOperation`, `JobStateKind`, `JobError`, `Result`-baserade transition methods, `next_queued`, `process_next` och ett omskrivet `main`; verifiering och testanpassning återstår.
- Adam implementerar ett enda diagnostiskt test för ett köat jobb med `max_attempts = 0`. Codex implementerar övriga överenskomna tester efter att detta test har granskats.
- Adams boundary-test matchar rätt `AttemptsExhausted`-variant och verifierar att state samt counter förblir oförändrade; Codex lade till det saknade `#[test]`-attributet och anpassade samtliga äldre tester till de nya `Result`-kontrakten.
- `cargo fmt --check` och `cargo check` passerar. Fem av sju tester passerar; failure-simuleringen och noll-attempt-testet exponerar två produktionsfel. Tre dead-code-varningar kvarstår för ännu oanvänd feldata.
- Adam korrigerade noll-attempt-klassificeringen och simulatorns terminala failure-avslut; de tidigare två felande testerna passerar nu.
- Codex lade till de sex återstående Unit 6-testerna för upprepat begin, ogiltiga completion-transitioner, FIFO genom `process_next`, tom process-kö och den avsiktliga kö/register-invariantpaniken.
- `cargo fmt --check` och `cargo check` passerar. Elva av tretton tester passerar; success- och failure-completion från `Queued` panikerar fortfarande i stället för att returnera `InvalidTransition`. Två binary-varningar kvarstår för feldata som ännu inte läses i `main`.
- Adam korrigerade båda completion methods så att `Queued` ger `InvalidTransition` utan mutation. `cargo fmt --check`, `cargo check`, alla tretton tester och `cargo run` passerar; körningen ger det förväntade terminala Email-resultatet med delay 6.
- Slutgranskningen fann tre kvarvarande kvalitetsfrågor: `main` panikerar på legitima `QueueEmpty` och `InvalidTransition`, `process_next` tar bort kö-ID:t innan en fallibel simulering och kan därför lämna ett `Queued` jobb utanför kön, samt state-till-kind-mappningen är duplicerad i stället för centraliserad. Binary-builden varnar därför fortfarande för oläst feldata.
- Codex färdigställde på Adams begäran exhaustive `JobError`-hantering i `main`, lade till textrepresentationer för operation och state kind samt centraliserade den ägda state-discriminanten i `JobState::kind`. Kösemantiken ändrades inte.
- `cargo fmt --check`, `cargo check`, alla tretton tester och `cargo run` passerar utan varningar; körningen ger det förväntade terminala Email-resultatet med delay 6.
- `next_queued` observerar nu front-ID:t med `VecDeque::front` utan att ta bort det. `process_next` tar bort ID:t först efter lyckad simulering, så ett returnerat fel lämnar både jobb och kö konsistenta.
- Ett regressionstest verifierar att ett misslyckat `process_next` behåller jobbet längst fram. `cargo fmt --check`, `cargo check`, samtliga fjorton tester, `cargo run` och `cargo clippy --all-targets --all-features` passerar utan varningar.
- Enhet 6 uppfyller sitt avslutskriterium: domänfel modelleras som data, `?` propagerar fel utan att förlora kö/state-konsistens och brutna privata invarianter skiljs från legitima felresultat.
- Enhet 7:s mentalmodell rekonstruerar kärnan som en ägarhierarki, en tillståndsmaskin och ett felprotokoll. Fyra frågor prövar borrowing, partial moves, exklusiv collection-iteration och exhaustive error handling.
- Enhet 7:s fyra konsolideringsfrågor gav två klara svar och två kvarvarande borrowingluckor. Partial moves och wildcard-armens effekt på exhaustive error handling återkallades korrekt; lån härledda ur `&mut JobServer` och `iter_mut`-iteratorns exklusiva collectionlån behöver senare tillämpning.
- Adam implementerade cancellation och testet för det mittersta köade jobbet. Borrowing mellan köiteration, job mutation och strukturell kömutation är korrekt avgränsad och förstärker två reviewobjekt.
- `cargo fmt --check` passerar. `cargo check`, samtliga femton tester och `cargo run` slutförs, men build och körning har dead-code-varningar eftersom cancellation inte används i binary-flödet. Granskningen fann även felaktig valideringsordning och saknade state/reason-assertions i testet.
- Codex korrigerade på Adams begäran den fullständiga cancellation-valideringen och kompletterade det befintliga testet med ID-, `get`-, `Cancelled`- och reason-assertions. `cargo fmt --check` och samtliga femton tester passerar; `cargo check`, `cargo run` och Clippy slutförs med fem dead-code-varningar tills cancellation används i `main`.
- Codex integrerade på Adams begäran cancellation i binary-flödet, centraliserade exhaustive `JobError`-rapportering och lade till sju tester för `JobNotFound`, `InvalidTransition`, terminal cancellation och fyra brutna kö/register-invarianter.
- Unit 7:s och Fas 1:s slutversion passerar `cargo fmt --check`, `cargo check`, samtliga tjugotvå tester, `cargo run` och `cargo clippy --all-targets --all-features` utan varningar. Unit 7 och Fas 1 uppfyller sina avslutskriterier.

## Nästa konkreta handling

När Adam vill fortsätta: planera Fas 2:s första enhet utifrån roadmapen och evidensen från Fas 1; Fas 2 ska inte påbörjas innan dess.

## Aktuell lärdom

Den synkrona kärnan har en tydlig ägarhierarki: registret äger jobben, kön äger kopierbara ID:n, transition methods skyddar state och application boundary hanterar legitima `JobError` exhaustively. Panic används endast för brutna privata invarianter.

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
