Robot tars = new Robot("TARS", 80);
Console.WriteLine(tars.ToString());
tars.LoadFuel(10);
Console.WriteLine(tars.ToString());
tars.Walk(23, -51);
Console.WriteLine(tars.ToString());

Submarine u12 = new Submarine("u12", 1200);
Console.WriteLine(u12.ToString());
u12.LoadFuel(10);
Console.WriteLine(u12.ToString());
u12.Dive(800);
Console.WriteLine(u12.ToString());

enum State
{
    Online,
    OutOfService,
    OnTheMove,
    Dive,
    Destroyed
}
abstract class Vehicle
{
    public string Name { get; set; }
    public float FuelLevel { get; set; }
    protected State State { get; set; }

    public Vehicle(string name, float fuelLevel)
    {
        Name = name;
        FuelLevel = fuelLevel;
        State = State.Online;
    }
    public override string ToString()
    {
        return $"{this.Name}. Yakıt {this.FuelLevel}. Durum {this.State}";
    }
    public void LoadFuel(float amount)
    {
        Console.WriteLine($"{amount} litre yakıt yükleniyor...");
        this.FuelLevel += amount;
    }
}

class Robot
    : Vehicle
{
    public Robot(string name, float fuel)
        : base(name, fuel)
    {
    }

    public void Walk(float x, float y)
    {
        Console.WriteLine($"{x},{y} noktasında hareket halinde");
        this.State = State.OnTheMove;
    }
}

class Submarine
    : Vehicle
{
    public Submarine(string name, float fuel)
        : base(name, fuel)
    {
    }
    public void Dive(int depth)
    {
        Console.WriteLine($"{depth} metreye dalıyor");
        this.State = State.Dive;
    }
}