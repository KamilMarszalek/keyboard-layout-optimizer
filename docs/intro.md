# Narzędzie do optymalizacji układu klawiatury

## Skład zespołu
- Kamil Marszałek
- Michał Szwejk

## Treść zadania
Projekt polega na zaimplementowaniu narzędzia optymalizującego układ klawiatury pod kątem konkretnej metryki/metryk i danego zbioru tekstu. Rozwiązanie powinno posiadać graficzny interfejs użytkownika, preferowana aplikacja webowa, ale może też być klasyczne desktopowe GUI.

Inspirację można częściowo czerpać z następującej strony: http://patorjk.com/keyboard-layout-analyzer/

Aplikacja powinna:
- pozwalać na optymalizację układu zakładającego alfabet łaciński (mile widziana transliteracja tekstów załadowanych w innych językach) za pomocą metaheurystyki
- pozwalać na wizualizację zaproponowanego układu klawiatury wraz z mapą ciepła
- pozwalać użytkownikowi na załadowanie własnego układu i wyświetlenie dla niego odp. statystyk

Moduł optymalizacyjny powinien zostać zaimplementowany w języku niskopoziomowym.

## Podział na podproblemy

### 1. Moduł optymalizacyjny (Rust)
Główna logika optymalizacji układu klawiatury, w tym reprezentacja układu, model geometrii klawiatury, transliteracja, obliczanie metryk kosztu i implementacja metaheurystyki zostanie zaimplementowana w języku Rust i skompilowana do WebAssembly przy użyciu `wasm-pack`. Moduł ten będzie odpowiedzialny za generowanie optymalnych układów klawiatury na podstawie zdefiniowanych metryk i danych wejściowych.


#### Reprezentacja klawiatury (Rust)
Klawiatura będzie reprezentowana jako permutacja 26 liter alfabetu łacińskiego przypisanych do konkretnych klawiszy. Palce będą przypisane do poszczególnych klawiszy, zmianom ulegać będą wyłącznie litery przypisane do klawiszy. Fizyczna geometria klawiatury - czyli układ rzędów  i stagger - pozostanie niezmienna.

Obsługę cyfr i znaków specjalnych (np. `+`, `{`, `:`, `"`) potraktujemy jako możliwe rozszerzenie projektu, które może zostać zaimplementowane w późniejszym etapie, jeśli czas na to pozwoli. W pierwszej kolejności skupimy się na optymalizacji układu dla liter alfabetu łacińskiego.

#### Funkcja kosztu (Rust)
Naszym celem będzie minimalizacja funkcji kosztu, która będzie składać się z następujących metryk ergonomicznych:
- same-finger bigrams (SFB) - dwa kolejne znaki pisane tym samym palcem 
- finger distance (FD) - łączna droga palców podczas pisania (mierzona w metryce euklidesowej)
- home row usage (HRU) - procent naciśnięć na home row (środkowy rząd klawiatury)
- hand alternation (HA) - naprzemienne użycie rąk
- row jumping (RJ) - skok przez cały rząd tym samym palcem (np. z `q` do `z`)

Funkcja kosztu będzie ważona, umożliwiając użytkownikowi dostosowanie wagi poszczególnych metryk do swoich preferencji.

Można to przedstawić wzorem:

$$Cost = w_1 \cdot SFB + w_2 \cdot FD - w_3 \cdot HRU - w_4 \cdot HA + w_5 \cdot RJ$$

gdzie $w_1, \ldots, w_5$ to wagi przypisane do poszczególnych metryk. Ujemne wagi dla HRU i HA wynikają z faktu, że chcemy maksymalizować ich wartości, co przekłada się na obniżenie całkowitego kosztu.


#### Metaheurystyka (Rust)
Optymalizacja układu klawiatury zostanie przeprowadzona za pomocą metaheurystyki. Zostanie użyty algorytm symulowanego wyżarzania ze względu na jego skuteczność w rozwiązywaniu problemów kombinatorycznych. Sąsiedztwo będzie definiowane jako permutacja dwóch losowo wybranych klawiszy. Algorytm będzie iteracyjnie generował nowe układy klawiatury, obliczał ich koszty i decydował o ich akceptacji na podstawie różnicy kosztów i aktualnej temperatury. Jeżeli wyniki będą niezadowalające, można rozważyć implementację innych metaheurystyk, takich jak algorytm genetyczny lub inne metody reprezentacji stanu czy generowania sąsiedztwa.

#### Transliteracja (Rust)
Tekst wejściowy zapisany w innym alfabecie zostanie transliterowany do małych liter
alfabetu łacińskiego bezpośrednio w module Rust, z użyciem crate `any_ascii`.
Dzięki temu cały preprocessing i optymalizacja odbywają się po stronie klienta, a do funkcji optymalizacyjnych trafiają już znormalizowane dane (`a`-`z`).

### 2. Frontend (TypeScript / Vue.js)
Interfejs użytkownika zostanie zaimplementowany jako aplikacja webowa, wykorzystująca framework Vue.js. 
Moduł Rust skompilowany do WASM ładowany jest bezpośrednio przez TypeScript - komunikacja odbywa się przez interfejs wygenerowany przez `wasm-bindgen`, bez żadnego
serwera pośredniczącego.
Frontend będzie odpowiedzialny za:
- ładowanie i inicjalizację modułu WASM,
- wizualizację układu klawiatury wraz z mapą ciepła, która będzie kolorować klawisze na podstawie ich użycia
- ustalenie parametrów optymalizacji, takich jak wagi metryk
- wyświetlanie wyników optymalizacji, w tym zaproponowanego układu klawiatury i statystyk w postaci wykresów dla poszczególnych składowych funkcji kosztu

## Narzędzia
| Warstwa | Narzędzia |
|---------|-----------|
| Rust | `cargo`, `rustfmt`, `clippy`, `wasm-pack`, `wasm-bindgen` |
| Frontend | `npm`, `vitest` |
| CI / Build | `just`, `GitHub Actions` |