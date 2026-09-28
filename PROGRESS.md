# Progress

Detta är projektets korta operativa återupptagningspunkt. Historisk evidens ligger i `phases/`, `review/`, koden och Git-historiken.

## Nuvarande position

- **Fas:** Fas 2, Rust som designspråk, pågår
- **Konceptenhet:** Enhet 1 och Enhet 2 avslutade; Enhet 3, generics, trait bounds och dispatch, pågår
- **Steg i inlärningsloopen:** Enhet 3:s mentalmodell fortsätter med associated types och dyn compatibility
- **Status:** Enhet 2 klar; Enhet 3 påbörjad
- **Repetition:** Fas 1 har 3 öppna och 4 förstärkta objekt; Fas 2 har 6 öppna och 3 förstärkta objekt

## Senast slutfört

- Enhet 2:s standardtrait-inkrement är slutfört. `JobOperation` och `JobStateKind` använder `Display`, `JobError` använder `Debug`, `Display` och `std::error::Error`, binaryn rapporterar fel genom standardkontraktet och `JobServer::Default` delegerar till `new()` utan att ändra ID-semantiken.
- Slutgrinden passerar: `cargo fmt --check`, `cargo check`, samtliga 26 tester, `cargo clippy --all-targets --all-features`, strikt rustdoc och `cargo run`. Körningen ger oförändrat resultat: Email-jobbet misslyckas efter tre attempts med total retry delay 6.
- Det tidigare öppna `Default`-objektet är förstärkt genom Adams korrekta manuella implementation och beteendetest. Övriga öppna reviewobjekt blockerar inte progression.
- Job servern behåller Unit 1:s library facade och tunna binary. Execution blir nästa verkliga trait-gräns; registry och queue förblir konkreta tills senare behov motiverar abstraktion.
- Enhet 3:s första fyra predictions är granskade. Samma-typkravet för `&E`, två monomorfiserade executorvarianter och projektets val av static dispatch identifierades; `impl Trait` sammanblandades med dynamic dispatch och `&dyn Trait` med obligatorisk heapallokering.

## Nästa konkreta handling

Fördjupa associated types som en entydig relation mellan en trait-implementation och dess outputtyp, samt dyn compatibility som kravet att ett trait-object-anrop kan beskrivas utan att känna konkret `Self`. Ge utrymme för Adams följdfrågor innan nästa prediction-runda eller dispatch-labb.

## Aktuell lärdom

`impl Trait` i parameterposition är generic och använder static dispatch; separata förekomster motsvarar separata anonyma type parameters. Anrop och monomorfiserade instansieringar är olika saker. `&dyn Trait` använder en data-pointer och vtable-pointer för runtime dispatch men lånar bara sitt värde och orsakar inte i sig heapallokering.

## Öppna frågor eller blockerare

Inga blockerare. Fas 2:s sex öppna objekt prövas genom naturlig användning och blockerar inte progression.

## Beslut som ska bestå

- Job servern förblir synkron genom Fas 2.
- Adam skriver den substantiva projekt- och övningskoden om han inte uttryckligen delegerar den.
- Exakta prediction questions och labbkrav skapas först när respektive enhet är aktiv.
- Efter en första teorigenomgång får Adam utrymme för följdfrågor; prediction questions börjar först när han uttryckligen är redo.
- Inga nya dependencies, persistence-, HTTP- eller async-abstraktioner införs i Fas 2 utan ett separat beslut.
- Registry och queue får inte traits enbart för arkitektonisk symmetri.
