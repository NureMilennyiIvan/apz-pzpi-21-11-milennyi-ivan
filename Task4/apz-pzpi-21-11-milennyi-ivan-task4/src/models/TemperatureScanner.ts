export class TemperatureScanner {
    id: number | null;
    temperature: number;
    password: string;

    constructor(id: number | null, temperature: number, password: string) {
        this.id = id;
        this.temperature = temperature / 10;
        this.password = password;
    }
}