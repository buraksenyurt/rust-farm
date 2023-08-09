#include <iostream>
#include <string>

class Player {
public:
    void SetTitle(const std::string& newTitle) {
        title = newTitle;
    }

    void SetPoint(int newPoint) {
        point = newPoint;
    }

    std::string GetTitle() {
        return title;
    }

    int GetPoint() {
        return point;
    }

private:
    std::string title;
    int point;
};

int del(Player* player){
    delete player;
    return 0;
}
int main() {
    Player* player = new Player();
    player->SetTitle("Champion");
    player->SetPoint(1000);

    del(player);

    std::string playerTitle = player->GetTitle(); // Use After Free
    int playerPoint = player->GetPoint(); // Use After Free

    std::cout << "Player Title: " << playerTitle << std::endl;
    std::cout << "Player Point: " << playerPoint << std::endl;

    return 0;
}
