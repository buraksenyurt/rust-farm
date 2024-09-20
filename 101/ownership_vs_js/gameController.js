const health = {
    value:100
};

const archer = {
    id : 1,
    name : "legolas",
    current_health : health
}

const orc = {
    id:2,
    name : "rando",
    current_health: health
}

function damage_to_orc(player,value){
    if(player.name === 'rando') {
        player.current_health.value -= value;
    }
}

console.log("Orc health before:", orc.current_health.value);
damage_to_orc(orc, 10);
console.log("Orc health after:", orc.current_health.value);
// Esasında damage_to_orc fonksiyonunda sadece 'rando' için bir damage söz konusu
// Ne var ki
console.log("Legolas health afrer: Upss",archer.current_health.value);