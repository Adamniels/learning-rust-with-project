# Roadmap

Roadmapen beskriver en beroendeordning mellan kunskaper, inte en tidsplan. En fas är färdig när koncepten kan förklaras och användas självständigt i relevant kod. Vi går tillbaka när en senare uppgift avslöjar en lucka.

Statusmarkörer:

- `[ ]` Inte påbörjad
- `[~]` Pågår
- `[x]` Behärskad på den nivå nästa fas kräver

## Fas 1: Rust som språk

Mål: bygga en synkron job server-kärna och förstå hur Rust representerar data, ägarskap och fel.

- [x] Enhet 0: Verktygskedjan och kompileringsmodellen
- [x] Enhet 1: Bindningar, typer, uttryck, funktioner och kontrollflöde
- [x] Enhet 2: Ownership, ägd data och structs
- [~] Enhet 3: Borrowing, slices och metoder
- [ ] Enhet 4: Enums, pattern matching, `Option` och tillståndsmodellering
- [ ] Enhet 5: Collections och iteration
- [ ] Enhet 6: `Result`, felmodellering och felpropagering
- [ ] Enhet 7: Konsolidering av den synkrona kärnan

Varje enhet kombinerar teori, förutsägelse, mikrolabb och ett projektinkrement. Den [detaljerade fasplanen](phases/01-rust-as-language.md) beskriver beroendeordningen och vad varje inkrement tillför.

Utgångskriterium: kunna förklara vem som äger varje centralt värde, välja mellan move och borrow med avsikt samt modellera jobbens tillstånd och fel utan ogiltiga mellanlägen.

## Fas 2: Rust som designspråk

Mål: forma en modulär och testbar kärna där abstraktionerna uttrycker verkliga ansvar.

- [ ] Moduler, crates, synlighet och publika API:er
- [ ] Traits och trait bounds
- [ ] Generics, monomorfisering och när trait objects är motiverade
- [ ] Iterators, closures och funktionell databehandling
- [ ] Lifetimes som relationer mellan referensers giltighet
- [ ] Testdesign, integrationstester och testbara systemgränser
- [ ] Domäninvarianter och idiomatisk API-design
- [ ] Separera domänkärnan från kö-, lagrings- och exekveringsadaptrar

Utgångskriterium: kunna utforma gränser som är lätta att testa, motivera statisk respektive dynamisk dispatch och använda lifetimes utan att behandla annoteringar som försök att förlänga datas livslängd.

## Fas 3: Rust som backendplattform

Mål: göra systemet persistent och åtkomligt över HTTP utan att domänkärnan blir beroende av infrastrukturen.

- [ ] Serialisering och deserialisering
- [ ] Konfiguration och applikationssammansättning
- [ ] Filbaserad persistence som första adapter
- [ ] Databaspersistence när dess nya koncept motiverar beroendet
- [ ] HTTP och API-design
- [ ] Axum handlers, extractors och felmappning
- [ ] Integrationstester över systemgränser
- [ ] Observability: strukturerad loggning och relevanta mätpunkter

Utgångskriterium: kunna följa ett jobb genom transport-, applikations-, domän- och persistencelagren samt förklara var validering och felöversättning hör hemma.

Nya externa beroenden och valet av databas fattas när vi når respektive enhet, inte i förväg.

## Fas 4: Rust under samtidighet

Mål: köra flera workers korrekt och förstå garantierna bakom async och trådsäkerhet.

- [ ] Futures, `async`/`await` och Tokios exekveringsmodell
- [ ] Tasks och strukturerad livscykelhantering
- [ ] Channels och message passing
- [ ] Delat tillstånd med `Arc` och synkroniseringsprimitiver
- [ ] `Send`, `Sync` och vad kompilatorn faktiskt garanterar
- [ ] Timeouts, cancellation och retries med definierad policy
- [ ] Graceful shutdown
- [ ] Deterministiska tester av samtidiga beteenden

Utgångskriterium: kunna resonera om jobbägarskap mellan tasks, välja mellan message passing och delat tillstånd samt visa att shutdown, retries och avbrott inte tappar eller dubbelbehandlar jobb utanför systemets uttryckliga garantier.

## Tvärgående mikrolabbar

Följande ämnen får egna labbar när deras förkunskaper finns. De pressas inte artificiellt in i huvudprojektet:

- Smarta pekare och interior mutability
- Makron
- Atomics
- `unsafe` och säkerhetsinvarianter
- Property-baserade tester

Roadmapen förfinas en fas i taget. Vi detaljplanerar inte senare implementation innan tidigare faser har lärt oss vilka gränser systemet faktiskt behöver.
