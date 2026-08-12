# Repetitionslogg: Fas 1

## Sammanfattning

| Mått | Antal |
|---|---:|
| Förutsägelsefrågor besvarade | 8 |
| Klara vid första försöket | 3 |
| Delvis korrekta | 3 |
| Missuppfattningar | 2 |
| Öppna repetitionsobjekt | 1 |
| Förstärkta repetitionsobjekt | 0 |
| Stabila repetitionsobjekt | 0 |
| Återkommande missuppfattningar | 0 |

Klassificeringen avser första försöket och är diagnostisk, inte ett betyg. Relaterade svar kan grupperas i ett enda repetitionsobjekt.

## F1-U0-001: Cargo targets och byggartefakter

- **Enhet:** 0
- **Kategori:** Mental modell
- **Status:** Öppen
- **Ursprung:** Förutsägelsefråga 7–8
- **Nästa tillfälle:** Första naturliga användningen efter ett meningsfullt uppehåll, annars under fasens konsolidering
- **Ska repeteras nu:** Nej

### Observerad modell

Cargo target blandades ihop med katalogen `target/`, och `src/bin/worker.rs` identifierades inte som ett eget binary target.

### Korrekt modell

Ett Cargo target beskriver något Cargo kan bygga. Targetet har en crate root och byggs som en crate. Kompileringen producerar byggartefakter, vilka Cargo normalt lagrar under katalogen `target/`.

`src/main.rs` och `src/bin/worker.rs` upptäcks normalt som två separata binary targets. När flera körbara targets finns kan `cargo run` behöva ett explicit val, exempelvis `cargo run --bin worker`.

### Framtida återkallningsfrågor

1. Förklara skillnaden mellan Cargo target, crate, byggartefakt och katalogen `target/`.
2. Vilka targets upptäcker Cargo från `src/main.rs` och `src/bin/worker.rs`, och hur väljs ett av dem vid körning?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-12 | Ursprunglig förutsägelse | Missuppfattning | Svar på fråga 7–8 |
| 2026-08-12 | Omedelbar tillämpning i mikrolabb | Delvis, target kunde inte identifieras säkert | Klassificering av `src/main.rs` efter `cargo new --bin` |
