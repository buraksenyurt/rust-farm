#include <iostream>
#include <string>

class Player {
public:
    Player(const std::string& name,int level) : name(name) {}

    std::string GetName() {
        return name;
    }

    int GetLevel(){
        return level;
    }

private:
    std::string name;
    int level;
};

void calc_bonus(Player* player){
    std::cout << "Bonus hesaplamaları" << std::endl;
    // Başka bir fonksiyonda gelen player ile bir şeyler yapılıyor
    // ve bellekten atılıyor
    delete player;
}

int main() {
    Player* player = new Player("Doktor Sitrenç",400);
    Player* danglingPlayer = player; // player başka bir işaretçiye atanıyor
    std::cout << "Oyuncu " << player->GetName() << std::endl;
    calc_bonus(player);
    // danglingPointer, bellekten atılmış pointer'ın işaret ettiği veriyi
    // referans etmeye devam ediyor.
    std::cout <<"Danling pointer için oyuncu " << danglingPlayer->GetName() << std::endl;

    return 0;
}