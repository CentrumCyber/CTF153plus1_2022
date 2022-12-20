# ILPy++

## Etap 1. - C#
Sprawdzenie typu pliku `ILPy++.exe` (np. za pomocą Unixowego polecenia `file`) zwróci informację, że jest to kod bajtowy platformy .NET - można go zatem zdekompilować np. za pomocą narzędzia [Avalonial ILSpy](https://github.com/icsharpcode/AvaloniaILSpy). Zdekompilowana metoda `Main` klasy `ILPy__.Program` wygląda następująco:

```cs
public static void Main(string[] args)
{
	Resolver.RegisterAssemblyResolver();
	if (!isOsValid())
	{
		Console.WriteLine("This program must be run on a x64 Linux");
		return;
	}
	if (args.Length != 1)
	{
		Console.WriteLine("Usage: ./ILPy++.exe <flag>");
		return;
	}
	string text = args[0];
	if (!text.StartsWith("flag{") || text.Last() != '}')
	{
		Console.WriteLine("The flag format is 'flag{.*}'");
		return;
	}
	if (text.Length < 10)
	{
		Console.WriteLine("Invalid flag!");
		return;
	}
	ResourceManager resourceManager = new ResourceManager("ILPy__.Stages", typeof(Program).Assembly);
	IEnumerable<byte> second = from c in text.Skip(5).Take(4).Cycle()
		select (byte)c;
	byte[] first = (byte[])resourceManager.GetObject("Stage2");
	Func<byte, byte, char> resultSelector = (byte a, byte b) => (char)(a ^ b);
	string text2 = new string(first.Zip(second, resultSelector).ToArray());
	if (!text2.StartsWith("from"))
	{
		Console.WriteLine("Invalid flag!");
		return;
	}
	ScriptEngine scriptEngine = Python.CreateEngine();
	ScriptScope scriptScope = scriptEngine.CreateScope();
	scriptEngine.Execute(text2, scriptScope);
	dynamic variable = scriptScope.GetVariable("run");
	Console.WriteLine(variable(new string(text.Skip(9).ToArray()), resourceManager));
}
```

Na początku umożliwia ona programowi ładowanie zestawów zapisanych w zasobie `ILPy__.Dlls` (`ILPy__.IgnoreMe.Resolver.RegisterAssemblyResolver()`) oraz sprawdza, czy program działa na Linuxie na procesorze o architekturze x64. Potem następuje sprawdzenie, czy został podany dokładnie 1 argument linii poleceń oraz czy ma on format `flag{.*}` i długość przynajmniej 10 znaków. Następnie utworzony zostaje manager zasobów odczytujący zasoby z `ILPy__.Stages`. W zmiennej `second` zapisany zostaje obiekt enumerowalny zwracający w nieskończoność w sposób zapętlony znaki od 6. do 9. z flagi rzutowane na typ `byte` (wykorzystuje do tego metodę rozszerzającą `Cycle` będącą statyczną metodą klasy `ILPy__.Program`) - oznacza to, że jeśli flagą jest `flag{ABCD__________}`, `second` będzie nieskończoną sekwencją postaci: `{ 0x41, 0x42, 0x43, 0x44, 0x41, 0x42, ... }`. Sekwencja ta zostaje zzipowana za pomocą funkcji anonimowej zwracającej znak ASCII, którego kod to wynik xorowania jej argumentów, z elemetem `Stage2` zasobu `ILPy__.Stages`. Po upewnieniu się, że otrzymany w ten sposób ciąg znaków rozpoczyna się literami "from", zostaje on uruchomiony jako kod Pythona za pomocą biblioteki IronPython. Na końcu wypisany zostaje wynik wywołania funkcji `run` pobranej z kontekstu IronPythona z 2 argumentami: flagą (poza pierwszymi 9 znakami) oraz utworzonym wcześniej managerem zasobów.

Klucz służacy do rozszyfrowania `Stage2` (będący równocześnie pierwszymi 4 znakami flagi po `flag{`) można odzyskać poprzez wykonanie operacji xor na ciągu `from` oraz pierwszych 4 bajtach `Stage2`. `Stage2` można w łatwy sposób wyodrębnić z pliku exe za pomocą ILSpy - wystarczy nacisnąc prawym przyciskiem myszy na element `ILPy++ > Resources > ILPy__.Stages.resources > Stage2` i wybrać opcję `Save Code...`. Cały plik można rozszyfrować poprzez xorowanie każdych kolejnych 4 bajtów z odzyskaną częścią flagi (`th3_`). Przykładowy skrypt wykonujący to zadanie znajduje się w pliku `scripts/decrypt_stage2.py` (zakłada on obecność pliku `Stage2.enc`, będącego zaszyfrowanym kodem 2. części zadania).

## Etap 2. - Python
Większość nazw w rozszyfrowanym skrypcie została zaobfuskowana, jednak po szybkiej analizie można podmienić je na sensowne ciągi i otrzymać następujący kod:

```py
from System import Array, Byte
from System.Diagnostics import Process, ProcessStartInfo
from System.IO import FileInfo, Path
from System.Security.Cryptography import SHA256


def cycle(arg):
	while True:
		yield from arg


def windows(arg, n):
	return (arg[i:i+n] for i in range(0, len(arg), n))


def decrypt_stage3(encrypted_stage3, flag2_1):
	if ord(flag2_1[-1]) != 95:
		return None

	with SHA256.Create() as s:
		hash = s.ComputeHash(Array[Byte](flag2_1[:2].encode()))
		digest = ''.join(map(lambda x: f'{x:x02}', hash))
		if digest != 'e4223ed20d7ea5740a326e2b268ca6db91d041cf5194f577e393a8ba3b85d8e9':
			return None

	flag2_1 = flag2_1[2:-1]

	result = []

	for stage3_chunk, key in zip(windows(encrypted_stage3, 16), cycle([flag2_1])):
		result += [*map(lambda p: p[0] ^ ord(p[1]), zip(reversed(stage3_chunk), key))]

	if bytes(result[:4]) != b'\x7fELF':
		return None

	if result[4] - result[5] - result[6] != 0 or result[7] != 0:
		return None

	if any(map((0).__ne__, result[7:16])):
		return None

	return bytes(result)


def run_stage3(stage3, flag2_2):
	filename = Path.GetTempFileName()

	try:
		with open(filename, 'wb') as f:
			f.write(stage3)

		Process.Start('chmod', f'+x {filename}')

		psi = ProcessStartInfo()
		psi.FileName = filename
		psi.Arguments = flag2_2
		psi.UseShellExecute = True

		proc = Process.Start(psi)
		proc.WaitForExit()

		if proc.ExitCode != 0:
			result = 'Invalid flag!'
		else:
			result = 'Congratulations! You\'ve found the flag!'

		return result
	finally:
		FileInfo(filename).Delete()


def run(flag2, manager):
	if len(flag2) < 20:
		return 'Invalid flag!'

	flag2_1 = flag2[:19]
	flag2_2 = flag2[19:]
	encrypted_stage3 = list(manager.GetObject('Stage3'))
	stage3 = decrypt_stage3(encrypted_stage3, flag2_1)

	if stage3 is None:
		return 'Invalid flag!'

	return run_stage3(stage3, flag2_2)
```

Funkcja `decrypt_stage3` wywołana zostaje z 2 argumentami: zaszyfrowanym kodem 3. części oraz pierwszymi 19 znakami 2. części flagi (tj. znakami 9-27 oryginalnej flagi, zakładając, że 1. znak ma indeks 1). Na początku sprawdzane jest, czy ostatni znak flagi ma kod 95 oraz czy hash pierwszych 2 znaków jest równy podanemu hashowi. Z racji, że istnieje jedynie 2^16 różnych kombinacji 2 znaków ascii, hash ten można łatwo złamać metodą siłową. Następnie odrzucone zostają 2 pierwsze oraz ostatni znak klucza (z 19 znaków zostaje 16) a reszta zostaje wykorzystana do rozszyfrowania kodu 3. etapu zadania. Zastosowany tutaj algorytm jest bardzo podobny do tego z części 1., z tą różnicą, że każdy blok zaszyfrowanych danych jest odwracany przed wykonaniem operacji xor. Następnie ma miejsce weryfikacja zdeszyfrowanych danych:
- Pierwsze 4 bajty muszą być magiczną liczbą formatu ELF
- różnica wartości znaku 5. oraz sumy znaków 6. i 7. musi być równa zero, znak 8. również musi mieć wartość zero - ponieważ 5. znak w nagłówku pliku ELF musi być równy 1 lub 2 a znak 7. musi być równy 1, to jedynymi możliwymi (zakładając, że plik jest poprawny - musi być jednak poprawny, gdyż później jest on uruchamiany) wartościami tych 4 bajtów są: `2 1 1 0`
- bajty 8-16 muszą być równe 0
Z tych danych łatwo można odzyskać klucz deszyfrowania 3. części zadania (oraz całą 2. część flagi) a następnie wykorzystać go do rozszyfrowania kodu - operacje te zostały zaimplementowane odpowiednio w plikach `scripts/get_flag_part_2.py` oraz `scripts/decrypt_stage3.py` (oba skrypty zakładają istnienie pliku `Stage3.enc`). Druga część flagi to `m0r3_L4ngu4ges_th3_`.

Po zdeszyfrowaniu kodu 3. części zadania, skrypt części 2. uruchamia go z 1 argumentem linii poleceń - ostatnią częścią flagi.

## Etap 3. - C++
Etap 3. to skompilowany pod Linuxa na architekturze x64 program napisany w C++.

Działanie funkcji `main`:
- zapisanie 2. argumentu z linii poleceń (1. po nazwie wykonywanego pliku) do zmiennej typu std::string (nazwijmy ją `flag`)
- utworzenie zmiennej typu std::optional<int> przechowującej liczbę 68 (nazwijmy ją `value`)
- wykonywanie następującej pętli (z licznikiem, który nazwiemy `i`, zaczynającym od 0) póki `value` nie jest pusta (nie jest równa `std::nullopt`; po zrzutowaniu na typ `bool` nie daje `false`; patrz dokumentacja C++):
	- przypisanie do `value` wartości zwróconej z `next_collatz(*value)`
	- sprawdzenie, czy `i` jest mniejsze niż długość flagi; jeśli nie, opuszczenie pętli i zwrócenie 1 (błędu) z programu
	- sprawdzenie, czy i-ty znak flagi jest równy wartości `data[*value]`; jeśli nie, opuszczenie pętli i zwrócenie 1 (błędu) z programu
	- inkrementacja licznika pętli
- sprawdzenie, czy wartość licznika jest równa długości flagi; zwrócenie 0 (flaga poprawna) z programu, jeśli tak, w przeciwnym razie zwrócenie 1

Działanie funkcji `next_collatz`:
- sprawdzenie, czy argument (nazwijmy go `n`) jest mniejszy od 2 i zwrócenie `std::nullopt`, jeśli tak
- zwrócenie `std::optional<int>(n / 2)` jeśli `n` jest parzyste, `std::optional<int>(3 * n + 1)` w przeciwnym wypadku

Podsumowanie:
Program sprawdza, czy kolejne znaki ostatniej części flagi odpowiadają wartościom zapisanym w globalnej tablicy `data` pod indeksami będącymi kolejnymi wartościami ciągu z problemu Collatza, poczynając od następcy liczny 68 (czyli 34). Ostania część flagi jest zatem ciągiem utworzonym z połączenia znaków `data[34]`, `data[17]`, `data[52]`, `data[26]`, `data[13]`, ..., `data[1]`. Ostatnia część flagi jest zatem równa `b3TT3r_Right?}`

## Flaga
`flag{th3_m0r3_L4ngu4ges_th3_b3TT3r_Right?}`