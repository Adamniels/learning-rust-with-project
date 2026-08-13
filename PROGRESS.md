# Progress

Detta är projektets operativa återupptagningspunkt. Filen ska vara kort och uppdateras när arbetets faktiska tillstånd förändras.

## Nuvarande position

- **Fas:** Fas 1, Rust som språk
- **Konceptenhet:** Enhet 2, ownership, ägd data och structs
- **Steg i inlärningsloopen:** Mental modell är nästa steg
- **Status:** Enhet 1 är avslutad och verifierad
- **Repetition:** 3 öppna objekt, inget ska repeteras omedelbart

## Senast slutfört

- Enhet 0 etablerade och verifierade Rusts toolchain, Cargo-modell och `job-server`-package.
- Enhet 1:s teori och förutsägelser är genomförda. Mikrolabbet hoppades över efter svårighetskalibrering.
- Job servern simulerar ett synkront jobb med högst ett konfigurerat antal försök och retry delay före varje retry.
- Simulatorns tabelltest täcker noll försök, framgång på försök 1–3 och failure när maxgränsen nås.
- `cargo fmt --check`, `cargo check`, `cargo test` och `cargo run` passerar; körningen ger `(3, false, 6)` för `simulate_job(3, 4)`.
- Enhet 1 uppfyller sitt avslutskriterium och är markerad som klar i roadmapen.

## Nästa konkreta handling

Starta enhet 2 med den mentala modellen för ownership: stack och heap, ägare, moves, `Copy`, explicit `Clone`, scopes och deterministisk destruktion. Introducera därefter structs som ägare av sammanhörande data innan förutsägelsefrågorna.

## Aktuell lärdom

En retry delay representerar väntan före ett nytt försök. Genom att lägga delay i början av försök 2 och senare uppstår ingen delay efter det sista försöket, och `can_attempt` behövs bara som loopvillkor.

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
