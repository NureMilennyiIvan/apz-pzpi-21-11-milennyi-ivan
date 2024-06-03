import { timestampToDate } from "../utils/helpers";

export class FeedingLogVM {
    id: number;
    date: string;
    amount: number;
    shepherdName?: string;
    shepherdSurname?: string;
    sheepId: number;

    constructor(id: number, timestamp: number, amount: number, sheepId: number, shepherdName?: string, shepherdSurname?: string) {
        this.id = id;
        this.amount = amount / 1000;
        this.shepherdName = shepherdName;
        this.shepherdSurname = shepherdSurname;
        this.sheepId = sheepId;
        this.date = timestampToDate(timestamp);
    }
}
