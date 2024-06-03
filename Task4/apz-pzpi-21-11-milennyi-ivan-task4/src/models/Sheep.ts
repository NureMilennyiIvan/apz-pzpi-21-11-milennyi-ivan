export class Sheep {
    id: number | null;
    birthDate: number;
    breedId: number;
    weight: number;
    sex: boolean;
    temperatureScannerId: number | null;
    shepherdId: number | null;

    constructor(id: number | null, birthDate: number, breedId: number, weight: number, sex: boolean, temperatureScannerId: number | null, shepherdId: number | null) {
        this.id = id;
        this.birthDate = birthDate;
        this.breedId = breedId;
        this.weight = weight;
        this.sex = sex;
        this.temperatureScannerId = temperatureScannerId;
        this.shepherdId = shepherdId;
    }
}