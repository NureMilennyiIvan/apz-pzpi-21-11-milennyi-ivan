export class FeedSupply {
    id: number | null;
    storekeeperId: number | null;
    amount: number;
    timestamp: number;
    feedId: number;

    constructor(id: number | null, storekeeperId: number | null, amount: number, timestamp: number, feedId: number) {
        this.id = id;
        this.storekeeperId = storekeeperId;
        this.amount = amount;
        this.timestamp = timestamp;
        this.feedId = feedId;
    }
}