# Progress

Detta är projektets korta operativa återupptagningspunkt. Historisk evidens ligger i `phases/`, `review/`, koden och Git-historiken.

## Nuvarande position

- **Fas:** Fas 2, Rust som designspråk, pågår
- **Konceptenhet:** Enhet 1 och Enhet 2 avslutade; Enhet 3, generics, trait bounds och dispatch, är nästa
- **Steg i inlärningsloopen:** Enhet 2:s avslutskriterium verifierat; Enhet 3:s mentalmodell återstår
- **Status:** Enhet 2 klar; Enhet 3 inte påbörjad
- **Repetition:** Fas 1 har 3 öppna och 4 förstärkta objekt; Fas 2 har 4 öppna och 3 förstärkta objekt

## Senast slutfört

- Enhet 2:s standardtrait-inkrement är slutfört. `JobOperation` och `JobStateKind` använder `Display`, `JobError` använder `Debug`, `Display` och `std::error::Error`, binaryn rapporterar fel genom standardkontraktet och `JobServer::Default` delegerar till `new()` utan att ändra ID-semantiken.
- Slutgrinden passerar: `cargo fmt --check`, `cargo check`, samtliga 26 tester, `cargo clippy --all-targets --all-features`, strikt rustdoc och `cargo run`. Körningen ger oförändrat resultat: Email-jobbet misslyckas efter tre attempts med total retry delay 6.
- Det tidigare öppna `Default`-objektet är förstärkt genom Adams korrekta manuella implementation och beteendetest. Övriga öppna reviewobjekt blockerar inte progression.
- Job servern behåller Unit 1:s library facade och tunna binary. Execution blir nästa verkliga trait-gräns; registry och queue förblir konkreta tills senare behov motiverar abstraktion.

## Nästa konkreta handling

Påbörja Enhet 3:s mentalmodell: generiska funktioner och typer, trait bounds, `where` clauses, `impl Trait`, monomorfisering och static dispatch. Introducera därefter skillnaden mot `dyn Trait`, vtables och object safety utan att ändra projektkod före prediction questions.

## Aktuell lärdom

Standardtraits ersätter ad hoc-metoder när semantiken är gemensam och etablerad: `Display` äger presentation, `Error` placerar domänfelet i Rusts felkontrakt och `Default` uttrycker en kanonisk giltig konstruktion. Derives och manuella implementationer måste motiveras av typens semantik, inte bara av att de kompilerar.

## Öppna frågor eller blockerare

Inga blockerare. Fas 2:s fyra öppna objekt prövas genom naturlig användning och blockerar inte progression.

## Beslut som ska bestå

- Job servern förblir synkron genom Fas 2.
- Adam skriver den substantiva projekt- och övningskoden om han inte uttryckligen delegerar den.
- Exakta prediction questions och labbkrav skapas först när respektive enhet är aktiv.
- Inga nya dependencies, persistence-, HTTP- eller async-abstraktioner införs i Fas 2 utan ett separat beslut.
- Registry och queue får inte traits enbart för arkitektonisk symmetri.
