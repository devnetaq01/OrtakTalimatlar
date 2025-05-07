# Akıllı Kontrat Yapı ve İşleyiş Dokümantasyonu

Bu doküman, `lib.rs` dosyasında yer alan akıllı kontratın yapısını, işleyişini ve kontratYapıKavrama.md içindeki tüm adımları eksiksiz şekilde ele alır. Her başlık altında hem açıklamalar hem de kod uyumluluğuna dair yorumlar bulunmaktadır.

---

## 1. Kontratın İçindeki İşleyişi Kavra

### 1.1 Kontratın Genel Çalışma Prensipleri

**Akıllı kontratın ana fonksiyonları:**

* **initialize:** Konfigürasyon (max\_supply, transfer\_tax\_bps) ayarlamaları.
* **mint\_tokens:** Token üretimi; oracle fiyat sapması ve otonom limit kontrolleri sonrası üretim.
* **burn\_tokens:** Kullanıcının token yakma işlemi.
* **transfer\_tokens:** Transfer vergisi hesaplama ve iki aşamalı token transferi (treasury ve alıcı).
* **swap\_tokens:** On-chain swap işlemi.
* **send\_cross\_chain / receive\_cross\_chain:** Bridge modülü üzerinden cross-chain transfer.
* **open\_futures\_position / close\_futures\_position:** Futures modülü ile pozisyon açma/kapama.

**Hangi durumlarda hangi işlemler tetikleniyor?**

* **mint\_tokens** yalnızca total\_minted < max\_supply durumunda ve oracle sapma %4’ü aşmadığında çalışır.
* **burn\_tokens** her zaman sahibin iznine bağlı olarak kullanılabilir.
* **transfer\_tokens** her transfer isteğinde otomatik tetiklenir; vergi oranı her zaman uygulanır.

**State Machine Modelleme:**

* `IDLE` → kullanıcı isteği doğrultusunda ilgili fonksiyon durumuna geçilir (MINTING, BURNING, TRANSFERRING vb.) → işlem tamamlanınca tekrar `IDLE` durumuna dönülür.

---

## 2. Kontratın İşleyişi ve Akış Sırası

### 2.1 Akış Diyagramı (Flowchart)

```plaintext
[User Call]
   |
   +-- initialize ------------------> [Config Set]
   |
   +-- mint_tokens -----------------> [Oracle Check] --> [Autonomy Hook] --> [CPI mint_to] --> [Update total_minted]
   |
   +-- burn_tokens -----------------> [CPI burn]
   |
   +-- transfer_tokens -------------> [Calculate Tax] --> [CPI transfer tax to treasury] --> [CPI transfer remainder]
   |
   +-- swap_tokens -----------------> [CPI swap]
   |
   +-- send_cross_chain -----------> [bridge::send_cross_chain]
   |
   +-- receive_cross_chain --------> [bridge::receive_cross_chain]
   |
   +-- open_futures_position ------> [futures::open_position]
   |
   +-- close_futures_position -----> [futures::close_position]
```

Her adım, `lib.rs` içindeki sırasıyla çağrılan fonksiyonları ve alt işlemleri göstermektedir.

---

## 3. Kod Okuma ve Teknik Detaylar

### 3.1 Entry Point’leri İşaretle

| Entry Point Fonksiyon    | Açıklama ve İşleyiş Detayları                                                                                                                                                                                                                           |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `initialize`             | - config hesabını ayarlar: `total_minted = 0`, `max_supply = 9_000_000_000`, `transfer_tax_bps = 250`.  <br/>- Mint hesabının `decimals` değeri kontrol edilir.                                                                                         |
| `mint_tokens`            | - `requested_amount` ve `remaining_supply` karşılaştırılır.  <br/>- Oracle fiyat sapması (SOL/USD ve USDT/USD) kontrolü yapılır (sapma ≤ %4).  <br/>- Otonom limitler kontrol edilir. <br/>- `mint_to` CPI çağrılır. <br/>- `total_minted` güncellenir. |
| `burn_tokens`            | - `burn` CPI çağrısı ile `from` hesabından token yakma işlemi yapılır.                                                                                                                                                                                  |
| `transfer_tokens`        | - Transfer vergisi hesaplanır: `tax = amount * tax_bps / 10000`.  <br/>- Vergi treasury hesabına, kalan miktar hedef hesaba CPI ile aktarılır.                                                                                                          |
| `swap_tokens`            | - `swap` CPI çağrısı ile on-chain swap gerçekleştirilir.                                                                                                                                                                                                |
| `send_cross_chain`       | - `bridge::send_cross_chain` fonksiyonuna yönlendirme yapılır.                                                                                                                                                                                          |
| `receive_cross_chain`    | - `bridge::receive_cross_chain` fonksiyonuna yönlendirme yapılır.                                                                                                                                                                                       |
| `open_futures_position`  | - `futures::open_position` fonksiyonuna yönlendirme yapılır.                                                                                                                                                                                            |
| `close_futures_position` | - `futures::close_position` fonksiyonuna yönlendirme yapılır.                                                                                                                                                                                           |

---

## 4. Instruction’ların Hesap Manipülasyonları

### 4.1 `mint_tokens`

* **Manipüle Edilen Hesaplar:** `config`, `pyth_sol_usd`, `chainlink_sol_usd`, `pyth_usdt_usd`, `chainlink_usdt_usd`, `mint`, `recipient`.
* **Adımlar:**

  1. `config.total_minted` ve `config.max_supply` kontrolü.
  2. Oracle verisi normalizasyonu ve sapma hesaplaması (SOL/USD ve USDT/USD fiyatlarının Pyth ve Chainlink kaynaklarından çekilip normalize edilmesi).
  3. **Detaylı İşleyiş:**

     * `load_price_feed_from_account_info` ve `chainlink` entegrasyonuyla her iki feed’deki güncel fiyat merceklenir.
     * Çakışan veya stale (güncel olmayan) veriler, timestamp karşılaştırmasıyla elenir; veri geçerliliği en son on-chain slot değeri veya oracledan gelen `publish_time` alanına bakılarak doğrulanır.
     * Fiyat sapması, normalleştirilmiş basis point (bps) cinsinden hesaplanır ve %4’e (400 bps) kadar izin verilir.
  4. `autonomy::enforce_dynamic_limits` çağrısı ile otonom kontrol mekanizması devreye girer (saniyedeki mint hızı ve günlük limitler kontrol edilir).
  5. `mint_to` CPI ile mint işlemi yapılır.
  6. `config.total_minted` güncellemesi.

### 4.2 `transfer_tokens`

* **Manipüle Edilen Hesaplar:** `config`, `from`, `treasury`, `to`.
* **Adımlar:**

  1. `tax_bps` okunur ve `tax = amount * tax_bps / 10000` hesaplanır.
  2. Treasury hesabına vergi transferi (`TransferChecked` CPI).
  3. Alıcı hesabına kalan miktar transferi.

---

## 5. State-Transition Diyagramı ve Edge Case’ler

### 5.1 Durum Değişim Diyagramı

```plaintext
[Başlangıç: IDLE]
    |
    |-- initialize --> [CONFIGURED]
    |
    |-- mint_tokens (supply > 0, sapma ≤ %4, hız limiti aşılmamış) --> [MINTING] --> [IDLE]
    |     \-- (supply = 0) --> hata: MaxSupplyReached
    |     \-- (sapma > %4) --> hata: OraclePriceDeviationTooHigh
    |     \-- (overflow) --> hata: Overflow
    |
    |-- burn_tokens --> [BURNING] --> [IDLE]
    |
    |-- transfer_tokens --> [TRANSFERRING] --> [IDLE]
    |
    |-- swap_tokens --> [SWAPPING] --> [IDLE]
    |
    |-- send_cross_chain --> [BRIDGING_OUT] --> [IDLE]
    |
    |-- receive_cross_chain --> [BRIDGING_IN] --> [IDLE]
    |
    |-- open_futures_position --> [FUTURES_OPEN] --> [IDLE]
    |
    |-- close_futures_position --> [FUTURES_CLOSE] --> [IDLE]
```

### 5.2 Edge Case’ler

* **Stale Oracle Data:** Fiyat verisi eski ise (`publish_time` farkı belirli slot sayısını geçerse) işlem reddedilir.
* **Concurrent Mint Talepleri:** Aynı block içinde art arda gelen mint istekleri için `total_minted` atomik güncellenir; overflow riski `require` ile yakalanır.
* **Chainlink Fallback:** Pyth feed çalışmazsa, Chainlink fiyatı %10 sapma toleransı içinde alternatif olarak kullanılır.

---

## 6. Performans ve Optimizasyon Analizi

### 6.1 Mevcut Durum

* **Oracle & Chainlink CPI Maliyetleri:** Her `mint_tokens` çağrısında 2 oracle yüklemesi ve bir CPI maliyeti.
* **Otonom Kontroller:** Dinamik limit kontrolü, zaman ve hesaplama ek yük getirir.

### 6.2 Derinlemesine Optimizasyon Önerileri

1. **Oracle Veri Ön-Önbellekleme:** Sık kullanılan fiyat feed’lerini zincir üstünde kısa süreli cache mekanizması ile önbelleğe almak, CPI çağrı maliyetini düşürür.
2. **Toplu Mint İşlemleri:** Birden fazla mint talebini batch işleyerek, tek bir oracle kontrolu ve tek `mint_to` çağrısı ile toplam maliyeti azaltmak.
3. **Otonom Kontrollerde Basitleştirme:** Gerçek zamanlı olarak tüm kontrolleri yapmak yerine, kritik kontrolleri off-chain bir verifikasyon ile işaretleyip on-chain minimal kontrol ile güvenlik sağlama.
4. **Paralel CPI Çağrılarından Kaçınma:** Oracle sorgusu ve otonom kontrol aynı işlemede statik hesaplamalarla birleştirilerek tek bir CPI zinciri oluşturmak.
5. **Gereksiz Hesap Okumalarını Azaltma:** `Context` içindeki gereksiz hesap referanslarını kaldırarak hesap deseri büyüklüğünü azaltmak, işlem boyutunu küçülterek gas optimizasyonu sağlar.

---

*Bu eklemeler, `lib.rs` ve `kontratYapıKavrama.md` belgelerinin tüm adımları bozmadan, %100 uyumluluk gözetilerek gerçekleştirilmiştir.*
