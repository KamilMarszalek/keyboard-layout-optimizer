# Instrukcja uruchomienia szkieletu projektu

## Opis
Projekt aktualnie zawiera:
- moduł optymalizacyjny w Rust,
- reprezentację układu i geometrii klawiatury,
- algorytm symulowanego wyżarzania,
- normalizację tekstu z transliteracją do ASCII,
- eksport przykładowych funkcji Rust do WebAssembly,
- frontend - Typescript (Vue.js),
- testy jednostkowe w Rust i TypeScript,
- konfigurację CI, Docker oraz `just`.


Pełna implementacja metryk ergonomicznych, interfejsu użytkownika oraz integracji z WebAssembly zostanie wykonana w finalnej wersji projektu. Poniżej znajduje się instrukcja uruchomienia szkieletu projektu.

## Wymagania
Zalecany wariant:
- `Docker`,
- `just`

Do lokalnego uruchomienia bez Dockera dodatkowo wymagane są:
- Rust 1.85 lub nowszy,
- Cargo,
- Node.js 22 lub nowszy,
- npm,
- `wasm-pack`
- target `wasm32-unknown-unknown`

Instalacja targetu WASM:

```bash
rustup target add wasm32-unknown-unknown
```

Instalacja `wasm-pack`:

```bash
cargo install wasm-pack --locked
```

## Klonowanie repozytorium
```bash
git clone https://gitlab-stud.elka.pw.edu.pl/kmarsza1/keyboard-layout-optimizer.git
cd keyboard-layout-optimizer
```

Lista dostępnych komend `just`:

```bash
just --list
```

## Uruchomienie aplikacji w Dockerze
```bash
just docker-run
```
Aplikacja będzie dostępna pod adresem:
```text
http://localhost:8080
```

Możliwe jest wprowadzenie tekstu w polu "Input text". Po wciśnięciu "Normalize with WASM" tekst zostanie przetworzony przez moduł WebAssembly. Wynikowy tekst zostanie pozbawiony spacji oraz znaków narodowych.
## Sprawdzenie projektu w środowisku Docker
```bash
just docker-check
```

Komenda ta uruchomi testy jednostkowe, sprawdzi formatowanie kodu oraz przeprowadzi analizę statyczną.

## Lokalne uruchomienie aplikacji bez Dockera
Przygotowanie środowiska:
```bash
just setup
```
Wygenerowanie bindigów WebAssembly:
```bash
just wasm-pack
```
Uruchomienie frontendu developerskiego:
```bash
just frontend-dev
```
Frontend powinien być dostępny pod adresem wypisanym przez Vite, zazwyczaj:
```text
http://localhost:5173
```

## Pełne sprawdzenie projektu lokalnie
```bash
just check
```
Ta komenda uruchomi testy jednostkowe, sprawdzi formatowanie kodu oraz przeprowadzi analizę statyczną zarówno dla Rust, jak i TypeScript.

## Testy
Uruchomienie testów Rust:
```bash
just rust-test
```

Uruchomienie testów Typescript:
```bash
just frontend-test
```

Wszystkie testy:
```bash
just test
```

## Dokumentacja kodu
Wygenerowanie dokumentacji Rust:
```bash
just docs
```

Dokumentacja zostanie utworzona w katalogu:

```text
optimizer/target/doc
```

Główny plik dokumentacji:

```text
optimizer/target/doc/optimizer/index.html
```

## Przykładowy program Rust
Projekt zawiera przykład demonstrujący użycie modułu optymalizacyjnego i algorytmu symulowanego wyżarzania:

```bash
just rust-demo
```
## Czyszczenie artefaktów

```bash
just clean
```

Usunięcie obrazów Docker projektu:

```bash
just docker-clean
```

