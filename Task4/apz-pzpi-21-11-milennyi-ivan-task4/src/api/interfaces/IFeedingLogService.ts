import { FeedingLog } from "../../models/FeedingLog";
import { FeedingLogVM } from "../../viewModels/FeedingLogVM";
import { IService } from "./IService";

export interface IFeedingLogService extends IService<FeedingLog, FeedingLogVM> {
    getAllVMsBySheepId(id: number): Promise<FeedingLogVM[]>;
    getAllVMsByFeedId(id: number): Promise<FeedingLogVM[]>;
}