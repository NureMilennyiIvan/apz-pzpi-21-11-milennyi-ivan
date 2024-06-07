import { timestampToDate } from "../utils/helpers";

export class ShearingLogVM {
    id: number;
    date: string;
    woolAmount: number;
    shepherdName: string | null;
    shepherdSurname: string | null;
    sheepId: number;

    constructor(id: number, timestamp: number, woolAmount: number, sheepId: number, shepherdName: string | null, shepherdSurname: string | null) {
        this.id = id;
        this.woolAmount = woolAmount / 1000;
        this.shepherdName = shepherdName;
        this.shepherdSurname = shepherdSurname;
        this.sheepId = sheepId;
        this.date = timestampToDate(timestamp);
    }
}