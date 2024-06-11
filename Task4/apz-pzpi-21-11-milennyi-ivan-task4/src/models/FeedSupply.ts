export class FeedSupply {
    id: number | null;
    storekeeper_id: number | null;
    amount: number;
    timestamp: number;
    feed_id: number;

    constructor(id: number | null, storekeeperId: number | null, amount: number, timestamp: number, feedId: number) {
        this.id = id;
        this.storekeeper_id = storekeeperId;
        this.amount = amount;
        this.timestamp = Math.floor(timestamp / 1000);
        this.feed_id = feedId;
    }
}