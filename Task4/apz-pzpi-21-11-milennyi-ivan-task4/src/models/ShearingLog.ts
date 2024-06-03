export class ShearingLog {
    id: number | null;
    sheepId: number;
    shepherdId: number | null;
    timestamp: number;
    woolAmount: number;

    constructor(id: number | null, sheepId: number, shepherdId: number | null, timestamp: number, woolAmount: number) {
        this.id = id;
        this.sheepId = sheepId;
        this.shepherdId = shepherdId;
        this.timestamp = timestamp;
        this.woolAmount = woolAmount;
    }
}