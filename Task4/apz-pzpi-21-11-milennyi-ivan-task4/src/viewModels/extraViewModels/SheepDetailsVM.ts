import { timeInDays, timestampToDate } from "../../utils/helpers";

export class SheepDetailsVM {
    id: number;
    breed: string;
    breedInfo: string;
    sex: boolean;
    age: number;
    lastFeedingDate: string | null;
    lastShearingDate: string | null;
    weight: number;
    temperature: number | null;
    feedId: number;
    feedName: string;
    feedAmount: number;
    isFeed: boolean;
    isShear: boolean;

    constructor(
        id: number,
        breed: string,
        breedInfo: string,
        sex: boolean,
        birthDate: number,
        weight: number,
        feedId: number,
        feedName: string,
        feedAmount: number,
        lastFeedingTimestamp: number | null,
        lastShearingTimestamp: number | null,
        temperature: number | null
    ) {
        this.id = id;
        this.breed = breed;
        this.breedInfo = breedInfo;
        this.sex = sex;
        this.age = timeInDays(birthDate);
        this.weight = weight / 1000;
        this.feedId = feedId;
        this.feedName = feedName;
        this.feedAmount = feedAmount / 1000;
        this.lastFeedingDate = (lastFeedingTimestamp != null) ? timestampToDate(lastFeedingTimestamp) : null;
        this.lastShearingDate = (lastShearingTimestamp != null) ? timestampToDate(lastShearingTimestamp) : null;
        this.temperature = (temperature != null) ? temperature / 10: null;
        this.isFeed = (timeInDays(lastFeedingTimestamp) > 0) ? true : false;
        this.isShear = (timeInDays(lastShearingTimestamp) > 0) ? true : false;
    }
}