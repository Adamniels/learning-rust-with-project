# Fas 2: Rust som designspråk

## Syfte

Fasen formar Fas 1:s fungerande synkrona kärna till en modulär, testbar och uttrycksfull Rust-design. Fokus flyttas från enskilda språkregler till hur moduler, traits, generics, iterators och lifetimes tillsammans uttrycker ansvar och beroenden.

Fas 1 slutade med en enda binary crate där domänmodell, orchestration, application boundary och 22 tester ligger i `src/main.rs`. `HashMap` och `VecDeque` har tydliga roller men ägs direkt av `JobServer`, och execution styrs fortfarande genom simulationsparametern `Option<u32>`. Dessa konkreta begränsningar är Fas 2:s utgångspunkt.

Exakta prediction questions, labbkrav och lösningar skapas först när respektive enhet blir aktiv. Planen definierar beroenden och önskade resultat, inte färdig implementation.

## Resultat efter fasen

Job servern ska bestå av en library crate och en tunn binary där:

- en offentlig facade exponerar ett litet och avsiktligt API,
- privata moduler äger domänregler, orchestration och in-memory state,
- starka typer ersätter råa ID:n och otydliga tuples vid API-gränser,
- ett execution contract uttrycks med en trait och används genom motiverad dispatch,
- read-only queries kan uttryckas med lazy iterators och korrekta lifetime-relationer,
- unit tests skyddar privata regler och integration tests verifierar det publika kontraktet,
- domänkärnan inte känner till binary presentation eller testscaffolding.

Fasen introducerar inte persistence, HTTP, async, parallella workers eller externa ramverk. `HashMap` och `VecDeque` förblir in-memory implementationer.

## Designbeslut för fasen

- Börja med en library boundary innan fler abstraktioner införs. Ett publikt API går inte att bedöma meningsfullt medan allt ligger i `main.rs`.
- Execution får den första egna trait-gränsen eftersom tester och binary redan behöver olika beteenden.
- Registry och queue separeras som konkreta ansvar, inte som traits. Repository- och queue-traits väntar tills Fas 3 ger minst en alternativ implementation; tidigare abstraction vore spekulativ.
- Static dispatch är standard när implementationen är känd vid compile time. Trait objects används i projektet endast om ett verkligt runtime-behov uppstår; skillnaden prövas ändå isolerat.
- Lifetimes används för att beskriva existerande lån. En annotation får aldrig behandlas som ett sätt att förlänga datas livslängd.
- Moduler skapas runt sammanhängande ansvar, inte mekaniskt en fil per typ.
- Inga nya dependencies tillkommer utan ett separat beslut när en aktiv enhet visar ett konkret behov.

## Arbetsmodell för varje enhet

Varje enhet följer repositoryts vanliga lärloop:

1. Mental modell och avgränsad teori
2. Högst fyra fokuserade prediction questions åt gången
3. Mikrolabb endast när en Rust-specifik osäkerhet behöver isoleras
4. Ett sammanhängande projektinkrement beskrivet som krav
5. Compiler-driven debugging av verkliga diagnostik
6. Test och idiomatisk refaktorering
7. Uppdatering av progress och reviewevidens

Öppna reviewobjekt från Fas 1 återkommer genom naturlig användning. Cargo targets passar särskilt i enhet 1; blockvärden samt tuple- och arraytyper återkallas när projektkod eller tester ger ett meningsfullt tillfälle.

## Enhet 1: Crates, moduler, synlighet och library boundary

### Koncept

- Package, library target, binary target och crate roots
- Modulträdet, `mod`, `use`, absoluta och relativa paths
- Privat som standard samt `pub`, `pub(crate)` och re-exports
- `lib.rs` som publik facade och `main.rs` som composition root
- API-yta som ett medvetet designbeslut
- Grundläggande rustdoc för publika items

### Prediction och labbfokus

Förutsäg vilka paths och items som är synliga över modul- och crate-gränser, samt vilka targets Cargo bygger från `src/lib.rs` och `src/main.rs`. Ett isolerat labb används bara om crate/target-modellen fortfarande är osäker efter prediction answers.

### Projektinkrement

Inför en library target utan beteendeförändring. Flytta domän- och serverkod ur `main.rs` till minsta antal sammanhängande moduler, låt `lib.rs` exponera en liten facade och gör binaryn till en tunn anropare av library-API:t. Fält förblir privata. Tester får tillfälligt ligga nära implementationen tills testarkitekturen behandlas i enhet 7.

Ingen tom lagerhierarki eller framtida adapterstruktur skapas i denna enhet.

### Avslutskriterium

Kunna rita crate- och modulträdet, förklara varför varje publikt item måste vara publikt och visa att library och binary är separata targets med egna crate roots.

## Enhet 2: Traits och standardkontrakt

### Koncept

- Traits som namngivna beteendekontrakt
- Inherent methods jämfört med trait methods
- `impl Trait for Type`, method resolution och coherence
- Orphan rule på den nivå API-design kräver
- `derive` och vanliga kontrakt som `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` och `Hash`
- `Display` och `std::error::Error` för fel vid systemgränser

### Prediction och labbfokus

Förutsäg vilken implementation ett metodanrop väljer, när ett bound saknas och vilka trait-implementationer orphan rule tillåter. Labbet jämför en inherent method med en standardtrait när skillnaden inte blir tydlig i projektet.

### Projektinkrement

Ersätt ad hoc-textmetoder för operationer, state kinds och errors med motiverade standardtrait-implementationer. Härled endast traits vars semantik verkligen stämmer. Binaryn ska kunna rapportera ett `JobError` genom dess publika kontrakt utan att känna till privata representationsdetaljer.

### Avslutskriterium

Kunna skilja data från beteendekontrakt, motivera varje derive eller manuell implementation och förklara varför en trait är bättre än ännu en inherent helper i det aktuella fallet.

## Enhet 3: Generics, trait bounds och dispatch

### Koncept

- Generiska funktioner och typer
- Trait bounds, `where` clauses och `impl Trait`
- Monomorfisering och static dispatch
- Trait objects, `dyn Trait`, vtables och object safety på grundnivå
- Associated types när ett kontrakt har en naturlig outputrelation
- Compile-time polymorphism jämfört med runtime polymorphism

### Prediction och labbfokus

Förutsäg monomorfiserade varianter, vilka bounds en implementation behöver och när en trait kan användas som trait object. Ett kompakt labb jämför generic dispatch med `dyn Trait`; projektet ska inte använda båda utan ett verkligt skäl.

### Projektinkrement

Inför ett `JobExecutor`-liknande execution contract och ersätt `Option<u32>` som produktions- och teststyrning. Processing tar emot ett executor-beteende genom static dispatch. Tester får en deterministisk fake, medan binaryn använder en konkret synkron implementation.

Den exakta attempt-inputen och execution-outputen definieras när enheten är aktiv, så att kontraktet följer den dåvarande domänmodellen i stället för att planeras fram i detalj nu.

### Avslutskriterium

Kunna förklara vilken kod som monomorfiseras, varför executor-gränsen är en trait och varför static eller dynamic dispatch valdes i projektet.

## Enhet 4: Domäninvarianter och idiomatisk API-design

### Koncept

- Newtypes och semantisk typseparation
- Commands, queries och explicit metodansvar
- Privata representationer och publika observationer
- Named return types jämfört med otydliga tuples
- Validering före mutation och tydliga commit points
- Felens abstraktionsnivå och vilka detaljer anroparen behöver
- När typdesign kan förhindra ett ogiltigt tillstånd och när runtime-validering krävs

### Prediction och labbfokus

Jämför API:n som accepterar råa primitives med API:n som använder domäntyper. Förutsäg vilka invariantbrott som fortfarande är möjliga när fält eller mutabla referenser exponeras.

### Projektinkrement

Inför en `JobId`-newtype och ett namngivet processing outcome i stället för råa `u64` och retur-tuples vid den publika gränsen. Granska commands och queries så att externa anropare inte får ett `&mut Job` som kan kringgå serverns kö/state-invarianter. Behåll endast de observationer och operationer som det publika API:t behöver.

### Avslutskriterium

Kunna försvara varje publik signatur, visa vilka felaktiga anrop som blivit omöjliga eller explicita och peka ut den enda ägaren för varje invariant.

## Enhet 5: Closures och iterators som databehandling

### Koncept

- `Iterator` och `next` som grundmodell
- Laziness, adapters och consuming operations
- `map`, `filter`, `find`, `position`, `any`, `all`, `collect` och `fold` efter behov
- Closure captures genom shared borrow, mutable borrow och move
- `Fn`, `FnMut` och `FnOnce` på användningsnivå
- Ägda, delade och mutabla iterator items
- När en loop är tydligare än en iterator pipeline

### Prediction och labbfokus

Förutsäg closure captures, iterator-itemtyper och när en pipeline faktiskt körs. Jämför en loop och en pipeline utifrån semantik och borrowing, inte radantal.

### Projektinkrement

Granska cancellation-sökningen och övriga collectionoperationer med full förståelse för deras iterator- och closuretyper. Inför eller refaktorera en read-only query över jobb där adapters förbättrar semantiken. Resultatet får samlas till en ägd collection när API-kontraktet verkligen beskriver en snapshot; lazy borrowing skjuts till nästa enhet.

### Avslutskriterium

Kunna ange itemtypen efter varje relevant adapter, förklara closure-capture och välja loop, lazy iterator eller eager collection med avsikt.

## Enhet 6: Lifetimes som relationer

### Koncept

- Lifetimes som relationer mellan referensers giltighet
- Lifetime elision och vad kompilatorn kan inferera
- Explicita lifetime parameters för flera input- och outputreferenser
- Lifetime bounds och `'_` i returtyper
- `'static` för data respektive referenser
- Structs som lånar data jämfört med structs som äger data
- Varför annoteringar inte förlänger ett värdes livslängd

### Prediction och labbfokus

Förutsäg vilken inputreferens en output kan vara bunden till och varför vissa signaturer är tvetydiga. Ett isolerat compiler-error-labb används om lifetime annotations annars bara blir syntax utan mental modell.

### Projektinkrement

Inför en lazy read-only query som returnerar en iterator bunden till `&self`, exempelvis över registerägda jobb eller filtrerade jobb-ID:n. Signaturen ska göra lånerelationen explicit där elision eller `impl Iterator + '_` kräver det, och tester ska visa att servermutation måste vänta tills iteratorns sista användning.

Ingen struct får lagra en referens endast för att undvika ett motiverat owned value.

### Avslutskriterium

Kunna läsa en explicit lifetime som en constraint mellan referenser, förklara varför den returnerade iteratorn inte kan överleva servern och avgöra när ett owned resultat är ett bättre API.

## Enhet 7: Testdesign och systemgränser

### Koncept

- Unit tests jämfört med integration tests i Rusts package-modell
- `#[cfg(test)]`, privata implementationstester och katalogen `tests/`
- Tester genom den publika facaden
- Deterministiska fakes genom traits utan mocking framework
- Normalt flöde, boundaries, legitima errors och brutna interna invarianter
- Behavioral assertions jämfört med implementation coupling
- Table-driven tests och när varje fall förtjänar ett eget namn

### Prediction och labbfokus

Klassificera tester efter vilken gräns de skyddar och vilka privata detaljer de får observera. Förutsäg vilka items ett integration test kan importera från library crate.

### Projektinkrement

Fördela den befintliga testsviten efter ansvar. Privata state transitions och avsiktliga invariantpaniker stannar som unit tests nära sina moduler. Publika submit, get, process och cancel-flöden verifieras genom integration tests som endast använder library-facaden och en deterministisk executor fake.

Ingen extern test- eller mockingdependency introduceras om standardbiblioteket och lokala fakes räcker.

### Avslutskriterium

Kunna motivera varje tests nivå, visa att integration tests inte når privata fält och ändra intern representation utan att publika behavior tests behöver skrivas om.

## Enhet 8: Ansvarseparation och konsolidering

### Koncept

- Domain, application orchestration och adapters som ansvar, inte mappar för sin egen skull
- Dependency direction och composition root
- Concrete dependency jämfört med abstraction
- Abstraction cost, cohesion och coupling
- Refaktorering som bevarar beteende
- API review över ownership, dispatch, lifetimes och errors

### Prediction och labbfokus

Placera ansvar i rätt del av ett litet system och identifiera både sammanblandning och överengineering. Ingen separat mikrolabb krävs om projektgranskningen ger tillräcklig evidens.

### Projektinkrement

Färdigställ Fas 2-arkitekturen. `Job` äger domäntillstånd och transitions. `JobServer` orkestrerar use cases. Registry och FIFO queue får konkreta in-memory ansvar med minimala API:n. Executor-traiten utgör execution-porten. Binaryn skapar implementationerna och hanterar presentation.

Registry och queue görs inte generiska och får inga traits enbart för symmetri. Om refaktoreringen visar att en separat wrapper inte förbättrar invariants eller cohesion ska den inte skapas.

### Avslutskriterium

Kunna rita dependency direction, försvara vilka gränser som är konkreta respektive trait-baserade och genomföra en mindre intern förändring utan att domänregler eller publika tests läcker mellan lager.

## Fasens kvalitetsgrind

Innan Fas 3 detaljplaneras ska:

- `cargo fmt --check`, `cargo check`, `cargo test` och `cargo clippy --all-targets --all-features` passera utan varningar,
- `cargo doc --no-deps` bygga den publika library-facaden,
- binaryn endast ansvara för composition och presentation,
- publika fält och metoder vara minsta motiverade API-yta,
- integration tests använda endast publika items,
- inga onödiga `clone()`, `panic!`, trait objects eller generiska lager dölja oklara ansvar,
- varje trait ha minst ett konkret beteendeskäl,
- lifetime annotations kunna förklaras som relationer, inte livstidsförlängning,
- registry-, queue- och executoransvar vara separerade utan spekulativa persistence-abstraktioner,
- Adam kunna försvara API-, dispatch- och testgränser utan att enbart hänvisa till kompilatorn,
- kvarvarande reviewobjekt vara aktuella och schemalagda utan att behandlas som betyg.
