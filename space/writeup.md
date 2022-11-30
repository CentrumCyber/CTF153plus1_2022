# Space

## Enumeracja
Kod aplikacji nie zawiera nigdzie niebezpiecznego kodu prowadzącego do SQL injection. Warto jednak zauważyć, że pole `username` tabeli `app_user` nie ma na sobie klucza `UNIQUE` (co oznacza, że w teorii może istnieć wielu użytkowników o nicku `admin`) a jego typ to `CHAR(20)`, a nie `VARCHAR(20)`, co oznacza, że wartości krótsze niż 20 znaków wpisane do tej tabeli zostaną dopełnione spacjami z prawej strony, a wszystkie spacje z prawej strony będą usuwane przy pobieraniu danych (przykład: dodajemy użytkownika o nazwie "abcd " -> w bazie danych zostanie on zapisany jako "abcd                " -> po odczytaniu jego danych z bazy, otrzymamy "abcd"; opisane w https://dev.mysql.com/doc/refman/8.0/en/char.html).

## Rozwiązanie
Ponieważ baza nie wymusza, by nazwy użytkowników były unikalne, możemy stworzyć dodatkowego użytkownika podając nazwę "admin " (zapytanie z `db.rs:50` nie zwróci nic, gdyż "admin" nie jest równe "admin ", więc aplikacja pozwoli stworzyć użytkownika w `auth.rs:39`), a następnie zalogować się za pomocą nazwy "admin" i jego hasła (zapytanie z `db.rs:40` zwróci naszego użytkownika), co spowoduje, że otrzymamy od aplikacji ciastko sesji potwierdzające, że nasza nazwa użytkownika to "admin" (`auth.rs:60`). Następnie próba odczytania notatki przypisanej do użytkownika spowoduje wykonanie do bazy zapytania o notatkę użytkownika "admin" - w tym momencie istnieje już 2 takich użytkowników (oryginalny admin, oraz nasz "podrobiony"), a silnik InnoDb, w tym przypadku, indeksuje użytkowników po kluczu głównym (id), więc pierwszym wynikiem będzie notatka admina o niższym id - czyli tego "oryginalnego". Tym sposobem można odczytać flagę.

## Rozwiązanie - wersja krótka
1. Stworzyć użytkownika o nazwie "admin " (nie zapomnieć o spacji!) i dowolnym haśle (np. "password123")
2. Zalogować się używając nazwy użytkownika "admin" i powyższego hasła ("password123")
3. Odczytać flagę

## Flaga
`flag{var_1s_go0d_1n_sql_unl1ke_1n_js}`