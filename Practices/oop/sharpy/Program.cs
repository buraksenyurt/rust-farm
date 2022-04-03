using System.Collections.Generic;

Robot tars = new Robot("TARS", 80);
Submarine u12 = new Submarine("u12", 1200);
Submarine alpha = new Submarine("Alpha", 5000);

// CallTools(tars);
// CallTools(u12);
// CallTools(alpha);

// void CallTools(IAbility ability){
//     ability.SetTools();
// }

List<IAbility> abilities=new List<IAbility>{tars,u12,alpha};
foreach (var a in abilities)
{
    a.SetTools();
}

enum State
{
    Online,
    OutOfService,
    OnTheMove,
    Dive,
    Destroyed
}

interface IAbility
{
    void SetTools();
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
    : Vehicle, IAbility
{
    public Robot(string name, float fuel)
        : base(name, fuel)
    {
    }

    public void SetTools()
    {
        Console.WriteLine($"{base.Name} için termal görüş sistemi, oksijen seviyesi ölçer yükleniyor.");
    }

    public void Walk(float x, float y)
    {
        Console.WriteLine($"{x},{y} noktasında hareket halinde");
        this.State = State.OnTheMove;
    }
}

class Submarine
    : Vehicle, IAbility
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

    public void SetTools()
    {
        Console.WriteLine($"{base.Name} için sonar, derinlik ölçer, ek batarya yükleniyor.");
    }
}