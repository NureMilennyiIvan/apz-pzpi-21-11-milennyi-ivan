import { Feed } from "../../models/Feed";
import { FeedVM } from "../../viewModels/FeedVM";
import { IService } from "./IService";

export interface IFeedService extends IService<Feed, FeedVM> {
    getAllVMs(): Promise<FeedVM[]>;
}