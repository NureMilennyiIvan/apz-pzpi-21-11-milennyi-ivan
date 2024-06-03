export class FeedingLog {
    id: number | null;
    sheepId: number;
    shepherdId: number | null;
    timestamp: number;
    feedId: number;
    amount: number;

    constructor(id: number | null, sheepId: number, shepherdId: number | null, timestamp: number, feedId: number, amount: number) {
        this.id = id;
        this.sheepId = sheepId;
        this.shepherdId = shepherdId;
        this.timestamp = timestamp;
        this.feedId = feedId;
        this.amount = amount;
    }
}