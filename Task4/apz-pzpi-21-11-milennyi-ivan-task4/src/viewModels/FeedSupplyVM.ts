import { timestampToDate } from "../utils/helpers";

export class FeedSupplyVM {
    id: number;
    amount: number;
    date: string;
    storekeeperName: string | null;
    storekeeperSurname: string | null;

    constructor(id: number, amount: number, timestamp: number, storekeeperName: string | null, storekeeperSurname: string | null) {
        this.id = id;
        this.amount = amount / 1000;
        this.storekeeperName = storekeeperName;
        this.storekeeperSurname = storekeeperSurname;
        this.date = timestampToDate(timestamp);
    }
}
