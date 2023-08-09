#include <iostream>
#include <string>

class Player {
public:
    Player(const std::string& title, int point) : title(title), point(point) {}

private:
    std::string title;
    int point;
};

void del(Player* player){
    delete player;
}
int main() {
    Player* player = new Player("Şarlotte", 91);
    del(player);

    // Belleği tekrar serbest bırakmaya çalışmak "Double Free" durumunun oluşmasına sebebiyet verir.
    // Kod derlenir ama çalışma zamanında Segmentation fault hatası alınır.
    del(player);

    return 0;
}
