export class Feed {
    id: number | null;
    amount: number;
    name: string;
    calories: number;
    fat: number;
    protein: number;
    carbohydrates: number;

    constructor(id: number | null, amount: number, name: string, calories: number, fat: number, protein: number, carbohydrates: number) {
        this.id = id;
        this.amount = amount;
        this.name = name;
        this.calories = calories;
        this.fat = fat;
        this.protein = protein;
        this.carbohydrates = carbohydrates;
    }
}