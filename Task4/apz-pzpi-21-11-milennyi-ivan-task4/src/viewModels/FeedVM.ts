export class FeedVM {
    id: number;
    amount: number;
    name: string;
    calories: number;
    fat: number;
    protein: number;
    carbohydrates: number;
    breedName: string;
    sheepCount: number;

    constructor(id: number, amount: number, name: string, calories: number, fat: number, protein: number, carbohydrates: number, breedName: string, sheepCount: number) {
        this.id = id;
        this.amount = amount / 1000;
        this.name = name;
        //Format later
        this.calories = calories;
        this.fat = fat;
        this.protein = protein;
        this.carbohydrates = carbohydrates;
        this.breedName = breedName;
        this.sheepCount = sheepCount;
    }
}