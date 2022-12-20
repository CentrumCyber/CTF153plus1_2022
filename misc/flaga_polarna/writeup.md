# Flaga Polarna

## Rozwiązanie
Załączony plik `challenge.db` to plik bazy danych SQLite3. Otwarcie go za pomocą narzędzia CLI `sqlite3` i wyświetlenie schematu poleceniem `.schema` ujawni, że w baze znajdują się 2 tabele: `image` z kolumnami `width` i `height`, oraz `pixels` z kolumnami `r`, `phi` i `colour`. Tabela `image` zawiera jeden wiersz z wartościami `111` i `111`, natomiast `pixels` - wiele wierzy, w których `r` i `phi` są liczbami rzeczywistymi, a `colour` wynosi `0` lub `1`. Biorąc pod uwagę nazwę zadania oraz nazwy kolumn `r` i `phi` można dojść do wniosku, że tabela `pixels` przechowuje informacje o kolorach pikseli, których współrzędne w układzie współrzędnych biegunowych to `(r, phi)`. Jedyny wiersz tabeli `image` informuje natomiast o wymiarach obrazka.

Najprostszym sposobem na odzyskanie flagi jest napisanie programu, który odczytuje dane z tabeli `pixels`, zamienia współrzędne biegunowe na kartezjańskie i nanosi kolor czarny (0) lub biały (1) na odpowiedni piksel na obrazku. Załączony plik `decode.py` zawiera przykład takiego programu.

Obrazek odtworzony z danych z bazy to kod QR. Odczytanie go (np. za pomocą programu `zbarimg`) zwraca flagę.

## Flaga
`flag{why_w0uldnt_you_just_us3_c4rt3sian_th0}`