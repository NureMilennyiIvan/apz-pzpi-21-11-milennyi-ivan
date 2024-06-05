export class Sheep {
    id: number | null;
    birth_date: number;
    breed_id: number;
    weight: number;
    sex: boolean;
    temperature_scanner_id: number | null;
    shepherd_id: number | null;

    constructor(id: number | null, birthDate: number, breedId: number, weight: number, sex: boolean, temperatureScannerId: number | null, shepherdId: number | null) {
        this.id = id;
        this.birth_date = birthDate;
        this.breed_id = breedId;
        this.weight = weight;
        this.sex = sex;
        this.temperature_scanner_id = temperatureScannerId;
        this.shepherd_id = shepherdId;
    }
}