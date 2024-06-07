import { timestampToDate } from "../utils/helpers";

export class FeedingLogVM {
    id: number;
    date: string;
    amount: number;
    shepherdName: string | null;
    shepherdSurname: string | null;
    sheepId: number;

    constructor(id: number, timestamp: number, amount: number, sheepId: number, shepherdName: string | null, shepherdSurname: string | null) {
        this.id = id;
        this.amount = amount / 1000;
        this.shepherdName = shepherdName;
        this.shepherdSurname = shepherdSurname;
        this.sheepId = sheepId;
        this.date = timestampToDate(timestamp);
    }
}
