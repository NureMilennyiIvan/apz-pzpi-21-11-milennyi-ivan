import { FeedingLog } from "../../models/FeedingLog";
import { FeedingLogVM } from "../../viewModels/FeedingLogVM";
import { IFeedingLogService } from "../interfaces/IFeedingLogService";

export class FeedingLogService implements IFeedingLogService {
    getAllVMsBySheepId(id: number): Promise<FeedingLogVM[]> {
        throw new Error("Method not implemented.");
    }
    getAllVMsByFeedId(id: number): Promise<FeedingLogVM[]> {
        throw new Error("Method not implemented.");
    }
    create(item: FeedingLog): Promise<FeedingLog> {
        throw new Error("Method not implemented.");
    }
    delete(itemId: number): Promise<void> {
        throw new Error("Method not implemented.");
    }
    update(item: FeedingLog): Promise<FeedingLog> {
        throw new Error("Method not implemented.");
    }
    getAll(): Promise<FeedingLog[]> {
        throw new Error("Method not implemented.");
    }
    getById(id: number): Promise<FeedingLog | null> {
        throw new Error("Method not implemented.");
    }
}