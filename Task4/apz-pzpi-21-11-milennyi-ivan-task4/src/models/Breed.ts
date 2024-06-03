export class Breed {
    id: number | null;
    name: string;
    feedId: number;
    info: string;

    constructor(id: number | null, name: string, feedId: number, info: string) {
        this.id = id;
        this.name = name;
        this.feedId = feedId;
        this.info = info;
    }
}