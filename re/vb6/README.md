# VB6
## Instrukcja przejścia

1. Gracz dostaje EXE z programem
2. Odpalenie -> prompt o login i hasło
3. Gracz wrzuca binarkę do ghidry / innego softu
4. Należy dojść do tego, że program calluje funkcję robiącą SHA384 na haśle i porównuje z zapisanym hashem; hash należy zbrute'ować (hasło ma 4 znaki; "nyaa"; potencjalnie do zmiany na dłuższe?)
5. Po drodze gracz musi znaleźć w binarce też login ("alice")
6. Gracz wprowadza login/hasło
7. Program pokazuje mu kawałek flagi (bo element UI jest zbyt krótki żeby pokazać ją w całości);
8. Gracz wyjmuje flagę z pamięci z użyciem debuggera (lub liczy ją na podstawie wcześniej znalezionych danych)