# Progress

Detta är projektets korta operativa återupptagningspunkt. Historisk evidens ligger i `phases/`, `review/`, koden och Git-historiken.

## Nuvarande position

- **Fas:** Fas 2, Rust som designspråk, pågår
- **Konceptenhet:** Enhet 1 och Enhet 2 avslutade; Enhet 3, generics, trait bounds och dispatch, är nästa
- **Steg i inlärningsloopen:** Enhet 3:s första prediction questions om generics och dispatch är aktiva
- **Status:** Enhet 2 klar; Enhet 3 påbörjad
- **Repetition:** Fas 1 har 3 öppna och 4 förstärkta objekt; Fas 2 har 4 öppna och 3 förstärkta objekt

## Senast slutfört

- Enhet 2:s standardtrait-inkrement är slutfört. `JobOperation` och `JobStateKind` använder `Display`, `JobError` använder `Debug`, `Display` och `std::error::Error`, binaryn rapporterar fel genom standardkontraktet och `JobServer::Default` delegerar till `new()` utan att ändra ID-semantiken.
- Slutgrinden passerar: `cargo fmt --check`, `cargo check`, samtliga 26 tester, `cargo clippy --all-targets --all-features`, strikt rustdoc och `cargo run`. Körningen ger oförändrat resultat: Email-jobbet misslyckas efter tre attempts med total retry delay 6.
- Det tidigare öppna `Default`-objektet är förstärkt genom Adams korrekta manuella implementation och beteendetest. Övriga öppna reviewobjekt blockerar inte progression.
- Job servern behåller Unit 1:s library facade och tunna binary. Execution blir nästa verkliga trait-gräns; registry och queue förblir konkreta tills senare behov motiverar abstraktion.

## Nästa konkreta handling

Adam besvarar de fyra aktiva prediction questions utan att köra eller slå upp koden: typrelationer mellan parametrar, monomorfisering, trait-object-representation och dispatchvalet för nuvarande job server. Granska varje svar konkret och fortsätt sedan mentalmodellen med associated types och dyn compatibility.

## Aktuell lärdom

Generics separerar en algoritm eller typ från en konkret implementation, medan trait bounds anger exakt vilket beteende den generiska koden får använda. Static dispatch monomorfiserar använda konkreta typer; `dyn Trait` raderar den konkreta typen bakom en pointer och väljer implementation via en vtable vid runtime. Job serverns nuvarande `Option<u32>` är simulationsstyrning, inte ett execution contract.

## Öppna frågor eller blockerare

Inga blockerare. Fas 2:s fyra öppna objekt prövas genom naturlig användning och blockerar inte progression.

## Beslut som ska bestå

- Job servern förblir synkron genom Fas 2.
- Adam skriver den substantiva projekt- och övningskoden om han inte uttryckligen delegerar den.
- Exakta prediction questions och labbkrav skapas först när respektive enhet är aktiv.
- Efter en första teorigenomgång får Adam utrymme för följdfrågor; prediction questions börjar först när han uttryckligen är redo.
- Inga nya dependencies, persistence-, HTTP- eller async-abstraktioner införs i Fas 2 utan ett separat beslut.
- Registry och queue får inte traits enbart för arkitektonisk symmetri.
