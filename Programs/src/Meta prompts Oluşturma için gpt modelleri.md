Teşekkürler. Kontratlara yönelik hesap ve ihtiyaçlara özel olarak ayrı ayrı promptlar oluşturabilecek, bu promptları test edip optimize etmenize imkân tanıyacak, profesyonel ve yapısal meta promptlar hazırlama konusunda en uygun GPT modelini ve Explore GPTs alanındaki en başarılı örnekleri araştıracağım.

Araştırma tamamlandığında, size en doğru modeli, nasıl kullanacağınızı ve örneklerle birlikte sunacağım.


# GPT Modelleri ve Hazır Örnekleri

Kontrat bileşenleri için meta-prompt şablonları oluştururken önce **prompt mühendisliği** konusunda uzmanlaşmış modeller ve araçlar incelenmelidir. Örneğin, GPT mağazasında “Prompt Engineering” kategorisinde çok sayıda özel GPT bulunmaktadır. Bu modeller, istenen göreve uygun **yapısal** ve optimize edilmiş promptlar üretmeyi hedefler. Aşağıdaki tabloda bazı öne çıkan örnek GPT modelleri ve kullanım alanları özetlenmiştir:

| **GPT Modeli/Şablonu**                                   | **Kullanım Alanı**                   | **Özellikler/Notlar (Kaynak)**                                                                                                                 |
| -------------------------------------------------------- | ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| **ChatGPT (GPT-4)**                                      | Genel amaçlı metin ve bilgi işleme   | Çok güçlü dil modeli; geniş bağlam anlayışı. Prompt tasarımında esneklik sağlar ancak sonuçların denetlenmesi gerekir. (Genel bilgi)           |
| **Prompt Creator GPT (chainbrain.ai)**                   | Prompt iyileştirme                   | Mevcut promptu alır, günceller ve sürekli geliştirme önerilerinde bulunur. Temel teşhisten kaçınmak için en iyi uygulamaları kullanır.         |
| **Prompt Wizard GPT (Jordan Gibbs)**                     | Prompt optimizasyonu                 | Kullanıcıdan alınan temel bir promptu, yeni araştırma temelli tekniklerle yeniden yazar ve optimize eder. Farklı varyasyonlar sunar.           |
| **Prompt Optimizer GPT (InfiniteCraft)**                 | İstem dönüşümü                       | Verilen basit promptu üç farklı şekilde yeniden yazar, her biri farklı optimizasyon stratejileri uygular.                                      |
| **Prompt Perfect GPT (promptperfect.xyz)**               | İstem yeniden yazma                  | Mevcut promptu otomatik olarak daha doğru ve net bir hâle getirir. Hassasiyeti artırmak için odaklanmış yapı kullanır.                         |
| **-- GIGA Prompt – Perfect Prompt Creator (gendojo.ai)** | Prompt oluşturma (NASA yöntemleri)   | NASA’nın uzmanlık teknikleriyle “mükemmel” promptlar oluşturmayı vaat eder. Özellikle bilimsel/teknik alanlarda iddialıdır.                    |
| **Prompt Engineer for Superior Prompts (aitoolreport)**  | Prompt mühendisliği asistanı         | Yeni bir prompt tasarlamanıza veya mevcut bir promptu geliştirmenize yardımcı olur. En iyi yöntemleri öğreten etkileşimli bir yapıya sahiptir. |
| **Legal+ Personal AI Lawyer (Rage Ahmed)**               | Hukuki danışmanlık, sözleşme taslağı | Gerçek zamanlı hukuk danışmanlığı ve özelleştirilmiş sözleşme şablonları üretir. Hukuk terminolojisine hakimdir.                               |
| **Legal Assistant GPT (Juan Velasco)**                   | Hukuki danışmanlık, sözleşme taslağı | Sözleşme ve yasal belge taslakları hazırlar, taraflara danışmanlık sağlar. Çok yönlü hukuki sorulara yanıt verir.                              |
| **Agent Agreement Legal Expert (LegalNowAI)**            | Acentelik sözleşmeleri               | Acentelik anlaşmaları hazırlama ve inceleme konusunda uzman destek sunar. Spesifik bir sözleşme türüne odaklıdır.                              |

Bu listede yer alan modeller, prompt tasarımı konusunda uzmanlık sağlarlar. Örneğin *Prompt Creator GPT* yeni isteminizi sürekli olarak geliştirip güncellemeyi taahhüt eder. *Prompt Wizard GPT* ise verdiğiniz ihtiyaca yönelik bir promptu “büyülü sonuçlar” elde edecek şekilde yeniden yazar. Tabii ki her modelin güçlü ve zayıf yönleri vardır; bu tablo son bölümde ele alınmıştır.

## Kontrat Bileşenleri İçin Model Yapılandırma Önerileri

Kontrat örneklerinde **ödeme hesaplamaları, taraf yükümlülükleri, ceza şartları** gibi spesifik görevler için model yapılandırılırken, **roller (persona) ve bağlam** açıkça belirlenmelidir. İyi bir meta-prompt şablonu genellikle aşağıdaki öğeleri içerir:

* **Amaç (Objective)**: Modelin ne yapacağını açıklar. Örneğin “Bir proje sözleşmesi için ödeme planı oluştur” şeklinde net bir görev tanımı yapılır. *Learn Prompting* rehberine göre etkili bir prompt, açık bir talimat (instruction) içermelidir.
* **Girdiler (Inputs)**: Prompt’a dahil edilmesi gereken tüm parametreler, veriler veya kullanıcı girişi. Örneğin, “sözleşme bedeli, taksit sayısı, faiz oranı” gibi spesifik değerler listelenir. Bu bölüm, “Input Data” olarak tanımlanır.
* **Çıktılar (Outputs)**: Modelin üretmesi gereken sonucun biçimi ve içeriği. Örneğin “aylık ödeme tutarları listesi” veya “tarafların yükümlülükleri maddeler hâlinde” gibi detaylar verilir. Prompt elementleri arasında “Output Indicator” olarak anılan bu kısım, beklenen formatı netleştirir.
* **Test/Kabul Kriterleri (Acceptance Criteria)**: Üretilen cevabın başarılı sayılabilmesi için sağlanması gereken ölçütler. Örneğin, “hesaplanan taksitlerin toplamı sözleşme bedeline eşit olmalı” veya “her yükümlülük açık ve kapsamlı olmalı” gibi. Freeplay Blog’a göre, etkili prompt testleri; **girdi-çıktı örnekleri** ve **değerlendirme kriterleri** içeren bir test paketi (test suite) olarak ele alınır.
* **Bağlam ve Kısıtlamalar (Context/Constraints)**: Gerekli ek bilgiler veya model sınırları. Örneğin, “modeli finans uzmanı gibi kurgula” ifadesiyle *rol/persona* verilebilir. Latitude blogundaki örnekte, “Act as a financial analyst” tarzında bir rol eklemek modelin yanıtını o uzmanlık doğrultusunda şekillendirir. Benzer şekilde “hukuk uzmanı gibi davran” talimatı, sözleşme diline odaklanmayı sağlar.

Bu yapılandırma, her bir kontrat bileşeni için özelleştirilebilir. Örneğin bir **ödeme hesaplama** promptu şu şekilde düzenlenebilir:

* Amaç: “Bir proje sözleşmesi için aylık ödeme planı oluşturun.”
* Girdiler: “Sözleşme bedeli, peşinat tutarı, taksit sayısı, faiz oranı”
* Çıktılar: “Ay bazında ödeme miktarları ve vade takvimi”
* Kabul Kriterleri: “Taksitlerin toplamı sözleşme bedeline eşit olmalı; peşinat ilk ay düşüldükten sonra geri kalan eşit taksitler olmalı.”
* Bağlam: “Bu sözleşme sabit faizlidir.”
* Kısıtlamalar: “Ödeme tarihi her ayın 1’i, yazılı listede gösterilmeli.”

Bu örnek, **Amaç-Girdiler-Çıktılar-Kriterler** yapılandırmasının nasıl uygulanabileceğine işaret eder. LearnPrompting rehberine göre promptun direktifi (amaç), gerekli giriş verileri ve istenen çıktı formatı açıkça belirtildiğinde model daha tutarlı sonuç verir. Ayrıca, çıkışta özel bir format talep ederek (örneğin madde işaretli liste veya tablo) netlik sağlanabilir.

## Test Edilebilirlik ve Geri Bildirim Döngüsü

Oluşturulan meta-promptların **test edilebilir olması**, güvenilir sonuçlar için kritiktir. Prompt mühendisliğinde *test-driven* yaklaşım yaygınlaşmaktadır: her prompt sürümü için belirlenen test senaryolarıyla performans değerlendirilir. Adımlar şu şekildedir:

1. **Test Paketi Oluşturma:** Prompt şablonu için çeşitli girdi-çıktı örnekleri tanımlanır. Freeplay blogu özellikle vurgular ki, LLM özellikli bir iş için testler giriş/çıkış çiftlerinden oluşur ve her sürümde yeniden çalıştırılır. Örneğin farklı sözleşme tutarı, vade gibi **neden-sonuç senaryoları** ile test seti hazırlanır.
2. **Versiyon Kontrolü:** Her yeni prompt sürümü (farklı kelimelemeler, ek konteks vb.) takip edilir. Bu, “Prompt Template version” olarak adlandırılır ve kullanılan model, parametre ayarları (örneğin temperature) gibi bilgiler not edilir.
3. **Değerlendirme Kriterleri:** Her çıktı, önceden belirlenen kabul ölçütlerine göre incelenir. Freeplay’a göre, testi otomatikleştirmek için objektif kriterler (doğru/yanlış, format uyumu vb.) belirlenebilir. Örneğin bir ödeme tablosunda toplamların doğruluğu, yükümlülük metni oluşturmada kapsam denetimi gibi. İnsan değerlendirici veya ikinci bir LLM de bu aşamada kullanılabilir.
4. **Geri Bildirim ve İyileştirme:** Elde edilen sonuçlar incelenerek prompt revize edilir. Latitude blogu, “girdileri gözden geçir, hataları tespit et, promptu kısıtlamalar ve örneklerle iyileştir” adımlarını önerir. Gerçek sonuçlar beklentileri karşılamıyorsa prompt içindeki belirsizlikler giderilir, ek bağlam eklenir veya rol yönlendirmesi netleştirilir. Örneğin, modelin cevapları tutarsızsa sistem mesajıyla “sen bir sözleşme uzmanısın” gibi bir yönlendirme eklemek yararlı olabilir.

Bu süreç **yeniden test ve optimizasyon** ile devam eder. Beauchemin’in çalışması da belirtir ki, prompt mühendisliğinde test odaklı geliştirme (TDD), geleneksel yazılımdan daha da önemlidir çünkü LLM’ler öngörülemezdir. “Tahmin edilemez doğa”, tutarlılığı sağlamak için sıkı testler gerektirir. Dolayısıyla, test sonuçlarına göre promptları sürekli iyileştirmek (örneğin eksik senaryoları eklemek, açıklamaları güncellemek) kalıcı bir döngüdür.

## Örnek Şablon Yapısı

Aşağıda önerilen **meta-prompt şablonu** genel bir formattadır. Bu yapı, her sözleşme bileşeni için kolayca uyarlanabilir:

```
Amaç:  <Modelin gerçekleştirmesini istediğiniz görev veya problem tanımı>
Girdiler:  <Gerekli tüm parametreler ve bilgiler (ör. sözleşme tutarı, taraflar)>
Çıktılar:  <Modelden beklenen sonuç formatı ve içeriği (ör. maddeler, hesaplama sonuçları)>
Test/Kabul Kriterleri:  <Çıktının başarılı sayılması için ölçütler (doğruluk, format uyumu vb.)>
Bağlam:  <Varsa ek bilgiler, örnekler veya belirli yönlendirmeler (ör. “bu bir ticari sözleşmedir”)>
Kısıtlamalar:  <Model sınırları veya uyulması gereken kurallar (ör. maksimum token, yapı vb.)>
```

Örneğin bir **ceza şartı** promptu şu şekilde olabilir:

```
Amaç:  Belirtilen sözleşme ihlallerine göre uygulanacak ceza şartlarını açıkla.
Girdiler:  Sözleşme ihlal türü (gecikme, kusurlu teslim, vb.), taraf bilgileri, ceza oranı (%) 
Çıktılar:  Her ihlal türü için maddeler hâlinde ceza bedelleri ve uygulanma koşulları
Test/Kabul Kriterleri:  Tüm ihlal türleri listelenmeli; her ceza oranı doğru hesaplanmış ve açıkça belirtilmiş olmalı.
Bağlam:  Bu şartlar bir hizmet sözleşmesi içindir; cezalar toplam ücretin %30’unu geçmemelidir.
Kısıtlamalar:  Yanıt, Türkçe ve resmi dilde olmalıdır. Kod bloğu veya tablo kullanılabilir.
```

Bu örneklerde görüldüğü gibi her bölüm net biçimde tanımlanmıştır. Prompting kılavuzları da belirtiyor ki, böyle **talimat + giriş + format** yapısı, modelin beklentiyi daha iyi anlamasını sağlar. Şablon, ihtiyaç duyulursa örnek kullanım senaryoları veya faydalı linklerle zenginleştirilebilir.

## Önerilen Yaklaşımların Güçlü ve Zayıf Yönleri

* **ChatGPT/GPT-4 (Temel Dil Modeli)** – *Güçlü Yanları:* Çok büyük bir bilgi ve dil anlayışına sahiptir; çeşitli formda çıktı üretebilir. Şablon formatlama, rol yönlendirme gibi teknikleri iyi uygulayabilir. *Sınırlamaları:* Kesin doğruluk gerektiren hesaplamalarda hata yapabilir; zaman zaman gerçek dışı (“halisünasyon”) bilgi üretebilir. Sözleşme gibi kritik dokümanlar için mutlaka insan denetimi gereklidir.
* **Prompt Mühendisliği GPT’leri (Örneğin Prompt Creator, Prompt Wizard, Prompt Perfect vb.)** – *Güçlü Yanları:* En iyi prompt yazım uygulamalarını kullanarak hızlı çözümler sunarlar. Özellikle teknik veya şablon odaklı düzenlemelerde tutarlı yapı üretirler. *Sınırlamaları:* Genel amaçlıdırlar; domain bilgisini (ör. hukuk) kendi başlarına eklemezler, bu yüzden sözleşmeye özel ayrıntılar için kullanıcıdan ek yönerge beklerler. Ayrıca eğitildikleri tarih kesitiyle sınırlıdır (en güncel mevzuat bilgisi olmayabilir).
* **Hukuk / Sözleşme GPT’leri (Legal Assistant, AI Lawyer, vb.)** – *Güçlü Yanları:* Hukuki terminoloji ve kapsamlı sözleşme örneklerine hakimdir. Taraf yükümlülükleri, ceza maddeleri gibi konuları doğru bir dille formüle edebilir. *Sınırlamaları:* Bunlar son kullanıcı ile etkileşimde sözleşme metni üretmeye odaklıdır; dolayısıyla meta-prompt oluşturma konusunda sınırlı rehberlik sağlarlar. Ayrıca, önerileri yasal tavsiye yerine geçmez; ülkeye ve döneme göre yasal geçerlilik kontrolü yapılmalıdır.
* **Genel** – *Avantajlar:* Yukarıdaki araçlar, iyi yapılandırılmış promptlar sayesinde model çıktılarının tutarlılığını artırır ve tekrar kullanılabilir hale getirir. *Riskler:* Yapay zeka öngörülemez olduğu için, modellerden çıkan sonuçlar mutlaka test ve incelemeden geçirilmelidir. Özellikle finansal hesaplama veya yasal içerikli metinlerde hata payı minimize edilmeli. Test-driven geliştirme yaklaşımı benimsenerek her prompt döngüsünde kalite arttırılmalıdır.

**Kullanım Rehberi:** Bu GPT’leri kullanırken şu ipuçlarına dikkat edebilirsiniz:

* Amacınızı açıkça tanımlayın ve örneklerle pekiştirin. Gerekiyorsa az sayıda örnek vaka (“few-shot”) verin.
* Roller belirleyin: Örneğin “sen bir hukuk danışmanısın” diyerek modelin tonunu ve perspektifini yönlendirin.
* Çıktı formatını netleştirin: Madde listesi, tablo veya belirli başlıklar kullanmasını isteyin. Bu, test/validasyon sürecini kolaylaştırır.
* Girdi çeşitliliğini sağlayın: İstemleri farklı senaryolarla sınayın. Hatalı sonuçlar gördükçe promptu yeniden düzenleyin.
* Otomasyon araçları kullanın: Promptimize veya benzeri kütüphaneler testleri otomatikleştirebilir. Version kontrol sistemleri ile her değişikliği kaydedin.

Bu adımlar ve araçlarla, **kontrat bileşenleri için özel meta-promptlar** tasarlayıp iteratif olarak geliştirebilir, eksik noktaları tespit ederek optimize edebilirsiniz. Yapısal şablonlar ve test döngüleri sayesinde hem efektif hem de güvenilir GPT yanıtları elde etmek mümkün olacaktır.
