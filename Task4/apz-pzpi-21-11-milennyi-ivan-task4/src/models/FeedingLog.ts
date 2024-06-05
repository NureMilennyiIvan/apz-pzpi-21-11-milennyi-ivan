export class FeedingLog {
    id: number | null;
    sheep_id: number;
    shepherd_id: number | null;
    timestamp: number;
    feed_id: number;
    amount: number;

    constructor(id: number | null, sheepId: number, shepherdId: number | null, timestamp: number, feedId: number, amount: number) {
        this.id = id;
        this.sheep_id = sheepId;
        this.shepherd_id = shepherdId;
        this.timestamp = timestamp;
        this.feed_id = feedId;
        this.amount = amount * 1000;
    }
}