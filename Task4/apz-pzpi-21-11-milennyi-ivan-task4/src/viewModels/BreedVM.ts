export class BreedVM {
    id: number;
    name: string;
    info: string;
    feedName: string;
    sheepCount: number;

    constructor(id: number, name: string, info: string, feedName: string, sheepCount: number) {
        this.id = id;
        this.name = name;
        this.info = info;
        this.feedName = feedName;
        this.sheepCount = sheepCount;
    }
}