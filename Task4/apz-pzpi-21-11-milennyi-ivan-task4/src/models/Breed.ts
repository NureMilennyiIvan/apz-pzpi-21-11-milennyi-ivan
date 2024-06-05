export class Breed {
    id: number | null;
    name: string;
    feed_id: number;
    info: string;

    constructor(id: number | null, name: string, feedId: number, info: string) {
        this.id = id;
        this.name = name;
        this.feed_id = feedId;
        this.info = info;
    }
}