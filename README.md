# Samudra App

Samudra ialah aplikasi perkamusan dan pengurusan istilah untuk memudahkan pencatatan istilah serta pengongsiannya sesama rakan sekerja atau orang awam.
Dengan memudahkan proses ini, kita dapat menambah jumlah rujukan bahasa Melayu dalam talian sekali gus mempercepatkan perkembangannya dari segi penggunaan, penyelidikan serta pembelajarannya.

## Demo

https://github.com/samudradev/samudra-app/assets/61819672/88a7ea18-03fe-45e9-abf3-25a31ec3d53e

## Reka Data

Data kamus direka supaya satu kata boleh mendukung banyak konsep serta satu perkataan boleh dipadankan ke banyak kata asing yang bergantung pada konsep dan konteks yang digunakan.

![](./docs/model-samudra.png)

## Inspirasi

Reka bentuk dan reka fungsi aplikasi ini diinspirasikan daripada beberapa kamus dalam talian:

1. [Akebi](https://play.google.com/store/apps/details?id=com.craxic.akebifree&hl=en&gl=US) (Bahasa Jepun)
2. [Meriam-Webster](https://www.merriam-webster.com/) (Bahasa Inggeris)
3. [KamusAstro](http://kamusastro.com/) (Bahasa Indonesia, khusus 'Astronomi')
4. [Gelintaran 'define' Google](https://www.google.com/search?q=define+something) (Bahasa Inggeris)


## Reka Fungsi
- [x] Mewujudkan pangkalan data sqlite dalam folder berbeza.
- [x] Tunjukkan jumlah item mengikut jenis.
- [x] Memasukkan lemma beserta konsep, golongan kata, cakupan, dan kata asing.
- [x] Dapatkan dan padam lemma.
- [x] Sunting lemma dan konsep.
- [x] Sunting bahagian-bahagian lemma lain.
- [x] Padam bahagian-bahagian lemma.
- [ ] Import dari CSV (dalam pembaikan).
- [x] Kongsi data dalam bentuk gambar.

## Pengaturcaraan
Prasyarat:
1. Dapatkan npm atau yarn untuk Javascript.
2. Dapatkan [Rust](https://www.rust-lang.org/).
3. Dapatkan [tauri](https://tauri.app/).
4. Dapatkan pakej-pakej Javascript melalui:
    ```
    npm install
    ```
    atau
    ```
    yarn install
    ```
5. Dapatkan [cargo-watch](https://crates.io/crates/cargo-watch) (cargo sudah tersedia melalui rust) untuk membolehkan pelaksanaan `cargo watch` untuk ujian kod melalui :
    ```
    cargo install cargo-watch
    ```
6. Dapatkan [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) melalui:
    ```
    cargo install sqlx-cli
    ```
    kemudian sediakan database contoh untuk dapatkan membolehkan sqlx semak queri:
    ```
    sqlx database setup --database-url sqlite:src-tauri/.samudra-check.db --source src-tauri/database/migrations
    ```
    atau (dimudahkan melalui package.json)
    ```
    npm run setup-db
    ```
    atau
    ```
    yarn setup-db
    ```

### Pembangunan
1. Klon repositori ini.
2. Gunakan cargo atau npm atau yarn untuk laksanakan skrip pembangunan:
    ```
    cargo tauri dev
    ```
    atau
    ```
    npm run tauri dev
    ```
    atau
    ```
    yarn tauri dev
    ```

### Ujian
1. Dari root, boleh pilih salah satu:
    ```
    cargo  watch -x \"test --workspace\" -C src-tauri/ -c
    ```
    atau (dimudahkan melalui package.json)
    ```
    npm run watch
    ```
    atau
    ```
    yarn watch
    ```

### Bina Habis
1. Untuk hasilkan produk akhir, dalam bentuk `.exe` atau yang sepadan, laksanakan:
    ```
    cargo tauri build
    ```
    atau
    ```
    npm run tauri build
    ```
    atau
    ```
    yarn tauri build
    ```
