import { timestampToDate } from "../utils/helpers";

export class SheepVM {
    id: number;
    breed: string;
    sex: boolean;
    birthDate: number;
    lastFeedingDate: string | null;
    lastShearingDate: string | null;

    constructor(id: number, breed: string, sex: boolean, birthDate: number, lastFeedingTimestamp: number | null, lastShearingTimestamp: number | null) {
        this.id = id;
        this.breed = breed;
        this.sex = sex;
        this.birthDate = birthDate;
        this.lastFeedingDate = (lastFeedingTimestamp != null) ? timestampToDate(lastFeedingTimestamp) : null;
        this.lastShearingDate = (lastShearingTimestamp != null) ? timestampToDate(lastShearingTimestamp) : null;
    }
}