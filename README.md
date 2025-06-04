# menadzer_hasel_rust
Prosty menadżer haseł zarządzany z linii poleceń, napisany w języku Rust.

**Twórcy:**  
 - Agnieszka Dąbek
 - Oliwia Dyszewska

---

## 🔧 Wymagania wstępne

1. **Rust**
    - Zainstaluj [rustup](https://rustup.rs), a następnie sprawdź wersje:
      ```bash
      rustc --version
      cargo --version
      ```
2. **GPG (GNU Privacy Guard)** – na Windows używamy [Gpg4win](https://www.gpg4win.org/)
    - Pobierz i zainstaluj Gpg4win ze strony powyżej.
    - Po instalacji otwórz PowerShell i sprawdź:
      ```powershell
      gpg --version
      ```
   Aby dodać (szyfrować) nowe hasło, ustaw zmienną RECIPIENT na e-mail lub fingerprint Twojego klucza publicznego.
    - PowerShell:
   ```powershell
   $Env:RECIPIENT = "twoj@mail.com"
    ```

---

## ⚙️ Kompilacja projektu
```powershell
cargo build
```

---

## 🚀 Dostępne komendy

Wszystkie komendy uruchamiaj w katalogu projektu.

1. **Init**
```powershell
cargo run -- init
```

- Tworzy katalog password-store
    - Jeśli katalog już istnieje, nic nie nadpisuje.
    - Wyświetla ścieżkę do utworzonego katalogu.

2. **Insert**
```powershell
#Ustawienie odbiorcy (tylko raz na sesję):
$Env:RECIPIENT="twoj@mail.com"

cargo run -- insert <nazwa>
```

- Program poprosi o hasło, które następnie zostanie zaszyfrowane i zapisane.
- Po sukcesie zobaczysz komunikat:
    ```powershell 
  Saved password for <nazwa>
    ```

4. **Remove**
```powershell
cargo run -- remove <nazwa>
```
- Usuwa zapisane hasło o podanej nazwie.
- Komunikat po usunięciu:
    ```powershell 
  Removed password for <nazwa_pod_jaka_zapisane_jest_haslo>
    ```

4. **Show**
```powershell
cargo run -- show <nazwa>
```
- Wyświetla odszyfrowane hasło.

6. **List**
```powershell
cargo run -- list
```
- Wyświetla listę wszystkich zapisanych haseł (nazwy).

5. **Generate**
```powershell
# Domyślnie (16 znaków):
cargo run -- generate

# Podaj konkretną długość (np. 24 znaki):
cargo run -- generate 24

# Dodaj flagę --special, aby włączyć także znaki: !@#$%^&*()
cargo run -- generate 24 --special
```

- Generuje i wypisuje losowe hasło