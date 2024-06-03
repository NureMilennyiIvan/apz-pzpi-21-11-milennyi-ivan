import { timestampToDate } from "../utils/helpers";

export class FeedSupplyVM {
    id: number;
    amount: number;
    date: string;
    storekeeperName?: string;
    storekeeperSurname?: string;

    constructor(id: number, amount: number, timestamp: number, storekeeperName?: string, storekeeperSurname?: string) {
        this.id = id;
        this.amount = amount / 1000;
        this.storekeeperName = storekeeperName;
        this.storekeeperSurname = storekeeperSurname;
        this.date = timestampToDate(timestamp);
    }
}
