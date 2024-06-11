export class ShearingLog {
    id: number | null;
    sheep_id: number;
    shepherd_id: number | null;
    timestamp: number;
    wool_amount: number;

    constructor(id: number | null, sheepId: number, shepherdId: number | null, timestamp: number, woolAmount: number) {
        this.id = id;
        this.sheep_id = sheepId;
        this.shepherd_id = shepherdId;
        this.timestamp = Math.floor(timestamp / 1000);
        this.wool_amount = woolAmount;
    }
}