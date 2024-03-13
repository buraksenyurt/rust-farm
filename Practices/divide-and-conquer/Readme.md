# Divide And Conquer Örnekleri

Divide and Conquer problem çözme algoritmalarından birisidir. Ana problemi çözmek için parçalara ayrılmış alt problemlerin çözümlerini kullanılır. Dağıtık sistemlerdeki bazı problemlerde kullanılabilir. Apache Spark, büyük veri setlerini Resilient Distributed Dataset isimli daha küçük veri setlerine alıp paralel işlerken, Azure'daki HDInsight ve Databricks gibi ürünler veri analizi ve makine öğrenimi görevlerinde, Amazon, Elastic MapReduce hizmeti ile EC2 üstünde veri işleme görevlerini parçalayıp paralel çalıştırırken bu algoritma fikrinden yararlanır.

Aslında Map Reduce, büyük veri setlerini işlemek için kullanılan bir modeldir ve Divide and Conquer algoritmasını baz almaktadır. Map aşamasında veri seti key-value çiftleri olarak ayrılır ve Reduce safhasında bu eşleştirmeler key bilgisine göre gruplanıp işlenmiş sonuç çıktıları üretilir. 

Örnekteki fonksiyonlarda basitten biraz daha işe yarar senaryolara doğru ilerlenmektedir ve Divide and Conquer algoritmasının uygulanması ele alınmaktadır.