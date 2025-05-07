**-Uyulması gereken Kurallar-** 
1. *Kontratın İçindeki İşleyişi Kavra*

   \* Tam olarak nasıl çalışıyor, hangi sırayla çalışıyor
   \* Çalışma sırası, çalışma prensipleri
   \* Genel yapısı
    \* Kod okuma, akış diyagramı çıkarma, state-transition diagram gibi teknik adımlar yok. Bunları ekle;

     \* lib.rs içindeki entrypoint’leri işaretle
     \* Her instruction’ın hesapları nasıl manipüle ettiğini grafikle anlat
**-Detaylı Talimat listesi-**
---

### **1. Kontratın İçindeki İşleyişi Kavra**

#### 1.1 **Kontratın Genel Çalışma Prensipleri**

* **Çalışma Prensipleri:**

  * Akıllı kontratın ana fonksiyonları nedir?
  * Hangi durumlarda hangi işlemler tetikleniyor?
  * Hangi koşullarda belirli fonksiyonlar çağrılır ve sonuçları nasıl hesaplanır?
    **Öneri:** Akıllı kontratın çalışma sırasını bir **state machine** olarak modelleyebilirsin. Bu, kontratın hangi koşullar altında hangi durumlardan geçeceğini ve hangi fonksiyonların hangi sırayla tetiklendiğini gösterir.

#### 1.2 **Kontratın İşleyişi ve Akış Sırası**

* **Akış Diyagramı (Flowchart):**

  * Kontratın giriş noktalarını ve her fonksiyonun tetiklenme sırasını grafiksel olarak göster.
  * Örneğin, **entrypoint** fonksiyonları, kontratın ilk başta hangi state’e geçtiğini, hangi fonksiyonların tetiklendiğini, hangi hesapların güncellendiğini ve hangi sonuçların elde edildiğini görsel bir formatta sunabilirsin.

* **State-Transition Diagram (Durum Değişim Diyagramı):**

  * Kontratın hangi **state**'lerde bulunduğu ve bu **state**'lerden hangi koşullarda geçiş yapılabileceğini tanımla.
  * Durum değişim diyagramı, bir kullanıcının işlem yaptığı andan itibaren kontratın hangi adımlardan geçtiğini ve ne tür aksiyonlar alındığını gösterir.
  * Her durumun açıklamaları ile birlikte, hangi verilerin (örneğin token bakiyeleri, hesap bilgilerinin güncellenmesi) etkilendiğini belirt.

**Öneri:** Flowchart ile her bir işlem sırasını, giriş ve çıkış noktalarını görselleştir. Örneğin, kullanıcı bir işlem başlatır, kontrat belirli kontroller yapar (örneğin, bakiyenin yeterli olup olmadığı) ve sonucu kullanıcıya bildirir. Bu süreçleri adım adım ve görsel olarak takip edebilmelisin.

---

#### 1.3 **Kod Okuma ve Teknik Detaylar**

* **lib.rs içindeki entrypoint’leri işaretle:**

  * Her entrypoint fonksiyonunun ne iş yaptığını açıkça belirleyin.
  * Hangi fonksiyonlar işlemi başlatıyor ve ne tür hesaplamalar yapıyor?
  * Hangi parametreler alınıyor, hangi koşullarda işlem gerçekleşiyor?
    **Öneri:** Entry point fonksiyonlarının her birini bir tablo veya liste halinde sıralayabilirsin. Her bir fonksiyonun:
  * Adı
  * Ne işe yaradığı
  * Parametreleri
  * Ne tür veriler döndürdüğü
  * Hangi hesapları manipüle ettiği

#### 1.4 **Instruction'ların Hesap Manipülasyonları**

* **Her Instruction’ın Hesapları Nasıl Manipüle Eder?**

  * Her instruction’ın (işlem talimatı) hangi hesapları değiştirdiğini açıklayan bir diyagram oluştur.
  * Hangi hesaplar üzerinde işlem yapılıyor (örneğin, kullanıcı bakiyesi, token kontratları)?
  * İşlem gerçekleştikten sonra hesaplarda nasıl değişiklikler oluyor?
    **Öneri:** Burada, her instruction'ı temsil eden bir şema veya diyagram oluşturabilirsin. Şemada:
  * Instruction türü
  * Manipüle edilen hesaplar (örneğin, kullanıcı bakiyesi, token transferi)
  * Sonuçlar (örneğin, bakiyenin artışı/azalışı)
  * Potansiyel hata durumları (örneğin, yetersiz bakiye)

---

### **2. Akış Diyagramları ve Görsel Temsil**

#### 2.1 **Akış Diyagramı (Flowchart)**

* **Ana Akış:**

  1. **Başlatma**: Kullanıcı işlemi başlatır.
  2. **Kontroller**: Gerekli kontroller yapılır (örneğin, bakiye kontrolü).
  3. **İşlem Gerçekleştirme**: Gerekli hesaplamalar ve işlemler yapılır (örneğin, token transferi).
  4. **Sonuç**: İşlem tamamlanır ve sonuç kullanıcıya iletilir.

* **Diyagram Örneği**:

  ```plaintext
  [Başlatma] --> [Kontroller] --> [İşlem Gerçekleştirme] --> [Sonuç]
  ```

#### 2.2 **State-Transition Diyagramı (Durum Değişim Diyagramı)**

* **Başlangıç Durumu**: Kontrat başlatılır.
* **State 1**: Kullanıcı bakiye sorgular.
* **State 2**: Kullanıcı işlem yapar (örneğin, token gönderir).
* **State 3**: İşlem tamamlanır ve kontrat durumunu günceller.

**Diyagram Örneği**:

```plaintext
[Başlangıç] --> [Kullanıcı Bakiye Sorgular] --> [Token Transferi] --> [İşlem Tamam]
```

---

### **3. Ekstra Analiz ve Hata Durumları**

#### 3.1 **Hata Durumları ve Geri Dönüşler**

* Kontratın hangi durumlarda hata döndüreceğini (örneğin, yetersiz bakiye, yanlış parametreler) ve bu hataların nasıl ele alındığını analiz et.
* **Hata Durumu Diyagramı** oluşturabilirsin.

#### 3.2 **Performans Analizi ve Optimizasyon**

* Kontratın işlem süreleri, gaz ücretleri, ve performansı ile ilgili analizler yap.
* Hangi işlemler daha fazla gaz harcıyor, hangi durumlarda işlem süreleri uzuyor?

---

###
