var tars = new Robot("TARS", 80);
Console.WriteLine(tars.ToString());
tars.LoadFuel(10);
Console.WriteLine(tars.ToString());
tars.Walk();
Console.WriteLine(tars.ToString());

// Robotun durumunu ifade edecek bir enum sabiti tanımladık.
public enum State
{
    Online,
    OutOfService,
    OnTheMove,
    Destroyed
}
class Robot
{
    public string Name { get; set; }
    public float FuelLevel { get; set; }
    // State özelliğine dışarıdan erişilemez.
    private State State { get; set; }

    // Parametrelerle güçlendirilmiş yapıcı metot(constructor)
    public Robot(string name, float fuel)
    {
        Name = name;
        FuelLevel = fuel;
        State = State.Online;
    }

    // Geriye bir şey döndürmeyen ve sınıfa ait nesne örneğinin FuelLevel özelliğini değiştiren kobay fonkisyon
    public void LoadFuel(float amount)
    {
        Console.WriteLine($"{amount} litre yakıt yükleniyor...");
        this.FuelLevel += amount;
    }

    // Yine kobay bir fonksiyon. Nesnenin state değerini değiştiriyor ve yürüyüş moduna geçiriyor
    public void Walk()
    {
        Console.WriteLine("Hareket halinde");
        this.State = State.OnTheMove;
    }

    // DotNet tarafında herkes bir Object olduğundan ve ToString fonksiyonu Object sınıfında
    // virutal tanımlandığından, istersek kendi türlerimiz için bu davranışı değiştirebiliriz.
    // Kısaca ToString metodunu Override ediyor ve varsayılan haliyle değil bizim istediğimiz
    // şekilde çalışması için yeniden kodluyoruz. Kısa olmadı yahu :D
    public override string ToString()
    {
        return $"{this.Name}. Yakıt {this.FuelLevel}. Durum {this.State}";
    }
}