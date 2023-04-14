const vikingNames = [
    "Urist", "Nerthus", "Dane", "Helga", "Holger", "Hulgekir", "Folke", "Halvard",
];

export const getVikingName = () => {
    return vikingNames[Math.floor(Math.random() * vikingNames.length)];
}

export const getTwoVikingNames = () => {
    let name1 = getVikingName();
    let name2 = getVikingName();

    while (name1 == name2) {
        name2 = getVikingName();
    }

    return [name1,name2];
}
