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
- [x] Dapatkan lemma.
- [x] Sunting lemma dan konsep.
- [ ] Sunting bahagian-bahagian lemma lain.
- [ ] Padam bahagian-bahagian lemma atau keseluruhan lemma.
- [ ] Import dari CSV (dalam pembaikan).
- [ ] Kongsi data dalam bentuk gambar.

## Pengaturcaraan
Prasyarat:
1. Dapatkan npm atau yarn untuk Javascript.
2. Dapatkan [Rust](https://www.rust-lang.org/).
3. Dapatkan [cargo-watch](https://crates.io/crates/cargo-watch) untuk membolehkan pelaksanaan `cargo watch` untuk ujian kod melalui (cargo sudah tersedia melalui rust):
    ```
    cargo install cargo-watch
    ```
4. Dapatkan [tauri](https://tauri.app/).
5. Dapatkan pakej-pakej Javascript melalui:
    ```
    npm install
    ```
    atau
    ```
    yarn install
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
    atau
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
